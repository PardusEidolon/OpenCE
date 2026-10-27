//! The engine's damage to one object against the game's: the shield,
//! body, stun, depletion, death, instant kills and material type that
//! tools/engine_cause_damage_reference.py records from damage.c's
//! `object_cause_damage` must be what `cause_damage` leaves.

use std::error::Error;

use opence_content::{CollisionModel, CollisionModelMaterial, DamageEffect, Group, TagRef};
use opence_sim::damage::{cause_damage, DamageHit, Vitality, LETHAL_TO_THE_UNSUSPECTING};

const REFERENCE: &str = include_str!("cause_damage_reference.txt");

type Failure = Box<dyn Error>;

/// A stun the damage has not set: the engine sets none so long.
const UNSTUNNED: u32 = u32::MAX;

fn real(field: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(field, 16)?))
}

fn number(field: &str) -> Result<i64, Failure> {
    Ok(field.parse()?)
}

fn tag(path: &str) -> TagRef {
    TagRef {
        group: Group(*b"coll"),
        path: path.into(),
    }
}

/// One of the line's parts between its bars.
fn part<'a>(parts: &[&'a [&'a str]], at: usize) -> Result<&'a [&'a str], Failure> {
    parts
        .get(at)
        .copied()
        .ok_or_else(|| "a part missing".into())
}

fn field<'a>(fields: &[&'a str], at: usize) -> Result<&'a str, Failure> {
    fields
        .get(at)
        .copied()
        .ok_or_else(|| "a field missing".into())
}

/// What the engine leaves of one hit, in the trace's words.
fn engine(case: &[&str]) -> Result<String, Failure> {
    let parts: Vec<&[&str]> = case.split(|field| *field == "|").collect();
    let object = part(&parts, 0)?;
    let resistance = part(&parts, 1)?;
    let materials = part(&parts, 2)?;
    let modifiers = part(&parts, 3)?;
    let side_effect: i16 = field(resistance, 1)?.parse()?;
    let unsuspecting = field(object, 8)? == "1";

    let mut struck_materials = Vec::new();
    for each in materials.get(2..).ok_or("no materials")?.chunks(5) {
        struck_materials.push(CollisionModelMaterial {
            name: "material".into(),
            flags: field(each, 0)?.parse()?,
            material_type: field(each, 1)?.parse()?,
            shield_leak_percentage: real(field(each, 2)?)?,
            shield_damage_multiplier: real(field(each, 3)?)?,
            body_damage_multiplier: real(field(each, 4)?)?,
        });
    }
    let model = CollisionModel {
        tag: tag("object"),
        flags: field(resistance, 4)?.parse()?,
        indirect_damage_material: u16::try_from(number(field(materials, 1)?)?).ok(),
        maximum_body_vitality: real(field(object, 0)?)?,
        friendly_damage_resistance: real(field(resistance, 6)?)?,
        body_damaged_threshold: 0.0,
        body_destroyed_threshold: 0.0,
        maximum_shield_vitality: real(field(object, 1)?)?,
        shield_material_type: field(resistance, 5)?.parse()?,
        shield_failure_function: field(resistance, 11)?.parse()?,
        shield_failure_threshold: real(field(resistance, 8)?)?,
        failing_shield_leak_fraction: real(field(resistance, 9)?)?,
        minimum_stun_damage: real(field(resistance, 7)?)?,
        stun_time: real(field(resistance, 10)?)?,
        recharge_time: 0.0,
        shield_damaged_threshold: 0.0,
        materials: struck_materials,
        regions: Vec::new(),
        pathfinding_box: Default::default(),
        pathfinding_spheres: Vec::new(),
        nodes: Vec::new(),
    };
    let mut material_modifiers = vec![1.0; DamageEffect::MATERIAL_TYPES];
    for (modifier, value) in material_modifiers.iter_mut().zip(modifiers) {
        *modifier = real(value)?;
    }
    let damage = real(field(object, 5)?)?;
    let effect = DamageEffect {
        tag: tag("effect"),
        radius: [0.0; 2],
        cutoff_scale: 0.0,
        flags: 0,
        sound: None,
        side_effect,
        category: field(resistance, 2)?.parse()?,
        damage_flags: field(resistance, 0)?.parse()?,
        area_of_effect_core_radius: 0.0,
        damage_minimum: real(field(object, 6)?)?,
        damage: [damage, damage],
        vehicle_passthrough_penalty: 0.0,
        active_camouflage_damage: 0.0,
        stun: 0.0,
        maximum_stun: 0.0,
        stun_time: 0.0,
        instantaneous_acceleration: 0.0,
        material_modifiers,
    };
    let hit = DamageHit {
        scale: real(field(resistance, 3)?)?,
        material: u16::try_from(number(field(materials, 0)?)?).ok(),
        friendly: field(object, 7)? == "1",
        unsuspecting,
        ..DamageHit::default()
    };
    let mut vitality = Vitality {
        body: real(field(object, 2)?)?,
        shield: real(field(object, 3)?)?,
        stun_ticks: UNSTUNNED,
        shield_depleted: field(object, 4)? == "1",
        dead: false,
        region_damage: Vec::new(),
        regions_destroyed: Vec::new(),
    };
    // (damage of nothing does nothing, nor kills)
    let total = (1.0 - hit.scale) * effect.damage_minimum + damage * hit.scale;
    let outcome = cause_damage(&mut vitality, &model, &effect, &hit);
    let killed_instantly = outcome.headshot
        || (side_effect == LETHAL_TO_THE_UNSUSPECTING && unsuspecting && total > 0.0);
    Ok(format!(
        "{:08x} {:08x} {} {} {} {} {} {}",
        vitality.body.to_bits(),
        vitality.shield.to_bits(),
        u8::from(vitality.stun_ticks != UNSTUNNED),
        u8::from(vitality.shield_depleted),
        u8::from(vitality.dead),
        u8::from(outcome.killed),
        u8::from(killed_instantly),
        outcome.material_type,
    ))
}

#[test]
fn the_engine_damages_an_object_as_the_game_does() -> Result<(), Failure> {
    let (mut cases, mut killed, mut shielded) = (0, 0, 0);
    for (number_of_line, line) in REFERENCE.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = number_of_line + 1;
        let fields: Vec<&str> = line.split_whitespace().collect();
        let ["hit", rest @ ..] = fields.as_slice() else {
            return Err(format!("an unknown line: {line}").into());
        };
        let arrow = rest
            .iter()
            .position(|field| *field == "->")
            .ok_or("no answer")?;
        let (case, answer) = rest.split_at(arrow);
        let answer = answer.get(1..).ok_or("no answer")?.join(" ");
        assert_eq!(engine(case)?, answer, "line {at}: {line}");
        killed += usize::from(answer.split(' ').nth(5) == Some("1"));
        shielded += usize::from(answer.split(' ').nth(3) == Some("1"));
        cases += 1;
    }
    assert_eq!(cases, 6000);
    assert!(killed > 1000, "{killed}");
    assert!(shielded > 1000, "{shielded}");
    Ok(())
}
