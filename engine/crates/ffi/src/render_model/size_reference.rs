//! A sphere's size on screen is the game's:
//! tools/engine_render_size_reference.py runs render_cameras.c's
//! `render_frustum_sphere_diameter_in_pixels` over frustums of six fields
//! of view and five heights, at spheres before, behind and at the eye, and
//! the engine's size is asked the same and compared.

use super::sphere_diameter_pixels;

const REFERENCE: &str = include_str!("../../tests/render_size_reference.txt");

type Failure = Box<dyn std::error::Error>;

#[test]
fn a_spheres_size_on_screen_is_the_games() -> Result<(), Failure> {
    let mut compared = 0;
    for line in REFERENCE.lines().filter(|line| line.starts_with("sphere ")) {
        let values = line
            .split_whitespace()
            .skip(1)
            .map(str::parse::<f32>)
            .collect::<Result<Vec<_>, _>>()?;
        let [fov, height, depth, radius, _, pixels] = values[..] else {
            return Err(format!("a short line: {line}").into());
        };
        assert_eq!(
            sphere_diameter_pixels(depth, radius, fov, height).to_bits(),
            pixels.to_bits(),
            "{line}"
        );
        compared += 1;
    }
    assert_eq!(compared, 6 * 5 * 20);
    Ok(())
}
