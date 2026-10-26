//! An animation's permutation is chosen as the game chooses it:
//! tools/engine_animation_permutation_reference.py runs model_animations.c's
//! `animation_choose_random_permutation_internal` over a graph whose lists
//! are a run, one alone, a loop, a link past the graph's last and a run
//! that ends before its last weight, from each first animation at numbers
//! from 0 to 1, and the engine's choice is asked the same and compared.

use opence_content::{Animation, AnimationKind};
use opence_sim::animation::choose_permutation;

const REFERENCE: &str = include_str!("animation_permutation_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

fn index(fields: &[&str], at: usize) -> Result<Option<u16>, Failure> {
    let value: i32 = fields.get(at).ok_or("a short line")?.parse()?;
    Ok(u16::try_from(value).ok())
}

fn animation(normalized_weight: f32, next_animation: Option<u16>) -> Animation {
    Animation {
        name: String::new(),
        kind: AnimationKind::Base,
        frame_count: 1,
        frame_size: 0,
        frame_info_type: 0,
        node_list_checksum: 0,
        node_count: 0,
        loop_frame: 0,
        weight: 1.0,
        key_frame: -1,
        second_key_frame: -1,
        next_animation,
        parent_animation: None,
        normalized_weight,
        flags: 0,
        sound: None,
        sound_frame: -1,
        left_foot_frame: -1,
        right_foot_frame: -1,
        frame_info: Vec::new(),
        rotation_flags: [0, 0],
        translation_flags: [0, 0],
        scale_flags: [0, 0],
        compressed_data_offset: 0,
        default_data: Vec::new(),
        data: Vec::new(),
    }
}

#[test]
fn a_permutation_is_chosen_as_the_games() -> Result<(), Failure> {
    let animations = lines("animation")
        .map(|fields| {
            Ok(animation(
                fields.get(1).ok_or("a short line")?.parse()?,
                index(&fields, 2)?,
            ))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    assert_eq!(animations.len(), 10);
    let mut compared = 0;
    for fields in lines("choose") {
        let first = index(&fields, 0)?;
        let random: f32 = fields.get(1).ok_or("a short line")?.parse()?;
        assert_eq!(
            choose_permutation(&animations, first, random),
            index(&fields, 2)?,
            "first {first:?} at {random}"
        );
        compared += 1;
    }
    assert_eq!(compared, 13 * 21);
    Ok(())
}
