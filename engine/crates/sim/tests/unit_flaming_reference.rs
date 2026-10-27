//! The engine's unit running blindly and burning to death against the
//! game's: what it does to the world, in order, what it draws from the
//! game's seed, the way it runs and what it leaves in the unit, as
//! tools/engine_unit_flaming_reference.py records the game's code doing it
//! over random units and worlds.

use std::error::Error;
use std::fmt::Write as _;

use glam::Vec3;
use opence_sim::random::RandomSeed;
use opence_sim::unit_control::UnitFlags;
use opence_sim::unit_flaming::{Burning, FlameDamage, FlameWorld, Owners, RunBlind};
use opence_sim::unit_life::DamageFlags;

const REFERENCE: &str = include_str!("unit_flaming_reference.txt");

type Failure = Box<dyn Error>;

/// The game's index (or -1) as an object.
fn object(field: &str) -> Result<Option<u32>, Failure> {
    let value: i32 = field.parse()?;
    Ok(u32::try_from(value).ok())
}

/// An index as the game writes it, -1 for none.
fn game<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "-1".to_owned(), |value| value.to_string())
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let [i, j, k] = fields else {
        return Err("a vector missing".into());
    };
    Ok(Vec3::new(i.parse()?, j.parse()?, k.parse()?))
}

/// The fake world, writing down what is done to it as the game's does.
#[derive(Default)]
struct World {
    /// Objects 1 to 7: their owners, if they are there.
    objects: Vec<Option<Owners>>,
    effect: Option<u32>,
    kills: bool,
    calls: String,
}

impl FlameWorld for World {
    fn drop_current_weapon(&mut self) {
        self.calls.push_str(" drop_current_weapon 0 1");
    }

    fn flaming_death_effect(&self) -> Option<u32> {
        self.effect
    }

    fn owners(&self, object: u32) -> Option<Owners> {
        let index = usize::try_from(object).ok()?.checked_sub(1)?;
        self.objects.get(index).copied().flatten()
    }

    fn cause_damage(&mut self, damage: FlameDamage, damage_flags: &mut DamageFlags) {
        // the rest of the damage data as damage_data_new leaves it, dealt
        // to the unit with no node, region or material and no normal
        let _ = write!(
            self.calls,
            " damage {} 0 {} {} {} -1 {:08x} {:08x} 00000000 -1 0 -1 -1 -1 0",
            damage.effect,
            game(damage.owners.player),
            game(damage.owners.object),
            game(damage.owners.team),
            1.0_f32.to_bits(),
            1.0_f32.to_bits()
        );
        if self.kills {
            damage_flags.set(DamageFlags::DEAD, true);
        }
    }
}

/// The world's objects, from exists:player:object:team.
fn objects(fields: &[&str]) -> Result<Vec<Option<Owners>>, Failure> {
    fields
        .iter()
        .map(|field| {
            let parts: Vec<&str> = field.split(':').collect();
            let [exists, player, owner, team] = parts[..] else {
                return Err(Failure::from("an object's fields"));
            };
            let team: i16 = team.parse()?;
            let owners = Owners {
                player: object(player)?,
                object: object(owner)?,
                team: (team >= 0).then_some(team),
            };
            Ok((exists != "0").then_some(owners))
        })
        .collect()
}

/// One case run by the engine: its calls, the way run and the unit after.
fn run(line: &str) -> Result<String, Failure> {
    let parts: Vec<&str> = line.split('|').map(str::trim).collect();
    let [head, unit_text, objects_text, game_text, ..] = parts[..] else {
        return Err("a case missing".into());
    };
    let head: Vec<&str> = head.split_whitespace().collect();
    let [name, attacker] = head[..] else {
        return Err("a case's head".into());
    };
    let attacker = object(attacker)?;
    let fields: Vec<&str> = unit_text.split_whitespace().collect();
    let &[flags, damage_flags, actor, has_way, ref way @ .., angle, delta, delay, burner] =
        &fields[..]
    else {
        return Err("a unit's fields".into());
    };
    let [way_i, way_j, way_k, forward_i, forward_j, forward_k] = way[..] else {
        return Err("a unit's way and facing".into());
    };
    let mut unit = Burning {
        flags: UnitFlags(flags.parse()?),
        damage_flags: DamageFlags(damage_flags.parse()?),
        run: RunBlind {
            angle: angle.parse()?,
            delta: delta.parse()?,
        },
        delay: delay.parse()?,
        attacker: object(burner)?,
    };
    // the actor's way, when the unit has an actor and the actor a way
    let actor_way = (actor != "-1" && has_way != "0")
        .then(|| vector(&[way_i, way_j, way_k]))
        .transpose()?;
    let forward = vector(&[forward_i, forward_j, forward_k])?;
    let object_fields: Vec<&str> = objects_text.split_whitespace().collect();
    let game_fields: Vec<&str> = game_text.split_whitespace().collect();
    let [count, effect, kills, seed] = game_fields[..] else {
        return Err("the game's fields".into());
    };
    let mut world = World {
        objects: objects(&object_fields)?,
        effect: if count == "0" { None } else { object(effect)? },
        kills: kills != "0",
        calls: String::new(),
    };
    let mut random = RandomSeed(seed.parse()?);
    let mut way = Vec3::ZERO;
    match name {
        "start_running_blindly" => unit.start_running_blindly(actor_way, forward, &mut random),
        "running_blind" => way = unit.run.run(actor_way, &mut random),
        "start_flaming_to_death" => {
            unit.start_flaming_to_death(attacker, actor_way, forward, &mut world, &mut random);
        }
        "flame_to_death" => unit.flame_to_death(&mut world),
        _ => return Err(format!("an unknown function {name}").into()),
    }
    Ok(format!(
        "{} -> {:08x} {:08x} {:08x} | {} {} {:08x} {:08x} {} {} {}",
        world.calls.trim_start(),
        way.x.to_bits(),
        way.y.to_bits(),
        way.z.to_bits(),
        unit.flags.0,
        unit.damage_flags.0,
        unit.run.angle.to_bits(),
        unit.run.delta.to_bits(),
        unit.delay,
        game(unit.attacker),
        random.0
    ))
}

#[test]
fn the_engine_runs_blindly_and_burns_as_the_game_does() -> Result<(), Failure> {
    let mut cases = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (left, right) = line.rsplit_once(" -> ").ok_or("a case without a way")?;
        let calls = left.rsplit_once('|').map_or("", |(_, calls)| calls).trim();
        let expected = format!("{calls} -> {right}");
        let got = run(line)?;
        assert_eq!(got.trim_start(), expected.trim_start(), "{line}");
        cases += 1;
    }
    assert_eq!(cases, 10_000);
    Ok(())
}
