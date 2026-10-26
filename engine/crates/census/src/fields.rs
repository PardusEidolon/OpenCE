//! The tag fields: each tag structure's fields, from the layout, marked
//! with whether the original reads them.
//!
//! The original reads a tag through a pointer it declares to the tag's
//! structure (`struct unit_definition *definition= ...`, a function's
//! parameter or a local), and then a field through that pointer
//! (`definition->unit.seats`). The census follows exactly that: within each
//! function, the pointers declared to a laid-out structure, and the
//! members declared as one (`camera_info.camera->`), and the paths read
//! through them, array indexes left out. A path reaching a field
//! that holds smaller ones (a point, a tag block) reads all of them; one
//! reaching an embedded tag structure (`unit.object`) does not, since the
//! original reads that through a pointer of the structure's own. What it
//! reads in an embedded structure, by either way, is read wherever that
//! structure is embedded: `object_definition->object.flags` reads the
//! flags of every unit and weapon too.
//!
//! `opence census --write` keeps the marks in `engine/census/tag_fields.tsv`;
//! `--check` marks them again and compares.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::layout::{Field, Record};
use crate::{read_text, walk, Error};

/// The written fields, from `engine/`.
pub const FIELDS: &str = "census/tag_fields.tsv";

/// How deep embedded structures are followed.
const DEPTH: usize = 8;

/// A field of a tag structure, marked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    /// The structure.
    pub structure: String,
    /// The field's path in it.
    pub path: String,
    /// Its byte offset.
    pub offset: u32,
    /// Its type, as clang prints it.
    pub kind: String,
    /// Whether the original reads it.
    pub original: bool,
}

/// The fields of `record` that hold no smaller ones, once each, in order.
pub fn leaves(record: &Record) -> Vec<&Field> {
    let mut seen = BTreeSet::new();
    record
        .fields
        .iter()
        .filter(|field| {
            let inside = format!("{}.", field.path);
            !record.fields.iter().any(|f| f.path.starts_with(&inside))
        })
        .filter(|field| seen.insert(field.path.as_str()))
        .collect()
}

/// A token of the C text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    /// An identifier or keyword.
    Name(&'a str),
    /// One character of punctuation.
    Punct(u8),
}

/// The text's names and punctuation, without comments, literals and
/// preprocessor lines.
fn tokens(text: &str) -> Vec<Token<'_>> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    let mut line_start = true;
    while let Some(&byte) = bytes.get(at) {
        let rest = bytes.get(at..).unwrap_or_default();
        if byte == b'\n' {
            line_start = true;
            at += 1;
            continue;
        }
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }
        if line_start && byte == b'#' {
            // The directive, through its continued lines.
            while let Some(&b) = bytes.get(at) {
                if b == b'\n' && bytes.get(at.wrapping_sub(1)) != Some(&b'\\') {
                    break;
                }
                at += 1;
            }
            continue;
        }
        line_start = false;
        if rest.starts_with(b"//") {
            while bytes.get(at).is_some_and(|&b| b != b'\n') {
                at += 1;
            }
        } else if rest.starts_with(b"/*") {
            at = find(bytes, at + 2, b"*/").map_or(bytes.len(), |end| end + 2);
        } else if byte == b'"' || byte == b'\'' {
            at += 1;
            while let Some(&b) = bytes.get(at) {
                at += if b == b'\\' { 2 } else { 1 };
                if b == byte {
                    break;
                }
            }
        } else if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = at;
            while bytes
                .get(at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
            {
                at += 1;
            }
            if let Some(name) = text.get(start..at) {
                found.push(Token::Name(name));
            }
        } else if byte.is_ascii_digit() {
            while bytes
                .get(at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'.')
            {
                at += 1;
            }
        } else {
            found.push(Token::Punct(byte));
            at += 1;
        }
    }
    found
}

/// Where `needle` next starts in `bytes` from `from`.
fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes
        .get(from..)?
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|at| at + from)
}

/// The token after the `[` at `open` that closes it.
fn closing(tokens: &[Token<'_>], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (at, token) in tokens.iter().enumerate().skip(open) {
        match token {
            Token::Punct(b'[') => depth += 1,
            Token::Punct(b']') => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// The pointer `struct S *name` declared at `at`, if `S` is one of
/// `structures`: the name and `S`.
fn declaration<'a>(
    tokens: &[Token<'a>],
    at: usize,
    structures: &BTreeSet<String>,
) -> Option<(&'a str, &'a str)> {
    let mut next = at + 1;
    let Some(Token::Name(structure)) = tokens.get(next) else {
        return None;
    };
    if !structures.contains(*structure) {
        return None;
    }
    next += 1;
    let qualifier =
        |token: Option<&Token<'_>>| matches!(token, Some(Token::Name("const" | "volatile")));
    while qualifier(tokens.get(next)) {
        next += 1;
    }
    if tokens.get(next) != Some(&Token::Punct(b'*')) {
        return None;
    }
    next += 1;
    while qualifier(tokens.get(next)) {
        next += 1;
    }
    match tokens.get(next) {
        Some(Token::Name(name)) => Some((name, structure)),
        _ => None,
    }
}

/// The path read through the pointer at `at` (`name->a[i].b`), if it
/// is one.
fn access(tokens: &[Token<'_>], at: usize) -> Option<String> {
    if tokens.get(at + 1) != Some(&Token::Punct(b'-'))
        || tokens.get(at + 2) != Some(&Token::Punct(b'>'))
    {
        return None;
    }
    let Some(Token::Name(first)) = tokens.get(at + 3) else {
        return None;
    };
    let mut path = vec![*first];
    let mut next = at + 4;
    loop {
        match tokens.get(next) {
            Some(Token::Punct(b'[')) => next = closing(tokens, next)? + 1,
            Some(Token::Punct(b'.')) => match tokens.get(next + 1) {
                Some(Token::Name(name)) => {
                    path.push(name);
                    next += 2;
                }
                _ => break,
            },
            _ => break,
        }
    }
    Some(path.join("."))
}

/// Every pointer `text` declares to one of `structures` (a parameter, a
/// local or a member of a structure of its own): the name and the
/// structures it is declared as.
pub fn declared(text: &str, structures: &BTreeSet<String>) -> BTreeMap<String, BTreeSet<String>> {
    let tokens = tokens(text);
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (at, token) in tokens.iter().enumerate() {
        if *token == Token::Name("struct") {
            if let Some((name, structure)) = declaration(&tokens, at, structures) {
                found
                    .entry(name.to_owned())
                    .or_default()
                    .insert(structure.to_owned());
            }
        }
    }
    found
}

/// The names of `declared` that are only ever declared as one structure,
/// with it: the members a read through `holder.member->` can trust.
pub fn members(declared: &BTreeMap<String, BTreeSet<String>>) -> BTreeMap<String, String> {
    declared
        .iter()
        .filter_map(
            |(name, structures)| match structures.iter().collect::<Vec<_>>()[..] {
                [only] => Some((name.clone(), only.clone())),
                _ => None,
            },
        )
        .collect()
}

/// The fields `text` reads through pointers to `structures`: each
/// structure with the path read, once each. A pointer is a parameter or a
/// local the function declares, or a member (`info.camera->`) of
/// `members`.
pub fn reads(
    text: &str,
    structures: &BTreeSet<String>,
    members: &BTreeMap<String, String>,
) -> BTreeSet<(String, String)> {
    let tokens = tokens(text);
    let mut found = BTreeSet::new();
    let mut pointers: BTreeMap<&str, &str> = BTreeMap::new();
    let mut depth = 0usize;
    for (at, token) in tokens.iter().enumerate() {
        match token {
            Token::Punct(b'{') => depth += 1,
            Token::Punct(b'}') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    pointers.clear();
                }
            }
            Token::Punct(b';') if depth == 0 => pointers.clear(),
            Token::Name("struct") => {
                if let Some((name, structure)) = declaration(&tokens, at, structures) {
                    pointers.insert(name, structure);
                }
            }
            Token::Name(name) => {
                let member = at.checked_sub(1).and_then(|i| tokens.get(i))
                    == Some(&Token::Punct(b'.'))
                    || at.checked_sub(1).and_then(|i| tokens.get(i)) == Some(&Token::Punct(b'>'))
                        && at.checked_sub(2).and_then(|i| tokens.get(i))
                            == Some(&Token::Punct(b'-'));
                let structure = if member {
                    members.get(*name).map(String::as_str)
                } else {
                    pointers.get(name).copied()
                };
                if let Some(structure) = structure {
                    if let Some(path) = access(&tokens, at) {
                        found.insert((structure.to_owned(), path));
                    }
                }
            }
            Token::Punct(_) => {}
        }
    }
    found
}

/// The structure a field of type `kind` embeds, if it is a laid-out one.
fn embedded<'a>(kind: &'a str, records: &BTreeMap<&str, &Record>) -> Option<&'a str> {
    let name = kind.strip_prefix("struct ")?;
    (!name.contains(['[', '*', ':']) && records.contains_key(name)).then_some(name)
}

/// Whether the original reads `path` of `record`.
fn is_read(
    record: &Record,
    path: &str,
    records: &BTreeMap<&str, &Record>,
    read: &BTreeMap<&str, Vec<&str>>,
    depth: usize,
) -> bool {
    let direct = read.get(record.name.as_str()).is_some_and(|paths| {
        paths.iter().any(|a| {
            if path == *a || a.starts_with(&format!("{path}.")) {
                return true;
            }
            path.starts_with(&format!("{a}."))
                && record
                    .fields
                    .iter()
                    .find(|f| f.path == *a)
                    .is_some_and(|f| embedded(&f.kind, records).is_none())
        })
    });
    direct
        || depth < DEPTH
            && record.fields.iter().any(|field| {
                let Some(inner) = embedded(&field.kind, records) else {
                    return false;
                };
                let Some(rest) = path.strip_prefix(&format!("{}.", field.path)) else {
                    return false;
                };
                records
                    .get(inner)
                    .is_some_and(|r| is_read(r, rest, records, read, depth + 1))
            })
}

/// The read of `path` in `structure` again in each structure it reaches
/// inside (`object.flags` of a `unit_definition` is `flags` of its
/// `_object_definition`), so that every structure embedding that one
/// has it read.
fn innermost(
    structure: &str,
    path: &str,
    records: &BTreeMap<&str, &Record>,
    found: &mut BTreeSet<(String, String)>,
    depth: usize,
) {
    let Some(record) = records.get(structure) else {
        return;
    };
    for field in &record.fields {
        let Some(inner) = embedded(&field.kind, records) else {
            continue;
        };
        if let Some(rest) = path.strip_prefix(&format!("{}.", field.path)) {
            if depth < DEPTH && found.insert((inner.to_owned(), rest.to_owned())) {
                innermost(inner, rest, records, found, depth + 1);
            }
        }
    }
}

/// Every leaf of `records`, in their order, marked with whether `reads`
/// reach it.
pub fn marks(records: &[Record], reads: &BTreeSet<(String, String)>) -> Vec<Mark> {
    let by_name: BTreeMap<&str, &Record> = records.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut inner = BTreeSet::new();
    for (structure, path) in reads {
        innermost(structure, path, &by_name, &mut inner, 0);
    }
    let mut read: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (structure, path) in reads.iter().chain(&inner) {
        read.entry(structure.as_str())
            .or_default()
            .push(path.as_str());
    }
    records
        .iter()
        .flat_map(|record| {
            leaves(record).into_iter().map(|field| Mark {
                structure: record.name.clone(),
                path: field.path.clone(),
                offset: field.offset,
                kind: field.kind.clone(),
                original: is_read(record, &field.path, &by_name, &read, 0),
            })
        })
        .collect()
}

/// The marks of `records` against `source/`'s C in the repository at
/// `root`.
pub fn generate(root: &Path, records: &[Record]) -> Result<Vec<Mark>, Error> {
    let structures: BTreeSet<String> = records.iter().map(|r| r.name.clone()).collect();
    let mut texts = Vec::new();
    let mut all: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for file in walk(&root.join("source"), &["c", "h"])? {
        let text = read_text(&file)?;
        for (name, types) in declared(&text, &structures) {
            all.entry(name).or_default().extend(types);
        }
        if file.extension().is_some_and(|e| e == "c") {
            texts.push(text);
        }
    }
    let members = members(&all);
    let mut found = BTreeSet::new();
    for text in &texts {
        found.extend(reads(text, &structures, &members));
    }
    Ok(marks(records, &found))
}

/// `tag_fields.tsv`'s text: a line for each field.
pub fn write_fields(marks: &[Mark]) -> String {
    let mut text =
        String::from("# structure\tfield\toffset\ttype\toriginal (opence census --write)\n");
    for mark in marks {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            mark.structure,
            mark.path,
            mark.offset,
            mark.kind,
            if mark.original { "read" } else { "-" }
        ));
    }
    text
}

/// `tag_fields.tsv` read back.
pub fn read_fields(text: &str) -> Result<Vec<Mark>, Error> {
    let mut marks = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{FIELDS}"),
            line: index + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [structure, path, offset, kind, original] = columns[..] else {
            return Err(malformed("not five columns"));
        };
        marks.push(Mark {
            structure: structure.to_owned(),
            path: path.to_owned(),
            offset: offset.parse().map_err(|_| malformed("not a number"))?,
            kind: kind.to_owned(),
            original: match original {
                "read" => true,
                "-" => false,
                _ => return Err(malformed("neither read nor -")),
            },
        });
    }
    Ok(marks)
}

/// The written fields, if they are not the marks of the written layout
/// (a tree with neither has none).
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let engine = root.join("engine");
    let layout = read_text(&engine.join(crate::layout::LAYOUT)).unwrap_or_default();
    let records = crate::layout::read_layout(&layout)?;
    let written = read_fields(&read_text(&engine.join(FIELDS)).unwrap_or_default())?;
    Ok(if written == generate(root, &records)? {
        Vec::new()
    } else {
        vec![format!(
            "engine/{FIELDS} is stale: run opence census --write"
        )]
    })
}
