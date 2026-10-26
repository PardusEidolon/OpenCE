//! The tag layout: every tag structure of `source/`'s `*_definitions.h`
//! with each field's offset, as the original's compiler laid it out.
//!
//! clang lays the headers out for 32-bit MSVC (`i686-pc-windows-msvc`),
//! the original's own target, and dumps each record's layout; the headers'
//! own `verify_*` asserts hold on that target, so the layout is the
//! original's. `opence census --write` keeps the result in
//! `engine/census/tag_layout.tsv`, so that the census's own check needs no
//! clang; `--check` lays the headers out again and compares. The crate's
//! test checks the written layout against every size and offset assert in
//! the headers. Only `source/`'s folders are on the include path, with a
//! few stand-in system headers.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{read_text, walk, Error};

/// The written layout, from `engine/`.
pub const LAYOUT: &str = "census/tag_layout.tsv";

/// The headers every definitions header leans on without including.
const PREAMBLE: &[&str] = &[
    "cseries.h",
    "real_math.h",
    "integer_math.h",
    "tag_groups.h",
    "game/game_globals.h",
];

/// Stand-ins for the system headers `cseries.h` includes. `StdDef.h`
/// defines what the headers use itself, since on a case-blind disk it is
/// also `stddef.h`.
const STAND_INS: &[(&str, &str)] = &[
    (
        "StdDef.h",
        "#pragma once\n#define offsetof(t, m) __builtin_offsetof(t, m)\n#define NULL ((void *)0)\ntypedef unsigned int size_t;\n",
    ),
    ("stdio.h", "#pragma once\n"),
    ("stdlib.h", "#pragma once\n"),
    ("string.h", "#pragma once\n"),
    ("stdarg.h", "#pragma once\n"),
    (
        "math.h",
        "#pragma once\ndouble sqrt(double); double sin(double); double cos(double); double tan(double);\n\
         double asin(double); double acos(double); double atan(double); double atan2(double, double);\n\
         double fabs(double); double floor(double); double ceil(double); double fmod(double, double);\n\
         double pow(double, double); double exp(double); double log(double);\n",
    ),
];

/// A record's field: its path from the record (`parts.count`; an unnamed
/// member adds nothing to it), its byte offset and its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The path, dotted through nested structures.
    pub path: String,
    /// The byte offset from the record's start.
    pub offset: u32,
    /// The type as clang prints it.
    pub kind: String,
}

/// A tag structure and its fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The structure's name, without `struct`.
    pub name: String,
    /// Its size in bytes.
    pub size: u32,
    /// Its fields, in clang's order.
    pub fields: Vec<Field>,
}

/// A header's own check of a structure's size (`field` is `None`) or of a
/// field's offset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assert {
    /// The structure.
    pub structure: String,
    /// The field, for an offset.
    pub field: Option<String>,
    /// The size or offset the header asserts.
    pub value: u32,
}

/// The number a dump's offset column holds (`12`, or `12:0-3` for a bit
/// field).
fn offset(column: &str) -> Option<u32> {
    column.trim().split(':').next()?.parse().ok()
}

/// A dumped type, with the absolute path clang gives an unnamed record
/// cut back to `source/`.
fn kind(text: &str) -> String {
    match (text.find(" at "), text.find("/source/")) {
        (Some(at), Some(source)) if source > at => {
            format!("{} at {}", &text[..at], &text[source + 1..])
        }
        _ => text.to_owned(),
    }
}

/// clang's `-fdump-record-layouts` output, read into records. A record
/// that is not complete (no offset, no size line) is left out.
pub fn read_dump(text: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for block in text.split("*** Dumping AST Record Layout").skip(1) {
        if let Some(record) = read_block(block) {
            records.push(record);
        }
    }
    records
}

/// One record of the dump.
fn read_block(block: &str) -> Option<Record> {
    let mut lines = block.lines().filter(|line| line.contains('|'));
    let (_, head) = lines.next()?.split_once('|')?;
    let head = head.trim();
    let name = head
        .strip_prefix("struct ")
        .or_else(|| head.strip_prefix("union "))?
        .to_owned();
    let mut fields = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    for line in lines {
        let (column, rest) = line.split_once('|')?;
        if let Some(size) = rest.trim().strip_prefix("[sizeof=") {
            let size = size
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()?;
            return Some(Record { name, size, fields });
        }
        let depth = (rest.len() - rest.trim_start().len()).saturating_sub(1) / 2;
        let text = rest.trim();
        let (kind_text, field) = match text.rsplit_once(' ') {
            Some((kind, field)) if !field.ends_with(')') => (kind, field),
            _ => (text, ""),
        };
        if depth == 0 {
            return None;
        }
        stack.truncate(depth - 1);
        stack.push(field.to_owned());
        fields.push(Field {
            path: stack
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join("."),
            offset: offset(column)?,
            kind: kind(kind_text),
        });
    }
    None
}

/// The structures a header defines (`struct name` with its body on the
/// next line), in order.
pub fn tag_structures(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    lines
        .windows(2)
        .filter(|pair| pair[1].trim() == "{")
        .filter_map(|pair| {
            let name = pair[0].trim().strip_prefix("struct ")?;
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
                .then(|| name.to_owned())
        })
        .collect()
}

/// A C number (`0x48` or `72`).
fn number(text: &str) -> Option<u32> {
    let text = text.trim();
    match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => u32::from_str_radix(hex, 16).ok(),
        None => text.parse().ok(),
    }
}

/// A header's size and offset asserts: `sizeof(struct s) == n` and
/// `offsetof(struct s, f) == n`.
pub fn asserts(text: &str) -> Vec<Assert> {
    let mut found = Vec::new();
    for (word, sized) in [("sizeof(struct ", true), ("offsetof(struct ", false)] {
        let mut rest = text;
        while let Some(at) = rest.find(word) {
            rest = &rest[at + word.len()..];
            let Some(close) = rest.find(')') else { break };
            let inside = &rest[..close];
            let after = &rest[close + 1..];
            let Some(value) = after
                .trim_start()
                .strip_prefix("==")
                .and_then(|v| number(v.split('?').next().unwrap_or_default()))
            else {
                continue;
            };
            let (structure, field) = match inside.split_once(',') {
                Some((s, f)) if !sized => (s.trim(), Some(f.trim().to_owned())),
                None if sized => (inside.trim(), None),
                _ => continue,
            };
            found.push(Assert {
                structure: structure.to_owned(),
                field,
                value,
            });
        }
    }
    found.sort_by_key(|a| a.field.is_some());
    found
}

/// The asserts `records` break, a line each.
pub fn disagreements(records: &[Record], asserts: &[Assert]) -> Vec<String> {
    let mut found = Vec::new();
    for assert in asserts {
        let Some(record) = records.iter().find(|r| r.name == assert.structure) else {
            found.push(format!("{} is not laid out", assert.structure));
            continue;
        };
        let (what, laid) = match &assert.field {
            None => (assert.structure.clone(), Some(record.size)),
            Some(field) => (
                format!("{}.{field}", assert.structure),
                record
                    .fields
                    .iter()
                    .find(|f| &f.path == field)
                    .map(|f| f.offset),
            ),
        };
        if laid != Some(assert.value) {
            found.push(format!(
                "{what}: the header asserts {:#x}, the layout gives {}",
                assert.value,
                laid.map_or_else(|| "nothing".to_owned(), |v| format!("{v:#x}"))
            ));
        }
    }
    found.sort();
    found
}

/// `tag_layout.tsv`'s text: a line for each record (`name`, an empty
/// field, its size, `sizeof`) followed by a line for each field.
pub fn write_layout(records: &[Record]) -> String {
    let mut text = String::from("# structure\tfield\toffset\ttype (opence census --write)\n");
    for record in records {
        text.push_str(&format!("{}\t\t{}\tsizeof\n", record.name, record.size));
        for field in &record.fields {
            text.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                record.name, field.path, field.offset, field.kind
            ));
        }
    }
    text
}

/// `tag_layout.tsv` read back.
pub fn read_layout(text: &str) -> Result<Vec<Record>, Error> {
    let mut records: Vec<Record> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{LAYOUT}"),
            line: index + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [name, path, number, kind] = columns[..] else {
            return Err(malformed("not four columns"));
        };
        let value: u32 = number.parse().map_err(|_| malformed("not a number"))?;
        if path.is_empty() && kind == "sizeof" {
            records.push(Record {
                name: name.to_owned(),
                size: value,
                fields: Vec::new(),
            });
            continue;
        }
        match records.last_mut() {
            Some(record) if record.name == name => record.fields.push(Field {
                path: path.to_owned(),
                offset: value,
                kind: kind.to_owned(),
            }),
            _ => return Err(malformed("a field before its structure")),
        }
    }
    Ok(records)
}

/// A folder of its own under the system's temporary folder, removed when
/// dropped.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Writes `text` at `path`.
fn put(path: &Path, text: &str) -> Result<(), Error> {
    std::fs::write(path, text).map_err(|source| Error::Io {
        path: path.to_owned(),
        source,
    })
}

/// Runs `command`, its standard output, or the problem.
fn run(command: &mut Command, what: &str) -> Result<String, Error> {
    let output = command.output().map_err(|source| Error::Io {
        path: PathBuf::from("clang"),
        source,
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first: Vec<&str> = stderr
            .lines()
            .filter(|l| l.contains("error"))
            .take(5)
            .collect();
        return Err(Error::Problems(vec![format!(
            "{what} failed: {}",
            first.join("; ")
        )]));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The layout of the tag structures of the repository at `root`, from
/// clang, sorted by name.
pub fn generate(root: &Path) -> Result<Vec<Record>, Error> {
    let source = root.join("source");
    let headers: Vec<PathBuf> = walk(&source, &["h"])?
        .into_iter()
        .filter(|h| h.to_string_lossy().ends_with("_definitions.h"))
        .collect();
    let mut names = Vec::new();
    let mut unit = String::new();
    for header in PREAMBLE {
        unit.push_str(&format!("#include \"{header}\"\n"));
    }
    for header in &headers {
        names.extend(tag_structures(&read_text(header)?));
        unit.push_str(&format!("#include \"{}\"\n", header.display()));
    }
    let scratch =
        Scratch(std::env::temp_dir().join(format!("opence-census-layout-{}", std::process::id())));
    let stand_ins = scratch.0.join("include");
    std::fs::create_dir_all(&stand_ins).map_err(|source| Error::Io {
        path: stand_ins.clone(),
        source,
    })?;
    for (name, text) in STAND_INS {
        put(&stand_ins.join(name), text)?;
    }
    let file = scratch.0.join("layout.c");
    put(&file, &unit)?;
    let resources = run(
        Command::new("clang").arg("-print-resource-dir"),
        "clang -print-resource-dir",
    )?;
    let mut clang = Command::new("clang");
    clang
        .args(["-cc1", "-triple", "i686-pc-windows-msvc", "-fsyntax-only"])
        .args(["-fms-extensions", "-fms-compatibility", "-w"])
        .args(["-DHALO_PORT_MAXIMUM_LIGHTS_PER_MAP=896"])
        .arg("-internal-isystem")
        .arg(Path::new(resources.trim()).join("include"))
        .arg(format!("-I{}", stand_ins.display()))
        .arg(format!("-I{}", source.display()));
    let mut folders = vec![source.clone()];
    while let Some(folder) = folders.pop() {
        let entries = std::fs::read_dir(&folder).map_err(|e| Error::Io {
            path: folder.clone(),
            source: e,
        })?;
        let mut found: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir() && !p.ends_with("zlib"))
            .collect();
        found.sort();
        for path in found {
            clang.arg(format!("-I{}", path.display()));
            folders.push(path);
        }
    }
    clang.arg("-fdump-record-layouts-complete").arg(&file);
    let dump = run(&mut clang, "clang's layout of the tag structures")?;
    let mut records: Vec<Record> = read_dump(&dump)
        .into_iter()
        .filter(|r| names.contains(&r.name))
        .collect();
    records.sort_by(|a, b| a.name.cmp(&b.name));
    records.dedup_by(|a, b| a.name == b.name);
    Ok(records)
}
