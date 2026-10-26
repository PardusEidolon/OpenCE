//! The main menu's title music, as `ui_start_main_menu_music` plays it:
//! ui.map's looping sound `sound\music\title1\title1` plays while the menu
//! asks for it and stops when it no longer does. Passes without
//! assets/maps/ui.map.

use std::error::Error;

use opence_ffi::ui::{main_menu_widget, Menu};
use opence_ffi::{Engine, LevelGeometry};
use opence_import::testing::{cache_folder, maps_folder};

type Result = std::result::Result<(), Box<dyn Error>>;

const TITLE_MUSIC: &str = "sound\\music\\title1\\title1";

/// The music class's voices playing.
fn music(engine: &Engine) -> u32 {
    engine
        .audio_census()
        .iter()
        .filter(|count| count.class == "music")
        .map(|count| count.playing)
        .sum()
}

#[test]
fn the_menus_title_music_loops_until_it_is_stopped() -> Result {
    let engine = Engine::new();
    // (no world: nothing to play it in)
    engine.set_menu_music(Some(TITLE_MUSIC.into()));
    assert!(engine.advance(1.0 / 60.0).is_err());
    assert_eq!(music(&engine), 0);

    let map = maps_folder().join("ui.map");
    if !map.exists() {
        return Ok(()); // (no game data here)
    }
    let geometry = LevelGeometry::load(
        map.to_string_lossy().into_owned(),
        cache_folder().to_string_lossy().into_owned(),
        0,
    )?;
    // (ui.map's main menu asks for it as it opens)
    let menu = Menu::new(geometry.clone());
    assert_eq!(menu.music(), None);
    assert!(menu.open(main_menu_widget()));
    menu.update(16);
    assert_eq!(menu.music().as_deref(), Some(TITLE_MUSIC));

    engine.start_level(geometry, 1)?;
    engine.set_menu_music(None);
    for _ in 0..10 {
        engine.advance(1.0 / 60.0)?;
    }
    assert_eq!(music(&engine), 0, "music before the menu asked for it");

    engine.set_menu_music(Some(TITLE_MUSIC.into()));
    for _ in 0..10 {
        engine.advance(1.0 / 60.0)?;
    }
    assert!(music(&engine) >= 1, "{:?}", engine.audio_census());
    // (one loop, however long it is asked for)
    for _ in 0..120 {
        engine.advance(1.0 / 60.0)?;
    }
    assert_eq!(music(&engine), 1, "{:?}", engine.audio_census());

    // (stopped, it plays out what it was playing and starts no more:
    // `looping_sound_stop` plays each track's end)
    engine.set_menu_music(None);
    let output = engine.audio_output();
    let mut seconds = 0.0;
    while music(&engine) > 0 && seconds < 120.0 {
        engine.advance(1.0 / 60.0)?;
        output.pull(800);
        seconds += 1.0 / 60.0;
    }
    assert_eq!(music(&engine), 0, "still playing after {seconds} s");
    Ok(())
}
