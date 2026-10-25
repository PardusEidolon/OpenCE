//! The engine's sprites face as the game's do: their axes and fade are
//! those of the game's own `build_sprite_compute_basis` and
//! `build_sprite_compute_vertex_fade`, recorded by
//! tools/engine_sprite_reference.py over cameras, sprites and directions
//! (straight up and down, nearly so, none and too short among them).

use opence_ffi::sprite::{basis, fade, Orientation};
use opence_sim::Vec3;

const REFERENCE: &str = include_str!("sprite_reference.txt");

/// A recorded case.
struct Case {
    orientation: Orientation,
    eye: Vec3,
    at: Vec3,
    direction: Vec3,
    x: Vec3,
    y: Vec3,
    fades: [f32; 3],
}

fn vector(words: &[&str], name: &str) -> Vec3 {
    let at = words.iter().position(|word| *word == name).unwrap_or(0) + 1;
    let component = |index: usize| words[at + index].parse::<f32>().unwrap_or(f32::NAN);
    Vec3::new(component(0), component(1), component(2))
}

fn cases() -> Vec<Case> {
    REFERENCE
        .lines()
        .filter(|line| line.starts_with("sprite "))
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let orientation = match words[1] {
                "0" => Orientation::Screen,
                "1" => Orientation::Parallel,
                _ => Orientation::Perpendicular,
            };
            let at = words.iter().position(|word| *word == "fade").unwrap_or(0) + 1;
            let fades = [0, 1, 2].map(|index| words[at + index].parse().unwrap_or(f32::NAN));
            Case {
                orientation,
                eye: vector(&words, "eye"),
                at: vector(&words, "at"),
                direction: vector(&words, "direction"),
                x: vector(&words, "x"),
                y: vector(&words, "y"),
                fades,
            }
        })
        .collect()
}

#[test]
fn the_reference_covers_every_orientation() {
    let cases = cases();
    assert!(cases.len() > 500, "{} cases", cases.len());
    for orientation in [
        Orientation::Screen,
        Orientation::Parallel,
        Orientation::Perpendicular,
    ] {
        assert!(cases.iter().any(|case| case.orientation == orientation));
    }
}

#[test]
fn sprites_lie_along_the_games_axes() {
    let (mut worst, mut compared) = (0.0f32, 0);
    for case in cases() {
        // (facing the screen, the game's axes are the camera's, which the
        // shader takes from the view; only the others are the sprite's own)
        if case.orientation == Orientation::Screen {
            continue;
        }
        let (x, y) = basis(case.orientation, case.eye, case.at, case.direction);
        let error = x.distance(case.x).max(y.distance(case.y));
        assert!(
            error < 1e-4,
            "{:?} at {} from {} along {}: {x} {y}, the game's {} {}",
            case.orientation,
            case.at,
            case.eye,
            case.direction,
            case.x,
            case.y
        );
        worst = worst.max(error);
        compared += 1;
    }
    assert!(compared > 300, "{compared} compared");
    println!("{compared} sprites, the worst {worst:e} from the game's axes");
}

#[test]
fn sprites_fade_as_the_games_turn_edge_on() {
    for case in cases() {
        let normal = case.x.cross(case.y);
        for (mode, expected) in case.fades.into_iter().enumerate() {
            let fade = fade(mode as u8, case.eye, case.at, normal);
            assert!(
                (fade - expected).abs() < 1e-4,
                "{:?} fade mode {mode} at {} from {}: {fade}, the game's {expected}",
                case.orientation,
                case.at,
                case.eye
            );
        }
    }
}
