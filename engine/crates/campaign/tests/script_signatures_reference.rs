//! The engine's script functions against the game's table of them: every
//! function the mission's registry gives the scripts must be in
//! tools/engine_script_signatures_reference.py's trace of
//! `hs_function_table`, returning the game's type and taking the game's
//! parameters, so a script the game parses is parsed alike.

use std::collections::BTreeSet;
use std::error::Error;

use opence_campaign::registry;
use opence_content::ValueType;

const REFERENCE: &str = include_str!("script_signatures_reference.txt");

type Failure = Box<dyn Error>;

fn value_type(field: &str) -> Result<ValueType, Failure> {
    let index: i16 = field.parse()?;
    ValueType::from_index(index).ok_or_else(|| format!("an unknown type: {index}").into())
}

#[test]
fn the_engine_signs_its_functions_as_the_game_signs_them() -> Result<(), Failure> {
    let registry = registry();
    let mut signed = BTreeSet::new();
    let mut natives = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let ["native", name, returns, count, parameters @ ..] = fields.as_slice() else {
            return Err(format!("an unknown line: {line}").into());
        };
        natives += 1;
        assert_eq!(count.parse::<usize>()?, parameters.len(), "{line}");
        let Some((engine_returns, engine_parameters)) = registry.signature(name) else {
            continue;
        };
        let parameters = parameters
            .iter()
            .map(|field| value_type(field))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(engine_returns, value_type(returns)?, "{name} returns");
        assert_eq!(
            engine_parameters,
            parameters.as_slice(),
            "{name}'s parameters"
        );
        signed.insert(*name);
    }
    assert_eq!(natives, 423);
    let unsigned: Vec<&str> = registry
        .function_names()
        .filter(|name| !signed.contains(name))
        .collect();
    assert!(unsigned.is_empty(), "not in the game's table: {unsigned:?}");
    Ok(())
}
