//! How blocked a sound is is the game's:
//! tools/engine_sound_obstruction_reference.py runs game_sound.c's
//! `compute_sound_obstruction` (with structure_bsp_definitions.c's
//! `structure_bsp_get_cluster_pvs` and sound distances) over two BSPs of
//! twenty clusters, one whose visible sets stop at its thirteenth, for 3000
//! sources at any distance, in any cluster or none and behind a wall or
//! not; the engine's obstruction is asked the same, and whether it tests the
//! line, and compared bit for bit.

use opence_content::{ClusterSound, StructureBsp};

use super::obstruction;

const REFERENCE: &str = include_str!("../../tests/sound_obstruction_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

fn integer(fields: &[&str], at: usize) -> Result<i64, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn real(fields: &[&str], at: usize) -> Result<f32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

/// A cluster of the game's (a short, `NONE` for none), as the engine
/// finds it.
fn cluster(fields: &[&str], at: usize) -> Result<Option<u16>, Failure> {
    Ok(u16::try_from(integer(fields, at)?).ok())
}

/// The trace's BSPs: their clusters, sound distances and visible sets.
fn bsps() -> Result<Vec<StructureBsp>, Failure> {
    let mut bsps = lines("bsp")
        .map(|fields| {
            let mut bsp = opence_content::devmap::level(None).structure_bsps.remove(0);
            bsp.cluster_skies.clear();
            bsp.cluster_sounds = vec![
                ClusterSound {
                    background: None,
                    environment: None,
                };
                usize::try_from(integer(&fields, 1)?)?
            ];
            bsp.cluster_sound_distances = fields
                .get(2..)
                .ok_or("a short line")?
                .iter()
                .map(|byte| byte.parse())
                .collect::<Result<_, _>>()?;
            bsp.cluster_pvs.clear();
            Ok(bsp)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    for fields in lines("visible") {
        let bsp = bsps
            .get_mut(usize::try_from(integer(&fields, 0)?)?)
            .ok_or("no such bsp")?;
        // (one word a row: the trace's BSPs have twenty clusters)
        assert_eq!(
            usize::try_from(integer(&fields, 1)?)?,
            bsp.cluster_pvs.len()
        );
        bsp.cluster_pvs.push(u32::try_from(integer(&fields, 2)?)?);
    }
    Ok(bsps)
}

#[test]
fn a_sounds_obstruction_is_the_games() -> Result<(), Failure> {
    let bsps = bsps()?;
    let (mut compared, mut tested) = (0, 0);
    for fields in lines("trial") {
        let bsp = bsps
            .get(usize::try_from(integer(&fields, 1)?)?)
            .ok_or("no such bsp")?;
        let paths = match (cluster(&fields, 2)?, cluster(&fields, 3)?) {
            (Some(from), Some(to)) => Some((
                bsp.cluster_sound_distance(from, to),
                bsp.cluster_in_view(from, to),
            )),
            _ => None,
        };
        let blocked = integer(&fields, 5)? != 0;
        let mut asked = false;
        let actual = obstruction(
            paths,
            || {
                asked = true;
                !blocked
            },
            real(&fields, 4)?,
        );
        let expected = (real(&fields, 7)?, real(&fields, 8)?);
        assert_eq!(
            (actual.0.to_bits(), actual.1.to_bits()),
            (expected.0.to_bits(), expected.1.to_bits()),
            "trial {fields:?}"
        );
        assert_eq!(asked, integer(&fields, 6)? != 0, "trial {fields:?}'s line");
        compared += 1;
        tested += usize::from(asked);
    }
    assert_eq!((compared, tested), (3000, 1050));
    Ok(())
}
