//! What a sound plays (its pitch range, its permutation and its scaled
//! values) against the game's own choice over scripted plays
//! (tools/engine_sound_choice_reference.py writes the table from
//! source/sound/sound_definitions.c and sound_manager.c).

use opence_audio::choice::{
    next_permutation, pitch_range, scale_random_value, LocalRandom, RangeState,
};
use opence_content::{
    Group, Sound, SoundCompression, SoundModifiers, SoundPermutation, SoundPitchRange, TagRef,
};

const REFERENCE: &str = include_str!("../../../../tools/traces/sound_choice_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// The fields after a line's first word, of the lines whose first word is
/// `kind`.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&'static str>> + '_ {
    REFERENCE.lines().filter_map(move |line| {
        let mut fields = line.split_whitespace();
        (fields.next() == Some(kind)).then(|| fields.collect())
    })
}

/// A permutation, as the game's script reads it: an index of -1 is none.
fn index(field: &str) -> Result<Option<u16>, Failure> {
    let index: i32 = field.parse()?;
    Ok(u16::try_from(index).ok())
}

/// A range of `count` permutations, the first `actual` of which play, each
/// skipped by its fraction (the fractions repeat) and linked to its next,
/// as the script makes them.
fn range(
    bounds: [f32; 2],
    count: usize,
    actual: u16,
    skips: &[f32],
    nexts: &[Option<u16>],
) -> SoundPitchRange {
    SoundPitchRange {
        name: String::new(),
        natural_pitch: 1.0,
        bend_bounds: bounds,
        actual_permutation_count: actual,
        playback_rate: 1.0,
        permutations: (0..count)
            .map(|permutation| SoundPermutation {
                name: String::new(),
                skip_fraction: if skips.is_empty() {
                    0.0
                } else {
                    skips[permutation % skips.len()]
                },
                gain: 1.0,
                compression: SoundCompression::XboxAdpcm,
                next: nexts.get(permutation).copied().flatten(),
                samples: Vec::new(),
                mouth: Vec::new(),
            })
            .collect(),
    }
}

fn sound(flags: u32, pitch_ranges: Vec<SoundPitchRange>) -> Sound {
    Sound {
        tag: TagRef {
            group: Group(*b"snd!"),
            path: String::new(),
        },
        flags,
        class: 0,
        sample_rate: 22050,
        channels: 1,
        distance: [1.0, 10.0],
        skip_fraction: 0.0,
        random_pitch_bounds: [1.0, 1.0],
        cone_angles: [0.0, 0.0],
        outer_cone_gain: 1.0,
        gain_modifier: 1.0,
        maximum_bend_per_second: 0.0,
        zero: SoundModifiers::default(),
        one: SoundModifiers::default(),
        promotion: None,
        promotion_count: 0,
        longest_permutation_ms: 0,
        pitch_ranges,
    }
}

#[test]
fn the_pitch_ranges_are_the_games() -> Result<(), Failure> {
    let sound = sound(
        0,
        vec![
            range([0.5, 0.9], 5, 5, &[], &[]),
            range([0.9, 1.1], 0, 0, &[], &[]),
            range([1.2, 2.0], 3, 3, &[], &[]),
            range([2.5, 4.0], 2, 2, &[], &[]),
        ],
    );
    let mut compared = 0;
    for fields in lines("pitch") {
        let [pitch, old, expected] = fields[..] else {
            return Err(format!("a pitch line of {} fields", fields.len()).into());
        };
        let pitch: f32 = pitch.parse()?;
        let old = index(old)?.map(usize::from);
        let expected = index(expected)?.map(usize::from);
        assert_eq!(
            pitch_range(&sound, pitch, old),
            expected,
            "the pitch {pitch} from the range {old:?}"
        );
        compared += 1;
    }
    assert_eq!(compared, 600);
    Ok(())
}

/// Plays a range as the script does, each permutation, played mask and
/// seed against the game's.
fn plays(
    kind: &str,
    sound: &Sound,
    range: usize,
    state: &mut RangeState,
    random: &mut LocalRandom,
) -> Result<usize, Failure> {
    let mut count = 0;
    for fields in lines(kind) {
        let [chosen, mask, seed] = fields[..] else {
            return Err(format!("a {kind} line of {} fields", fields.len()).into());
        };
        let got = next_permutation(sound, range, state, None, random);
        assert_eq!(got, index(chosen)?, "{kind} play {count}");
        assert_eq!(
            state.played,
            mask.parse::<u32>()?,
            "{kind} play {count}'s mask"
        );
        assert_eq!(random.0, seed.parse::<u32>()?, "{kind} play {count}'s seed");
        count += 1;
    }
    Ok(count)
}

#[test]
fn the_permutations_are_the_games() -> Result<(), Failure> {
    let skips = [0.0, 0.3, 0.9, 0.0, 1.0, 0.5, 0.0];
    let sound_of_ranges = sound(
        0,
        vec![
            range([0.0, 2.0], 7, 6, &skips, &[]),
            range([0.0, 2.0], 32, 32, &[0.25, 0.75], &[]),
            range([0.0, 2.0], 1, 1, &[], &[]),
            range([0.0, 2.0], 4, 4, &[1.0], &[]),
        ],
    );
    let mut states = [RangeState::default(); 4];
    let mut random = LocalRandom(0x1234);
    for (kind, range, count) in [
        ("six", 0, 400),
        ("all", 1, 200),
        ("lone", 2, 20),
        ("skipped", 3, 40),
    ] {
        let played = plays(
            kind,
            &sound_of_ranges,
            range,
            &mut states[range],
            &mut random,
        )?;
        assert_eq!(played, count, "the {kind} plays");
    }
    states[0].forced = Some(4);
    assert_eq!(
        plays("forced", &sound_of_ranges, 0, &mut states[0], &mut random)?,
        8
    );

    let linked = sound(
        Sound::LINKED_PERMUTATIONS,
        vec![range(
            [0.0, 2.0],
            4,
            4,
            &[],
            &[Some(1), Some(2), None, Some(9)],
        )],
    );
    let mut state = RangeState::default();
    let mut count = 0;
    for fields in lines("linked") {
        let [current, expected, seed] = fields[..] else {
            return Err(format!("a linked line of {} fields", fields.len()).into());
        };
        let current = index(current)?;
        let got = next_permutation(&linked, 0, &mut state, current, &mut random);
        assert_eq!(got, index(expected)?, "linked from {current:?}");
        assert_eq!(
            random.0,
            seed.parse::<u32>()?,
            "the seed after linked from {current:?}"
        );
        count += 1;
    }
    assert_eq!(count, 6);
    Ok(())
}

#[test]
fn the_scaled_values_are_the_games() -> Result<(), Failure> {
    let mut random = LocalRandom(0xDEAD_BEEF);
    let mut count = 0;
    for fields in lines("scale") {
        let values: Vec<f32> = fields
            .iter()
            .map(|field| field.parse::<f32>())
            .collect::<Result<_, _>>()?;
        let [lower, upper, zero, one, scale, expected] = values[..] else {
            return Err(format!("a scale line of {} fields", values.len()).into());
        };
        let got = scale_random_value(&mut random, [lower, upper], zero, one, scale);
        assert!(
            (got - expected).abs() <= 1e-6,
            "the scale {lower} {upper} {zero} {one} {scale}: {got}, the game's {expected}"
        );
        count += 1;
    }
    assert_eq!(count, 40);
    Ok(())
}
