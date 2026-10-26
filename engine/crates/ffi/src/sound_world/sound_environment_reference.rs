//! The sound around the camera is the game's:
//! tools/engine_sound_environment_reference.py runs
//! structure_bsp_definitions.c's `structure_bsp_get_cluster_encoded_sound_distance`
//! over five BSPs (one with fewer distances than its pairs) for each pair of
//! clusters and past the last, and scenario.c's `scenario_get_sound_environment`
//! for a camera walking twelve clusters for 600 frames, whose palettes have
//! environments of any priority, entries without a tag and indices past
//! their ends; the engine's distances, room lookups and glide are asked the
//! same and compared bit for bit.

use opence_audio::Environment;
use opence_content::{ClusterSound, Group, SoundEnvironment, StructureBsp, TagRef};

use super::room_sound;

const REFERENCE: &str = include_str!("../../tests/sound_environment_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

fn integer(fields: &[&str], at: usize) -> Result<i32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn reals<const N: usize>(fields: &[&str], from: usize) -> Result<[f32; N], Failure> {
    let mut values = [0.0; N];
    for (offset, value) in values.iter_mut().enumerate() {
        *value = fields.get(from + offset).ok_or("a short line")?.parse()?;
    }
    Ok(values)
}

/// An index of the game's (a short, `NONE` or below for none), as the
/// engine imports it.
fn index(fields: &[&str], at: usize) -> Result<Option<u16>, Failure> {
    Ok(u16::try_from(integer(fields, at)?).ok())
}

/// A background sound the trace names by its tag index.
fn background(index: i32) -> TagRef {
    TagRef {
        group: Group(*b"lsnd"),
        path: index.to_string(),
    }
}

/// An empty BSP of `clusters` clusters.
fn bsp(clusters: usize) -> StructureBsp {
    let mut bsp = opence_content::devmap::level(None).structure_bsps.remove(0);
    bsp.cluster_skies.clear();
    bsp.cluster_sounds = vec![
        ClusterSound {
            background: None,
            environment: None,
        };
        clusters
    ];
    bsp.cluster_sound_distances.clear();
    bsp.sound_environments.clear();
    bsp.background_sounds.clear();
    bsp
}

#[test]
fn the_distance_between_two_clusters_is_the_games() -> Result<(), Failure> {
    let bsps = lines("bsp")
        .map(|fields| {
            let mut bsp = bsp(usize::try_from(integer(&fields, 1)?)?);
            bsp.cluster_sound_distances = fields
                .get(3..)
                .ok_or("a short line")?
                .iter()
                .map(|byte| byte.parse())
                .collect::<Result<_, _>>()?;
            assert_eq!(
                bsp.cluster_sound_distances.len(),
                usize::try_from(integer(&fields, 2)?)?
            );
            Ok(bsp)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let mut compared = 0;
    for fields in lines("distance") {
        let bsp = bsps
            .get(usize::try_from(integer(&fields, 0)?)?)
            .ok_or("no such bsp")?;
        let (from, to) = (
            u16::try_from(integer(&fields, 1)?)?,
            u16::try_from(integer(&fields, 2)?)?,
        );
        let [expected] = reals::<1>(&fields, 4)?;
        assert_eq!(
            bsp.cluster_sound_distance(from, to).to_bits(),
            expected.to_bits(),
            "{fields:?}"
        );
        compared += 1;
    }
    assert_eq!(compared, 2049);
    Ok(())
}

#[test]
fn the_cameras_environment_and_background_are_the_games() -> Result<(), Failure> {
    let mut environments = Vec::new();
    for fields in lines("environment") {
        environments.push(SoundEnvironment {
            priority: i16::try_from(integer(&fields, 1)?)?,
            values: reals::<12>(&fields, 2)?,
        });
    }
    let mut structure = bsp(0);
    structure.sound_environments = lines("environment_palette")
        .map(|fields| {
            Ok(index(&fields, 1)?
                .map(|at| {
                    environments
                        .get(usize::from(at))
                        .copied()
                        .ok_or("no such environment")
                })
                .transpose()?)
        })
        .collect::<Result<_, Failure>>()?;
    structure.background_sounds = lines("background_palette")
        .map(|fields| {
            let tag = integer(&fields, 1)?;
            Ok((tag >= 0).then(|| background(tag)))
        })
        .collect::<Result<_, Failure>>()?;
    structure.cluster_sounds = lines("cluster")
        .map(|fields| {
            Ok(ClusterSound {
                environment: index(&fields, 1)?,
                background: index(&fields, 2)?,
            })
        })
        .collect::<Result<_, Failure>>()?;

    let mut heard = Environment::DEFAULT;
    let (mut frames, mut backgrounds) = (0, 0);
    for fields in lines("frame") {
        let (target, playing) = room_sound(Some(&structure), index(&fields, 1)?);
        // (the game glides once a frame, the engine once a tick: at 30
        // frames a second they are the same)
        heard.glide_towards(&target);
        assert_eq!(integer(&fields, 2)?, 0, "the scenario has no water");
        let expected = integer(&fields, 3)?;
        assert_eq!(
            playing,
            (expected >= 0).then(|| background(expected)),
            "{fields:?}"
        );
        backgrounds += usize::from(playing.is_some());
        assert_eq!(
            heard.values().map(f32::to_bits),
            reals::<12>(&fields, 4)?.map(f32::to_bits),
            "frame {fields:?}"
        );
        frames += 1;
    }
    assert_eq!((frames, backgrounds), (600, 160));
    Ok(())
}
