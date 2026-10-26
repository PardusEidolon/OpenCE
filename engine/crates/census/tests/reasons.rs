//! The real tree's reasons: what the census classes as done by a library,
//! and as the editor and tools.

mod common;

use opence_census::classes::Class;
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

/// The functions of `files` that are neither Replaced nor in `kept`.
fn not_replaced(census: &Census, files: &[&str], kept: &[&str]) -> Vec<String> {
    census
        .functions
        .iter()
        .filter(|f| files.contains(&f.file.as_str()))
        .filter(|f| f.class != Some(Class::Replaced) && !kept.contains(&f.name.as_str()))
        .map(|f| format!("{} ({}): {:?}", f.name, f.file, f.class))
        .collect()
}

#[test]
fn the_collision_bsp_walk_is_done_by_jolt() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let left = not_replaced(
        &census,
        &[
            "physics/collision_bsp.c",
            "physics/bsp2d.c",
            "physics/bsp3d.c",
        ],
        // (the point test is the engine's own, matched to the original)
        &["bsp3d_test_point"],
    );
    assert!(left.is_empty(), "{left:#?}");
    Ok(())
}
