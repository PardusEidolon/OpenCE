//! The coverage census's record of what the engine gives scripts:
//! `engine/census/script_wired.tsv`, the runtime's special forms and the
//! registry's natives and globals. The census marks the original's tables
//! with it; run this test with `OPENCE_RECORD=1` to write it again.

use std::error::Error;
use std::path::PathBuf;

use opence_campaign::registry;
use opence_script::SPECIAL_FORMS;

#[test]
fn the_census_has_what_the_engine_gives_scripts() -> Result<(), Box<dyn Error>> {
    let mut text = String::from(
        "# What the engine gives scripts (the campaign crate's tests/census.rs writes it): special, function or global, and the name.\n",
    );
    let registry = registry();
    for name in SPECIAL_FORMS {
        text.push_str(&format!("special\t{name}\n"));
    }
    for name in registry.function_names() {
        text.push_str(&format!("function\t{name}\n"));
    }
    for name in registry.global_names() {
        text.push_str(&format!("global\t{name}\n"));
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../census/script_wired.tsv");
    if std::env::var_os("OPENCE_RECORD").is_some() {
        std::fs::write(&path, &text)?;
    }
    let written = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        written == text,
        "engine/census/script_wired.tsv is stale: run this test with OPENCE_RECORD=1, then opence census --write"
    );
    Ok(())
}
