//! The tag fields the engine uses: the importer's labels traced to the
//! fields it fills, and those fields' uses outside the importer.

use std::collections::{BTreeMap, BTreeSet};

use opence_census::engine::{assigned, uses};
use opence_census::fields::{engine, keep, marks, read_fields, write_fields, Engine};
use opence_census::layout::read_dump;

type Failure = Box<dyn std::error::Error>;

const IMPORTER: &str = r#"
fn thing(tag: &Tag) -> Result<Thing, ImportError> {
    let mut flags = tag.u16(0, "flags")?;
    // a comment's "quote" is not a label: x: f(y, "no")
    let parts = tag.block(4, "parts", 8, 16)?;
    Ok(Thing {
        flags,
        mass: parts.get(0).map(|p| p.f32(0, "mass")).transpose()?,
        origin: point(tag, 12, "origin")?,
    })
}
fn other(tag: &Tag, out: &mut Other) {
    out.inner.scale = tag.f32(8, "scale").unwrap_or(1.0);
    *out = fail(tag.fault("odd", format!("is {}", 1)));
}
"#;

#[test]
fn each_label_is_the_field_its_read_fills() {
    let pairs = assigned(IMPORTER);
    let found: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(l, i)| (l.as_str(), i.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            ("flags", "flags"),
            ("parts", "parts"),
            ("mass", "mass"),
            ("origin", "origin"),
            ("scale", "scale"),
        ]
    );
}

#[test]
fn a_use_is_a_field_read_through_a_dot_not_a_method_or_a_comment() {
    let text = r#"
fn draw(thing: &Thing) -> f32 {
    // thing.unused
    let label = "thing.quoted";
    thing.mass * thing.origin.len() + thing.items.iter().count() as f32 + t.0
}
#[cfg(test)]
mod tests {
    fn t(thing: &Thing) { thing.flags; }
}
"#;
    assert_eq!(
        uses(text),
        ["mass".to_owned(), "origin".to_owned(), "items".to_owned()]
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

const DUMP: &str = "
*** Dumping AST Record Layout
         0 | struct thing_definition
         0 |   short flags
         4 |   float mass
         8 |   float scale
        12 |   float unread
           | [sizeof=16, align=4]
";

#[test]
fn a_kept_field_is_used_unused_or_untraced() -> Result<(), Failure> {
    let records = read_dump(DUMP);
    let mut marked = marks(&records, &BTreeSet::new());
    let field = |path: &str| ("thing_definition".to_owned(), path.to_owned());
    let kept: BTreeSet<(String, String)> = [field("flags"), field("mass"), field("scale")].into();
    keep(&mut marked, &records, &kept);
    let labels: BTreeMap<(String, String), BTreeSet<String>> = [
        (field("flags"), ["flags".to_owned()].into()),
        (field("mass"), ["mass".to_owned()].into()),
        (field("scale"), ["passed on".to_owned()].into()),
    ]
    .into();
    let assigned: BTreeMap<String, BTreeSet<String>> = [
        ("flags".to_owned(), ["flags".to_owned()].into()),
        ("mass".to_owned(), ["mass".to_owned()].into()),
    ]
    .into();
    let used: BTreeSet<String> = ["mass".to_owned()].into();
    engine(&mut marked, &records, &labels, &assigned, &used);
    let found: Vec<(&str, Engine)> = marked.iter().map(|m| (m.path.as_str(), m.engine)).collect();
    assert_eq!(
        found,
        [
            ("flags", Engine::Unused),
            ("mass", Engine::Used),
            ("scale", Engine::Untraced),
            ("unread", Engine::Unkept),
        ]
    );
    assert_eq!(read_fields(&write_fields(&marked))?, marked);
    let text = write_fields(&marked);
    assert!(text.contains("\tkept\tused\n"), "{text}");
    assert!(read_fields("t\tf\t0\tshort\tread\tkept\tmaybe\n").is_err());
    Ok(())
}
