//! The tag fields: which of each tag structure's fields the original's C
//! reads through the pointers it declares to it.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use opence_census::fields::{
    declared, leaves, marks, members, read_fields, reads, write_fields, FIELDS,
};
use opence_census::layout::{read_dump, read_layout, Record, LAYOUT};

type Failure = Box<dyn std::error::Error>;

const DUMP: &str = "
*** Dumping AST Record Layout
         0 | struct thing_part
         0 |   float mass
         4 |   short flags
           | [sizeof=8, align=4]

*** Dumping AST Record Layout
         0 | struct thing_definition
         0 |   short flags
         4 |   struct tag_block parts
         4 |     long count
         8 |     void * address
        12 |   union real_point3d origin
        12 |     float[3] n
        12 |     struct real_point3d::(anonymous at source/math/real_math.h:421:2)
        12 |       float x
        16 |       float y
        20 |       float z
        24 |   float scale
        28 |   float unread
           | [sizeof=32, align=4]

*** Dumping AST Record Layout
         0 | struct big_definition
         0 |   struct thing_part part
         0 |     float mass
         4 |     short flags
         8 |   long other
           | [sizeof=12, align=4]
";

const C: &str = "
static void thing_update(struct thing_definition *definition, long index)
{
\tstruct thing_part *part= TAG_BLOCK_GET_ELEMENT(&definition->parts, index, struct thing_part);
\treal_point3d at= definition->origin; /* definition->unread */
\tif (definition->flags & 1) part->mass*= definition->scale;
}

static void big_update(void)
{
\tstruct big_definition const *definition= tag_get('big ', 0);
\tstruct thing_part *part= &definition->part;
\tlong flags= part[0].flags + definition->other;
\tchar *text= \"definition->unread\";
}
";

fn names(records: &[Record]) -> BTreeSet<String> {
    records.iter().map(|r| r.name.clone()).collect()
}

#[test]
fn the_leaves_are_the_fields_with_nothing_inside() {
    let records = read_dump(DUMP);
    let thing = &records[1];
    let paths: Vec<&str> = leaves(thing).iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "flags",
            "parts.count",
            "parts.address",
            "origin.n",
            "origin.x",
            "origin.y",
            "origin.z",
            "scale",
            "unread"
        ]
    );
}

#[test]
fn accesses_through_declared_pointers_are_read_with_their_paths() {
    let records = read_dump(DUMP);
    let found = reads(C, &names(&records), &BTreeMap::new());
    let found: Vec<(&str, &str)> = found
        .iter()
        .map(|(s, p)| (s.as_str(), p.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            ("big_definition", "other"),
            ("big_definition", "part"),
            ("thing_definition", "flags"),
            ("thing_definition", "origin"),
            ("thing_definition", "parts"),
            ("thing_definition", "scale"),
            ("thing_part", "mass"),
        ]
    );
}

#[test]
fn a_member_declared_as_one_structure_only_is_a_pointer_to_it() {
    let records = read_dump(DUMP);
    let header = "struct info { struct thing_part const *part; struct thing_part *both; };\n\
                  struct other { struct big_definition *both; };";
    let found = declared(header, &names(&records));
    assert_eq!(found.len(), 2);
    let members = members(&found);
    assert_eq!(members.len(), 1);
    let text = "void f(struct info *info) { real m= info->part->mass + info->both->other; }\n\
                void g(struct info info) { short s= info.part->flags; }";
    let read: Vec<(String, String)> = reads(text, &names(&records), &members)
        .into_iter()
        .collect();
    assert_eq!(
        read,
        [
            ("thing_part".to_owned(), "flags".to_owned()),
            ("thing_part".to_owned(), "mass".to_owned())
        ]
    );
}

#[test]
fn a_read_inside_an_embedded_structure_is_read_wherever_it_is_embedded() {
    let records = read_dump(DUMP);
    let text = "void f(struct big_definition *big) { short s= big->part.flags; }";
    let marked = marks(&records, &reads(text, &names(&records), &BTreeMap::new()));
    let read: Vec<(&str, &str)> = marked
        .iter()
        .filter(|m| m.original)
        .map(|m| (m.structure.as_str(), m.path.as_str()))
        .collect();
    assert_eq!(
        read,
        [("thing_part", "flags"), ("big_definition", "part.flags")]
    );
}

#[test]
fn every_leaf_is_marked_read_or_not() {
    let records = read_dump(DUMP);
    let found = reads(C, &names(&records), &BTreeMap::new());
    let marked = marks(&records, &found);
    let read: Vec<(&str, &str)> = marked
        .iter()
        .filter(|m| m.original)
        .map(|m| (m.structure.as_str(), m.path.as_str()))
        .collect();
    assert_eq!(
        read,
        [
            ("thing_part", "mass"),
            ("thing_definition", "flags"),
            ("thing_definition", "parts.count"),
            ("thing_definition", "parts.address"),
            ("thing_definition", "origin.n"),
            ("thing_definition", "origin.x"),
            ("thing_definition", "origin.y"),
            ("thing_definition", "origin.z"),
            ("thing_definition", "scale"),
            // A field the original reads from the structure it is part of
            // is read wherever that structure is embedded.
            ("big_definition", "part.mass"),
            ("big_definition", "other"),
        ]
    );
    let unread: Vec<&str> = marked
        .iter()
        .filter(|m| !m.original)
        .map(|m| m.path.as_str())
        .collect();
    assert_eq!(unread, ["flags", "unread", "part.flags"]);
}

#[test]
fn the_fields_round_trip_through_their_tsv() -> Result<(), Failure> {
    let records = read_dump(DUMP);
    let marked = marks(&records, &reads(C, &names(&records), &BTreeMap::new()));
    assert_eq!(read_fields(&write_fields(&marked))?, marked);
    assert!(read_fields("thing\tflags\tx\tshort\tread\n").is_err());
    assert!(read_fields("thing\tflags\t0\tshort\tmaybe\t-\t-\n").is_err());
    Ok(())
}

#[test]
fn a_file_the_reader_cannot_follow_gives_fewer_reads_not_a_panic() {
    let records = read_dump(DUMP);
    for text in [
        "struct thing_definition *",
        "struct thing_definition *d; d->",
        "d->flags",
        "struct thing_definition *d; d->origin[",
        "\"",
        "/*",
    ] {
        assert!(
            reads(text, &names(&records), &BTreeMap::new()).is_empty(),
            "{text:?}"
        );
    }
}

/// The written fields are the marks of the written layout against
/// `source/`'s C as it is now.
#[test]
fn the_written_fields_are_current() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(());
    };
    let records = read_layout(&std::fs::read_to_string(root.join("engine").join(LAYOUT))?)?;
    let written = read_fields(&std::fs::read_to_string(root.join("engine").join(FIELDS))?)?;
    let fresh = opence_census::fields::generate(&root, &records)?;
    assert!(
        written == fresh,
        "engine/{FIELDS} is stale: run opence census --write"
    );
    let read = fresh.iter().filter(|m| m.original).count();
    assert!(read > 1000, "only {read} fields read");
    Ok(())
}
