//! A contrail's fade is the game's: tools/engine_render_size_reference.py
//! runs render_contrails.c's `contrail_fade`, with the game's transition
//! tables, over strips at points facing each way, under each fade mode
//! with and without the slow fade, and the engine's fade is asked the same
//! and compared.

use opence_content::{Contrail, Group, SpriteShader, TagRef};
use opence_sim::Vec3;

use super::fade;

const REFERENCE: &str = include_str!("../../../tests/render_size_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn contrail(flags: u16, fade_mode: i16) -> Contrail {
    Contrail {
        tag: TagRef {
            group: Group(*b"cont"),
            path: String::new(),
        },
        flags,
        scale_flags: 0,
        point_generation_rate: 0.0,
        velocity: [0.0, 0.0],
        velocity_cone_angle: 0.0,
        inherited_velocity_fraction: 0.0,
        render_type: 0,
        texture_repeats: [1.0, 1.0],
        texture_animation: [0.0, 0.0],
        frames_per_second: 0.0,
        bitmap: None,
        sequences: [0, 1],
        shader: SpriteShader {
            fade_mode,
            ..SpriteShader::default()
        },
        states: Vec::new(),
    }
}

#[test]
fn a_contrails_fade_is_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for line in REFERENCE.lines().filter(|line| line.starts_with("fade ")) {
        let fields = line.split_whitespace().skip(1).collect::<Vec<_>>();
        let (Some(flags), Some(mode)) = (fields.first(), fields.get(1)) else {
            return Err(format!("a short line: {line}").into());
        };
        let values = fields[2..]
            .iter()
            .map(|field| field.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()?;
        let [px, py, pz, ni, nj, nk, cx, cy, cz, expected] = values[..] else {
            return Err(format!("a short line: {line}").into());
        };
        let actual = fade(
            &contrail(flags.parse()?, mode.parse()?),
            Vec3::new(px, py, pz),
            Vec3::new(ni, nj, nk),
            Vec3::new(cx, cy, cz),
        );
        assert_eq!(actual.to_bits(), expected.to_bits(), "{line}");
        compared += 1;
    }
    assert!(compared > 2000, "{compared}");
    Ok(())
}
