//! The small requests the game and its scripts make of a unit: its custom
//! animation stopped, its animation aborted, opened or closed, its emotion
//! and its seat set, and the base seat of the magic animations (units.c).
//!
//! Each says what the game asks of the unit's animation, interpolation and
//! network; the state change itself is the animation's
//! (`UnitAnimation::set_state`).
//!
//! Each is checked against the game's code by
//! tests/unit_requests_reference.rs.

use crate::unit_states::{base_seat_index, UnitState};

/// The frames a new emotion blends in over (`unit_set_emotion`).
pub const EMOTION_INTERPOLATION_FRAMES: i16 = 6;

/// A state a request asks the unit's animation for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateRequest {
    /// The state it asks for.
    pub state: UnitState,
    /// Whether the request is noted for the network's other machines.
    pub network_note: bool,
}

/// Stops a unit's custom animation: if `state`, the unit's, is a custom
/// animation, idle, noted for the network; otherwise, or no unit, nothing
/// (`unit_stop_custom_animation`).
pub fn stop_custom_animation(state: Option<UnitState>) -> Option<StateRequest> {
    (state == Some(UnitState::UserAnimation)).then_some(StateRequest {
        state: UnitState::Idle,
        network_note: true,
    })
}

/// Aborts a unit's animation: idle, with no note
/// (`unit_abort_animation`).
pub fn abort_animation() -> StateRequest {
    StateRequest {
        state: UnitState::Idle,
        network_note: false,
    }
}

/// Opens a unit (or closes it), noted for the network (`unit_open`,
/// `unit_close`).
pub fn open(opens: bool) -> StateRequest {
    StateRequest {
        state: if opens {
            UnitState::Opening
        } else {
            UnitState::Closing
        },
        network_note: true,
    }
}

/// Sets a unit's emotion: the game's word kept in its signed byte, blended
/// in over six frames, which it returns; no unit, nothing
/// (`unit_set_emotion`).
pub fn set_emotion(emotion_index: Option<&mut i8>, emotion: u16) -> Option<i16> {
    let index = emotion_index?;
    *index = i8::from_le_bytes([emotion.to_le_bytes()[0]]);
    Some(EMOTION_INTERPOLATION_FRAMES)
}

/// A seat as a unit's signed byte keeps it: -1 for none.
fn seat_byte(seat: Option<usize>) -> i8 {
    seat.and_then(|seat| i8::try_from(seat).ok()).unwrap_or(-1)
}

/// Sets the seat of a unit's magic animations to the base seat `label`
/// names, none for none it names; no unit, nothing
/// (`unit_scripting_set_seat`).
pub fn scripting_set_seat(magic_seat: Option<&mut i8>, label: &str) {
    if let Some(seat) = magic_seat {
        *seat = seat_byte(base_seat_index(label));
    }
}

/// The base seat of every unit's magic animations: the one `label` names,
/// none for none (`scripting_set_magic_base_seat`).
pub fn magic_base_seat(label: &str) -> Option<usize> {
    base_seat_index(label)
}
