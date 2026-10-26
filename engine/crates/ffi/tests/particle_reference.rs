//! The engine's particle life against the game's: the particles of
//! tools/engine_particle_reference.py (`particle_new` and `particles_update`
//! with `particle_next_sequence`, `particle_next_frame` and
//! `particle_update_frame_time`, over hundreds of definitions with every
//! mix of the life flags, lifespans, frame rates, sequence counts, sprite
//! counts and local players) are lived again through `Life`, from the same
//! local seed, and at its birth and after every update a particle's
//! sequence, frame, age, frame time, stage and radius must be the game's,
//! as must the update it dies on.

use std::error::Error;

use opence_content::particle::Particle;
use opence_content::{Group, TagRef};
use opence_ffi::particle_life::{Clock, Life, Stage};
use opence_sim::random::RandomSeed;

const REFERENCE: &str = include_str!("particle_reference.txt");

type Failure = Box<dyn Error>;

fn field<T: std::str::FromStr>(fields: &[&str], at: usize) -> Result<T, Failure>
where
    T::Err: Error + 'static,
{
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

/// The same number, or both not one (a particle that lives no time is
/// no age through its life).
fn assert_same(engine: f32, field: f32, what: &str, line: &str) {
    assert!(
        engine == field || engine.is_nan() && field.is_nan(),
        "{what}: {engine} for {field}: {line}"
    );
}

fn stage(stage: Stage) -> i32 {
    match stage {
        Stage::Initial => 0,
        Stage::Looping => 1,
        Stage::StillLooping => 2,
        Stage::Final => 3,
        Stage::Done => 4,
    }
}

/// The particle's state as the trace prints it, from `at` on.
fn assert_particle(
    life: &Life,
    particle: &Particle,
    fields: &[&str],
    at: usize,
    line: &str,
) -> Result<(), Failure> {
    assert_eq!(
        stage(life.stage),
        field::<i32>(fields, at)?,
        "stage: {line}"
    );
    assert_eq!(
        life.sequence,
        field::<i32>(fields, at + 1)?,
        "sequence: {line}"
    );
    assert_eq!(life.frame, field::<i32>(fields, at + 2)?, "frame: {line}");
    assert_eq!(life.age, field::<f32>(fields, at + 3)?, "age: {line}");
    assert_eq!(
        life.frame_time,
        field::<f32>(fields, at + 4)?,
        "frame time: {line}"
    );
    assert_same(
        life.radius(particle),
        field(fields, at + 7)?,
        "radius",
        line,
    );
    Ok(())
}

#[test]
fn particles_live_and_die_as_the_games_do() -> Result<(), Failure> {
    // the game's local seed, as the reference leaves it: zero
    let mut random = RandomSeed(0);
    let mut clock = Clock::default();
    let mut particle = Particle::new(TagRef {
        group: Group(*b"part"),
        path: "reference".into(),
    });
    let (mut sprites, mut players, mut dt, mut has_effect) = (Vec::new(), 1, 0.0, false);
    let mut life: Option<Life> = None;
    let (mut cases, mut updates, mut deaths) = (0, 0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first().copied() {
            Some("case") => {
                particle.flags = field(&fields, 1)?;
                particle.lifespan = [field(&fields, 2)?, field(&fields, 3)?];
                particle.fps = [field(&fields, 4)?, field(&fields, 5)?];
                particle.radius = [field(&fields, 6)?, field(&fields, 7)?];
                particle.first_sequence = field(&fields, 8)?;
                particle.initial_sequences = field(&fields, 9)?;
                particle.looping_sequences = field(&fields, 10)?;
                particle.final_sequences = field(&fields, 11)?;
                has_effect = field::<i32>(&fields, 12)? != 0;
                players = field(&fields, 13)?;
                dt = field(&fields, 14)?;
                let from = fields
                    .iter()
                    .position(|&f| f == "sprites")
                    .ok_or("no sprites")?;
                sprites = fields[from + 1..]
                    .iter()
                    .map(|count| count.parse())
                    .collect::<Result<_, _>>()?;
                cases += 1;
            }
            Some("born") if fields.get(1) == Some(&"dead") => {
                let born = Life::new(&particle, &sprites, players, &mut random);
                assert!(!born.alive(), "case {cases}: {line}");
                assert_eq!(field::<i32>(&fields, 2)? != 0, has_effect, "{line}");
                life = None;
            }
            Some("born") => {
                let born = Life::new(&particle, &sprites, players, &mut random);
                assert!(born.alive(), "case {cases}: {line}");
                let flags: u32 = field(&fields, 1)?;
                assert_eq!(born.backwards, flags & 1 != 0, "backwards: {line}");
                assert_eq!(
                    born.mirrors,
                    [flags & 4 != 0, flags & 8 != 0],
                    "mirrors: {line}"
                );
                assert_eq!(born.lifespan, field::<f32>(&fields, 4)?, "lifespan: {line}");
                assert_eq!(
                    born.frame_span,
                    field::<f32>(&fields, 5)?,
                    "frame span: {line}"
                );
                assert_particle(&born, &particle, &fields, 6, line)?;
                life = Some(born);
            }
            Some("update") => {
                let living = life.as_mut().ok_or("an update of no particle")?;
                let ticks = clock.ticks(dt);
                assert_eq!(ticks, field::<i32>(&fields, 1)?, "ticks: {line}");
                let at_rest = field::<i32>(&fields, 2)? != 0;
                assert!(
                    living.update(&particle, &sprites, (dt, ticks), at_rest, &mut random),
                    "case {cases}: it died: {line}"
                );
                assert_particle(living, &particle, &fields, 4, line)?;
                updates += 1;
            }
            Some("dies") => {
                let mut dying = life.take().ok_or("a death of no particle")?;
                let ticks = clock.ticks(dt);
                let at_rest = field::<i32>(&fields, 4)? != 0;
                assert!(
                    !dying.update(&particle, &sprites, (dt, ticks), at_rest, &mut random),
                    "case {cases}: it lived: {line}"
                );
                assert_eq!(dying.age, field::<f32>(&fields, 3)?, "age: {line}");
                assert_eq!(field::<i32>(&fields, 2)? != 0, has_effect, "{line}");
                deaths += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(cases, 400);
    assert!(updates >= 10_000, "{updates} updates");
    assert!(deaths >= 250, "{deaths} deaths");
    Ok(())
}
