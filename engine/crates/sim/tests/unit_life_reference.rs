//! The engine's unit life against the game's: the hooks that do nothing,
//! the quiet deaths, a biped's flight, surfaces and melee, whether a unit
//! speaks and the equipment it holds, as
//! tools/engine_unit_life_reference.py records the game's code doing them.

use std::error::Error;

use opence_sim::unit_life::{
    flying_through_air, is_speaking, max_relaxation_iterations, stop_melee_attack, BipedAir,
    BipedSurfaces, DamageFlags, UnitEquipment,
};

const REFERENCE: &str = include_str!("unit_life_reference.txt");

type Failure = Box<dyn Error>;

/// `None` for the game's -1 (or any negative), otherwise the value.
fn index(field: &str) -> Result<Option<u32>, Failure> {
    let value: i64 = field.parse()?;
    Ok(if value < 0 {
        None
    } else {
        Some(u32::try_from(value)?)
    })
}

/// The game's index: -1 for none.
fn game_index(index: Option<u32>) -> i64 {
    index.map_or(-1, i64::from)
}

fn bit(value: bool) -> u8 {
    u8::from(value)
}

/// The biped type (`_object_type_biped`).
const BIPED: &str = "0";

fn case(line: &str) -> Result<String, Failure> {
    let (asked, _) = line.rsplit_once(" -> ").ok_or("no `->`")?;
    let fields: Vec<&str> = asked.split(' ').collect();
    Ok(match fields.as_slice() {
        // the engine has nothing to do where the game does nothing
        ["hook", _] => "unchanged".to_owned(),
        ["kill", silent, flags] => {
            let mut flags = DamageFlags(flags.parse()?);
            if *silent == "1" {
                flags.kill_silent();
            } else {
                flags.kill_no_statistics();
            }
            flags.0.to_string()
        }
        ["air", kind, flags, flies, ticks] => {
            let biped = BipedAir {
                airborne_ticks: ticks.parse()?,
                flying: *flies == "1",
                dead: DamageFlags(flags.parse()?).has(DamageFlags::DEAD),
            };
            let flying = biped.flying_through_air();
            if *kind == BIPED {
                format!("{} {}", bit(flying_through_air(Some(&biped))), bit(flying))
            } else {
                bit(flying_through_air(None)).to_string()
            }
        }
        ["disconnect", support, pathfinding, last] => {
            let mut surfaces = BipedSurfaces {
                support: index(support)?,
                pathfinding: index(pathfinding)?,
                last_pathfinding: index(last)?,
            };
            surfaces.disconnect_from_structure_bsp();
            format!(
                "{} {} {}",
                game_index(surfaces.support),
                game_index(surfaces.pathfinding),
                game_index(surfaces.last_pathfinding)
            )
        }
        ["stop_melee", biped, ticks] => {
            let mut ticks: i8 = ticks.parse()?;
            stop_melee_attack(index(biped)?.map(|_| &mut ticks));
            ticks.to_string()
        }
        ["speaking", priority] => bit(is_speaking(priority.parse()?)).to_string(),
        ["equipment", equipment] => {
            game_index(UnitEquipment(index(equipment)?).current()).to_string()
        }
        ["relaxation"] => max_relaxation_iterations().to_string(),
        _ => return Err(format!("an unknown case: {line}").into()),
    })
}

#[test]
fn the_unit_life_is_the_games() -> Result<(), Failure> {
    let mut cases = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (_, expected) = line.rsplit_once(" -> ").ok_or("no `->`")?;
        assert_eq!(case(line)?, expected, "{line}");
        cases += 1;
    }
    assert!(cases > 9000, "only {cases} cases");
    Ok(())
}

#[test]
fn a_quiet_death_keeps_the_other_flags() {
    let mut flags = DamageFlags(DamageFlags::DEAD);
    flags.kill_silent();
    flags.kill_no_statistics();
    assert!(flags.has(DamageFlags::DEAD));
    assert!(flags.has(DamageFlags::DIE_ACT_OF_GOD_SILENT));
    assert!(flags.has(DamageFlags::DIE_ACT_OF_GOD_NO_STATISTICS));
}
