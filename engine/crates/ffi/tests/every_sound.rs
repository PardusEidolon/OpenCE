//! Every sound Blood Gulch's tags refer to, against what a sweep of play
//! asks for: each weapon fired at the ground and at the bases, reloaded,
//! meleed and swapped; grenades; walking, jumping and landing; each kind
//! of vehicle entered, driven, fired and left; the powerups picked up;
//! dying; and a wait for the ambience. Prints the sounds no event asked
//! for, by what refers to them, and fails on any path the engine asked
//! for that the level has no sound of (needs assets/maps; passes without
//! it).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use opence_ffi::{Engine, InputState, LevelGeometry};
use opence_import::{Cache, GroupTag};
use opence_sim::input::Buttons;
use serde_json::Value;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

const SOUND: GroupTag = GroupTag::new(b"snd!");
const LOOPING: GroupTag = GroupTag::new(b"lsnd");
const BSP: GroupTag = GroupTag::new(b"sbsp");

fn run(engine: &Engine, line: &str) -> Result<Value> {
    let output = engine.run_command(line.into());
    if !output.ok {
        return Err(format!("{line}: {}", output.text).into());
    }
    Ok(serde_json::from_str(&output.json)?)
}

fn still(buttons: u32) -> InputState {
    InputState {
        buttons,
        forward: 0,
        right: 0,
        up: 0,
        yaw: 0.0,
        pitch: 0.0,
    }
}

/// Holds `input` for `frames` frames of a thirtieth, mixing as the app
/// does so that voices end.
fn play(engine: &Engine, input: InputState, frames: u32) -> Result {
    let output = engine.audio_output();
    for _ in 0..frames {
        engine.set_input(input);
        engine.advance(1.0 / 30.0)?;
        output.pull(output.sample_rate() / 30);
    }
    engine.set_input(still(0));
    Ok(())
}

/// Presses `buttons` for a few frames, then lets go for `after`.
fn press(engine: &Engine, buttons: Buttons, after: u32) -> Result {
    play(engine, still(buttons.0), 3)?;
    play(engine, still(0), after)
}

fn rows(engine: &Engine, filter: &str) -> Result<Vec<Value>> {
    Ok(run(engine, &format!("objects {filter}"))?
        .as_array()
        .cloned()
        .unwrap_or_default())
}

fn id(row: &Value) -> Result<i64> {
    row["id"]
        .as_i64()
        .ok_or_else(|| format!("{row} has no id").into())
}

/// The map's tags that refer to each sound and looping sound, by path:
/// every reference of those groups in the tag data, credited to the tag
/// whose definition it is in (the one at or before it).
fn referrers(map: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let cache = Cache::open(std::fs::read(map)?)?;
    let tags = cache.tags();
    let base = cache.header().kind.tag_base();
    let mut starts: Vec<(u32, usize)> = tags
        .iter()
        .enumerate()
        // (the BSP's address is the structure's, not the tag data's)
        .filter(|(_, tag)| !tag.in_resource_map && tag.group != BSP)
        .map(|(index, tag)| (tag.address, index))
        .collect();
    starts.sort_unstable();
    let bytes = cache.tag_data();
    let word = |at: usize| {
        bytes
            .get(at..at + 4)
            .and_then(|word| word.try_into().ok())
            .map(u32::from_le_bytes)
    };
    let mut referred: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for at in (0..bytes.len().saturating_sub(16)).step_by(4) {
        let Some(group) = word(at).map(GroupTag) else {
            continue;
        };
        if group != SOUND && group != LOOPING {
            continue;
        }
        let Some(sound) = word(at + 12)
            .and_then(|handle| cache.handle_index(handle))
            .and_then(|index| tags.get(index))
            .filter(|sound| sound.group == group)
        else {
            continue;
        };
        let address = base.wrapping_add(u32::try_from(at)?);
        let by = starts
            .partition_point(|(start, _)| *start <= address)
            .checked_sub(1)
            .and_then(|slot| starts.get(slot))
            .and_then(|(_, index)| tags.get(*index))
            .map_or_else(
                || "?".to_owned(),
                |tag| format!("{} {}", tag.group, tag.name),
            );
        referred
            .entry(sound.name.to_lowercase())
            .or_default()
            .insert(by);
    }
    // (and the sounds no tag data refers to: the BSP's, outside it)
    for tag in tags
        .iter()
        .filter(|tag| tag.group == SOUND || tag.group == LOOPING)
    {
        referred.entry(tag.name.to_lowercase()).or_default();
    }
    Ok(referred)
}

/// Blood Gulch, alone, the player at the first start, and the map; none
/// without game data.
fn blood_gulch() -> Result<Option<(Arc<Engine>, std::path::PathBuf)>> {
    let map = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps/bloodgulch.map");
    if !map.exists() {
        return Ok(None);
    }
    let cache = Path::new(env!("CARGO_TARGET_TMPDIR")).join("every_sound");
    std::fs::create_dir_all(&cache)?;
    let geometry = LevelGeometry::load(
        map.to_string_lossy().into(),
        cache.to_string_lossy().into(),
        0,
    )?;
    let engine = Engine::new();
    engine.start_level(geometry, 1)?;
    engine.spawn_player(0)?;
    Ok(Some((engine, map)))
}

/// Fires the weapon in hand at the ground, turning, then reloads, melees
/// the ground and swaps.
fn handle_weapon(engine: &Engine) -> Result {
    play(
        engine,
        InputState {
            pitch: -0.5,
            ..still(0)
        },
        1,
    )?;
    play(
        engine,
        InputState {
            yaw: 0.05,
            ..still(Buttons::FIRE.0)
        },
        60,
    )?;
    play(engine, still(0), 30)?;
    press(engine, Buttons::RELOAD, 90)?;
    // (the melee at the ground at the feet, which it reaches)
    let down = |pitch: f32| InputState { pitch, ..still(0) };
    play(engine, down(-0.9), 1)?;
    press(engine, Buttons::MELEE, 30)?;
    play(engine, down(0.9), 1)?;
    press(engine, Buttons::ZOOM, 15)?;
    press(engine, Buttons::ZOOM, 15)?;
    press(engine, Buttons::SWAP, 45)?;
    press(engine, Buttons::SWAP, 45)?;
    play(
        engine,
        InputState {
            pitch: 0.5,
            ..still(0)
        },
        1,
    )
}

/// A sound of each kind of event the sweep makes: a shot, an impact by
/// its material, a casing, a footstep and a landing, a grenade's throw and
/// detonation, a pickup, a vehicle's entry and engine, a melee and the
/// ambience.
const EVENT_SOUNDS: &[&str] = &[
    "sound\\sfx\\weapons\\assault rifle\\fire",
    "sound\\sfx\\impulse\\impacts\\dirthits",
    "sound\\sfx\\impulse\\melee\\assault_impact",
    "sound\\sfx\\impulse\\melee\\melee_dirt",
    "sound\\sfx\\impulse\\shellcasings\\single_shell_dirt",
    "sound\\sfx\\impulse\\footsteps\\cyborg\\gravel\\run",
    "sound\\sfx\\impulse\\footsteps\\cyborg\\gravel\\jump",
    "sound\\sfx\\weapons\\frag grenade\\throwgren",
    "sound\\sfx\\weapons\\frag grenade\\expl",
    "sound\\sfx\\weapons\\weapon_pickup_ammo\\grenade_ammo",
    "sound\\sfx\\vehicles\\warthog_7_in",
    "sound\\sfx\\vehicles\\ghost_in",
    "sound\\sfx\\vehicles\\warthog_engine",
    "sound\\sfx\\weapons\\weapon_anims\\ar_melee",
    "sound\\sfx\\ambience\\multiplayer\\countryday2",
];

/// What the sweep asked for: each sound the level has that played,
/// culled or failed, and the paths it has no sound of.
fn sweep(engine: &Engine) -> Result {
    // (the ambience, before anything else)
    play(engine, still(0), 300)?;

    // (on foot)
    play(
        engine,
        InputState {
            forward: 127,
            ..still(0)
        },
        90,
    )?;
    press(engine, Buttons::JUMP, 60)?;
    press(engine, Buttons::CROUCH, 15)?;
    press(engine, Buttons::FLASHLIGHT, 15)?;
    press(engine, Buttons::FLASHLIGHT, 15)?;

    // (each weapon of the level's)
    let weapons: BTreeSet<String> = run(engine, "tags")?
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter(|tag| tag["kind"] == "weapon")
        .filter_map(|tag| tag["path"].as_str().map(str::to_owned))
        .collect();
    for weapon in &weapons {
        // (single-quoted, so that the shell keeps the path's backslashes)
        match run(engine, &format!("give '{weapon}'")) {
            Ok(_) => {
                play(engine, still(0), 45)?;
                handle_weapon(engine)?;
            }
            Err(error) => println!("not given: {error}"),
        }
    }

    // (grenades, of each kind)
    for _ in 0..2 {
        press(engine, Buttons::GRENADE, 150)?;
        press(engine, Buttons::SWITCH_GRENADE, 15)?;
    }

    // (shots at the bases and the scenery: metal, rock, wood)
    for row in rows(engine, "scenery")?.iter().take(12) {
        run(engine, &format!("teleport {}", id(row)?))?;
        play(engine, still(0), 10)?;
        play(
            engine,
            InputState {
                yaw: 0.2,
                ..still(Buttons::FIRE.0)
            },
            30,
        )?;
        press(engine, Buttons::RELOAD, 60)?;
    }

    // (the powerups and equipment, picked up)
    for row in rows(engine, "equipment")? {
        if run(engine, &format!("teleport {}", id(&row)?)).is_ok() {
            play(
                engine,
                InputState {
                    forward: 127,
                    ..still(0)
                },
                20,
            )?;
            press(engine, Buttons::ACTION, 20)?;
        }
    }

    // (one vehicle of each kind: in, driven, fired, out)
    let mut kinds = BTreeSet::new();
    for row in rows(engine, "vehicle")? {
        let tag = row["tag"].as_str().unwrap_or_default().to_owned();
        if !kinds.insert(tag) {
            continue;
        }
        let vehicle = id(&row)?;
        run(engine, &format!("teleport {vehicle}"))?;
        play(engine, still(0), 5)?;
        if run(engine, &format!("enter {vehicle} 0")).is_err() {
            continue;
        }
        play(engine, still(0), 30)?;
        play(
            engine,
            InputState {
                forward: 127,
                up: 127,
                ..still(Buttons::JUMP.0)
            },
            120,
        )?;
        play(
            engine,
            InputState {
                right: 127,
                yaw: 0.05,
                ..still(Buttons::FIRE.0)
            },
            60,
        )?;
        press(engine, Buttons::GRENADE, 60)?;
        play(engine, still(0), 60)?;
        play(engine, still(Buttons::ACTION.0), 30)?;
        play(engine, still(0), 60)?;
    }

    // (dying, and spawning again)
    run(engine, "kill")?;
    play(engine, still(0), 150)?;
    engine.spawn_player(0)?;
    play(engine, still(0), 60)
}

#[test]
fn every_sound_the_level_refers_to_plays_when_its_event_happens() -> Result {
    let Some((engine, map)) = blood_gulch()? else {
        return Ok(()); // (no game data here)
    };
    let referred = referrers(&map)?;
    sweep(&engine)?;
    let heard = engine.sounds_heard();
    let asked: BTreeSet<String> = heard
        .iter()
        .filter(|sound| sound.in_level)
        .map(|sound| sound.path.to_lowercase())
        .collect();
    let unknown: Vec<_> = heard.iter().filter(|sound| !sound.in_level).collect();
    let failed: Vec<_> = heard
        .iter()
        .filter(|sound| sound.in_level && sound.failed > 0)
        .collect();

    // (the sounds no event asked for, by what refers to them)
    let mut silent: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for (sound, by) in &referred {
        if asked.contains(sound) {
            continue;
        }
        let group = by
            .iter()
            .filter_map(|by| by.split_once(' ').map(|(group, _)| group.to_owned()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join(",");
        silent
            .entry(if group.is_empty() {
                "(none)".into()
            } else {
                group
            })
            .or_default()
            .push(sound);
    }
    println!(
        "{} sounds referred to, {} asked for, {} never",
        referred.len(),
        asked.len(),
        referred.len().saturating_sub(asked.len())
    );
    for (group, sounds) in &silent {
        println!("never asked for, referred to by {group}:");
        for sound in sounds {
            let by: Vec<_> = referred
                .get(*sound)
                .map(|by| by.iter().take(3).cloned().collect())
                .unwrap_or_default();
            println!("  {sound}  <- {}", by.join("; "));
        }
    }
    // (asked for and never heard: culled each time it was asked for)
    let never_heard: Vec<_> = heard
        .iter()
        .filter(|sound| sound.in_level && sound.played == 0 && sound.culled > 0)
        .collect();
    for sound in &never_heard {
        println!("culled each time: {} ({}x)", sound.path, sound.culled);
    }
    for sound in &unknown {
        println!("not the level's: {} ({}x)", sound.path, sound.failed);
    }
    for sound in &failed {
        println!("failed: {} ({}x)", sound.path, sound.failed);
    }
    assert!(unknown.is_empty(), "asked for sounds the level has none of");
    // (the first-person weapon's animation sounds are heard whole, as
    // `first_person_weapon_update` plays them)
    let animations: Vec<_> = never_heard
        .iter()
        .filter(|sound| sound.path.contains("weapon_anims"))
        .map(|sound| &sound.path)
        .collect();
    assert!(
        animations.is_empty(),
        "first-person sounds never heard: {animations:?}"
    );
    // (and a sound of each kind of event the sweep makes)
    let missing: Vec<_> = EVENT_SOUNDS
        .iter()
        .filter(|path| {
            !heard
                .iter()
                .any(|sound| sound.path == **path && sound.played > 0)
        })
        .collect();
    assert!(
        missing.is_empty(),
        "the sweep's events made no sound: {missing:?}"
    );
    Ok(())
}
