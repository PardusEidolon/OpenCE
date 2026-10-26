//! The tag fields the engine uses. The importer labels each read with the
//! field it is reading; the label's call is assigned to a field or local of
//! the importer's own types, and a field the engine reads through a `.`
//! outside the importer is used.
//!
//! The match is by name, so it is approximate: a name shared by two types
//! counts a use of either as a use of both.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::{read_text, walk, Error};

/// The crates whose code is not the engine's: the importer itself, this
/// census and the command line's tools.
const NOT_ENGINE: [&str; 3] = ["import", "census", "cli"];

/// `text` with its comments and the insides of its string literals
/// blanked, and the spans of those literals' insides.
fn code(text: &str) -> (Vec<u8>, Vec<(usize, usize)>) {
    let mut bytes = text.as_bytes().to_vec();
    let mut literals = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'/' if bytes.get(at + 1) == Some(&b'/') => {
                while at < bytes.len() && bytes[at] != b'\n' {
                    bytes[at] = b' ';
                    at += 1;
                }
            }
            // A character literal, so that '"' opens no string (a
            // lifetime has no closing quote two or three bytes on).
            b'\'' if bytes.get(at + 2) == Some(&b'\'') => at += 3,
            b'\'' if bytes.get(at + 1) == Some(&b'\\') && bytes.get(at + 3) == Some(&b'\'') => {
                at += 4;
            }
            b'"' => {
                let start = at + 1;
                at = start;
                while at < bytes.len() && bytes[at] != b'"' {
                    at += if bytes[at] == b'\\' { 2 } else { 1 };
                }
                let end = at.min(bytes.len());
                literals.push((start, end));
                for byte in &mut bytes[start..end] {
                    *byte = b' ';
                }
                at = end + 1;
            }
            _ => at += 1,
        }
    }
    (bytes, literals)
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The identifier `segment` assigns: `let (mut) x =`, `x:` or
/// `path.x =`.
fn target(segment: &str) -> Option<String> {
    let segment = segment.trim_start();
    let (segment, binding) = match segment.strip_prefix("let ") {
        Some(rest) => (
            rest.trim_start().trim_start_matches("mut ").trim_start(),
            true,
        ),
        None => (segment, false),
    };
    let path_end = segment
        .bytes()
        .position(|b| !(is_ident(b) || b == b'.'))
        .unwrap_or(segment.len());
    let path = &segment[..path_end];
    let ident = path.rsplit('.').next().unwrap_or_default();
    if ident.is_empty() || ident.bytes().next().is_some_and(|b| b.is_ascii_digit()) {
        return None;
    }
    let rest = segment[path_end..].trim_start();
    let assigns = rest.starts_with('=') && !rest.starts_with("==");
    let typed = rest.starts_with(':') && !rest.starts_with("::");
    let fits = if binding {
        !path.contains('.') && (assigns || typed)
    } else if path.contains('.') {
        assigns
    } else {
        typed
    };
    fits.then(|| ident.to_owned())
}

/// Each string literal of `text` that is an argument of a call, with the
/// identifier the call's value is assigned to, in order.
pub fn assigned(text: &str) -> Vec<(String, String)> {
    let (bytes, literals) = code(text);
    let mut found = Vec::new();
    for (start, end) in literals {
        let open = start - 1;
        let before = bytes[..open].iter().rposition(|b| !b.is_ascii_whitespace());
        if !before.is_some_and(|at| bytes[at] == b'(' || bytes[at] == b',') {
            continue;
        }
        let mut level = 0usize;
        let mut exited = false;
        let mut stop = None;
        for at in (0..open).rev() {
            match bytes[at] {
                b')' | b']' => level += 1,
                b'(' | b'[' if level > 0 => level -= 1,
                b'(' | b'[' => exited = true,
                b',' | b';' | b'{' | b'}' if level == 0 && exited => {
                    stop = Some(at);
                    break;
                }
                b';' | b'{' | b'}' if level == 0 => break,
                _ => {}
            }
        }
        let Some(stop) = stop else {
            continue;
        };
        let segment = String::from_utf8_lossy(&bytes[stop + 1..open]);
        if let Some(ident) = target(&segment) {
            found.push((text[start..end].to_owned(), ident));
        }
    }
    found
}

/// The fields `text` reads through a `.` (not methods, paths, tuple
/// indices, comments, strings or its `#[cfg(test)]` module).
pub fn uses(text: &str) -> BTreeSet<String> {
    let text = text.find("#[cfg(test)]").map_or(text, |at| &text[..at]);
    let (bytes, _) = code(text);
    let mut found = BTreeSet::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'.' || at > 0 && bytes[at - 1] == b'.' {
            at += 1;
            continue;
        }
        let start = at + 1;
        let mut end = start;
        while end < bytes.len() && is_ident(bytes[end]) {
            end += 1;
        }
        at = end.max(start);
        if end == start || bytes[start].is_ascii_digit() {
            continue;
        }
        let rest = &bytes[end..];
        let skip = rest
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(rest.len());
        let rest = &rest[skip..];
        if rest.starts_with(b"(") || rest.starts_with(b"::") {
            continue;
        }
        found.insert(String::from_utf8_lossy(&bytes[start..end]).into_owned());
    }
    found
}

/// Each of the importer's labels with the identifiers it is assigned to.
pub type Labels = BTreeMap<String, BTreeSet<String>>;

/// The importer's labels with the identifiers they are assigned to, and
/// the fields the engine's crates read, in the repository at `root` (none
/// when it has no crates).
pub fn traced(root: &Path) -> Result<(Labels, BTreeSet<String>), Error> {
    let crates = root.join("engine").join("crates");
    let mut labels = Labels::new();
    let importer = crates.join("import").join("src");
    if !importer.is_dir() {
        return Ok((labels, BTreeSet::new()));
    }
    for file in walk(&importer, &["rs"])? {
        for (label, ident) in assigned(&read_text(&file)?) {
            labels.entry(label).or_default().insert(ident);
        }
    }
    let mut used = BTreeSet::new();
    let mut names: Vec<_> = std::fs::read_dir(&crates)
        .map_err(|source| Error::Io {
            path: crates.clone(),
            source,
        })?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| !NOT_ENGINE.contains(&name.as_str()))
        .collect();
    names.sort();
    for name in names {
        let src = crates.join(&name).join("src");
        if !src.is_dir() {
            continue;
        }
        for file in walk(&src, &["rs"])? {
            used.extend(uses(&read_text(&file)?));
        }
    }
    Ok((labels, used))
}
