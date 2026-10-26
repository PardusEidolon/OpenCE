//! The tag layout: every tag structure of `source/`'s `*_definitions.h`,
//! laid out by clang as the original's compiler laid it out, and checked
//! against the headers' own size and offset asserts.

mod common;

use common::Tree;
use opence_census::layout::{
    asserts, disagreements, generate, is_tag_header, read_dump, read_layout, tag_structures,
    write_layout, Assert, LAYOUT,
};

type Failure = Box<dyn std::error::Error>;

const DUMP: &str = "
*** Dumping AST Record Layout
         0 | struct tag_block
         0 |   long count
         4 |   void * address
           | [sizeof=8, align=4]

*** Dumping AST Record Layout
         0 | struct thing_definition
         0 |   short flags
         4 |   struct tag_block parts
         4 |     long count
         8 |     void * address
     12:0-3 |   unsigned int mode
        16 |   byte[16] unused
           | [sizeof=32, align=4]
";

const HEADER: &str = "
struct thing_part
{
\treal mass;
};

struct thing_definition
{
\tshort flags;
\tshort pad;
\tstruct tag_block parts;
\treal scale;
};

typedef char verify_thing_definition_size[
\tsizeof(struct thing_definition) == 0x14 ? 1 : -1];
typedef char verify_thing_definition_scale[
\toffsetof(struct thing_definition, scale) == 0x10 ? 1 : -1];
";

#[test]
fn clangs_dump_is_read_into_records_with_their_fields() {
    let records = read_dump(DUMP);
    assert_eq!(records.len(), 2);
    let thing = &records[1];
    assert_eq!((thing.name.as_str(), thing.size), ("thing_definition", 32));
    let fields: Vec<(&str, u32, &str)> = thing
        .fields
        .iter()
        .map(|f| (f.path.as_str(), f.offset, f.kind.as_str()))
        .collect();
    assert_eq!(
        fields,
        [
            ("flags", 0, "short"),
            ("parts", 4, "struct tag_block"),
            ("parts.count", 4, "long"),
            ("parts.address", 8, "void *"),
            ("mode", 12, "unsigned int"),
            ("unused", 16, "byte[16]"),
        ]
    );
}

#[test]
fn a_malformed_dump_gives_no_record_rather_than_a_panic() {
    for text in [
        "*** Dumping AST Record Layout\n",
        "*** Dumping AST Record Layout\n  x | struct a\n",
        "*** Dumping AST Record Layout\n 0 | struct a\n 0 |   long b\n",
        "| [sizeof=",
    ] {
        assert!(read_dump(text).is_empty(), "{text:?}");
    }
}

#[test]
fn a_headers_tag_structures_and_asserts_are_read() {
    assert_eq!(tag_structures(HEADER), ["thing_part", "thing_definition"]);
    assert_eq!(
        asserts(HEADER),
        [
            Assert {
                structure: "thing_definition".into(),
                field: None,
                value: 0x14,
            },
            Assert {
                structure: "thing_definition".into(),
                field: Some("scale".into()),
                value: 0x10,
            },
        ]
    );
}

#[test]
fn a_tag_header_is_a_definitions_header_or_one_that_names_a_tag_group() {
    let path = std::path::Path::new;
    assert!(is_tag_header(path("units/unit_definitions.h"), ""));
    assert!(is_tag_header(
        path("bitmaps/bitmap_group.h"),
        "enum\n{\n\tBITMAP_GROUP_TAG= 'bitm'\n};\n"
    ));
    assert!(!is_tag_header(path("math/real_math.h"), "enum { X= 1 };"));
    assert!(!is_tag_header(path("units/units.h"), "#define TAG 'unit'"));
}

#[test]
fn the_layout_round_trips_through_its_tsv() -> Result<(), Failure> {
    let records = read_dump(DUMP);
    assert_eq!(read_layout(&write_layout(&records))?, records);
    assert!(read_layout("thing\tflags\tnot a number\tshort\n").is_err());
    Ok(())
}

#[test]
fn an_assert_the_layout_breaks_is_reported() {
    let records = read_dump(DUMP);
    let wrong = [
        Assert {
            structure: "thing_definition".into(),
            field: None,
            value: 32,
        },
        Assert {
            structure: "thing_definition".into(),
            field: Some("mode".into()),
            value: 8,
        },
        Assert {
            structure: "gone_definition".into(),
            field: None,
            value: 4,
        },
    ];
    let found = disagreements(&records, &wrong);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].contains("gone_definition"), "{found:?}");
    assert!(found[1].contains("thing_definition.mode"), "{found:?}");
}

/// clang lays out a fake `*_definitions.h` as the original's compiler
/// did (skipped where clang is not on the path).
#[test]
fn clang_lays_out_a_fake_definitions_header() -> Result<(), Failure> {
    if std::process::Command::new("clang")
        .arg("--version")
        .output()
        .is_err()
    {
        return Ok(());
    }
    let tree = Tree::new("layout")
        .with(
            "source/cseries/cseries.h",
            "#include <StdDef.h>\ntypedef float real;\ntypedef unsigned char byte;\n",
        )
        .with(
            "source/tag_files/tag_groups.h",
            "struct tag_block { long count; void *address; void *definition; };\n",
        )
        .with("source/math/real_math.h", "\n")
        .with("source/math/integer_math.h", "\n")
        .with("source/game/game_globals.h", "\n")
        .with("source/things/thing_definitions.h", HEADER);
    let records = generate(tree.root())?;
    let names: Vec<&str> = records.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["thing_definition", "thing_part"]);
    assert_eq!(records[0].size, 0x14);
    let scale = records[0].fields.iter().find(|f| f.path == "scale");
    assert_eq!(scale.map(|f| f.offset), Some(0x10));
    assert_eq!(
        disagreements(&records, &asserts(HEADER)),
        Vec::<String>::new()
    );
    Ok(())
}

/// The written layout holds every tag structure and agrees with every
/// size and offset assert in `source/`'s headers.
#[test]
fn the_written_layout_agrees_with_the_headers_asserts() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(());
    };
    let records = read_layout(&std::fs::read_to_string(root.join("engine").join(LAYOUT))?)?;
    let mut all = Vec::new();
    let mut names = Vec::new();
    for header in opence_census::walk(&root.join("source"), &["h"])? {
        let text = opence_census::read_text(&header)?;
        if is_tag_header(&header, &text) {
            all.extend(asserts(&text));
            names.extend(tag_structures(&text));
        }
    }
    assert!(all.len() > 50, "only {} asserts", all.len());
    assert_eq!(disagreements(&records, &all), Vec::<String>::new());
    let missing: Vec<&String> = names
        .iter()
        .filter(|n| !records.iter().any(|r| &r.name == *n))
        .collect();
    assert!(missing.is_empty(), "not laid out: {missing:?}");
    Ok(())
}
