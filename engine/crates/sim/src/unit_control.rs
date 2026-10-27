//! A unit's flags as its unit datum keeps them (units.h), with units.c's
//! functions that set and ask them, move its mouth and ask whether AI
//! drives or guns it and whether a seat of it is filled.
//!
//! Each is checked against the game's code by
//! tests/unit_control_reference.rs and tests/unit_seats_reference.rs.

/// A unit's flags (`unit_datum.unit.flags`): a word of the bits below.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnitFlags(pub u32);

impl UnitFlags {
    /// It is controlled by a player or an actor (`_unit_actively_controlled_bit`).
    pub const ACTIVELY_CONTROLLED: u32 = 1 << 0;
    /// Its controls move it (`_unit_controllable_bit`).
    pub const CONTROLLABLE: u32 = 1 << 6;
    /// It aims without turning to face its aim (`_unit_aim_without_turning_bit`).
    pub const AIM_WITHOUT_TURNING: u32 = 1 << 14;
    /// It does not blink (`_unit_cannot_blink_bit`).
    pub const CANNOT_BLINK: u32 = 1 << 22;
    /// Its integrated light is on (`_unit_integrated_light_on_bit`).
    pub const INTEGRATED_LIGHT_ON: u32 = 1 << 19;
    /// It runs blindly, as a flood-infected unit does (`_unit_running_blindly_bit`).
    pub const RUNNING_BLINDLY: u32 = 1 << 25;
    /// A recording plays it (`_unit_possessed_by_recording_bit`).
    pub const POSSESSED_BY_RECORDING: u32 = 1 << 27;
    /// Its integrated light is wished on (`_unit_desired_integrated_light_on_bit`).
    pub const DESIRED_INTEGRATED_LIGHT_ON: u32 = 1 << 28;
    /// Its integrated light is wished off (`_unit_desired_integrated_light_off_bit`).
    pub const DESIRED_INTEGRATED_LIGHT_OFF: u32 = 1 << 29;

    /// Whether `flag` is set.
    pub fn has(self, flag: u32) -> bool {
        self.0 & flag != 0
    }

    /// Sets `flag` to `on`.
    pub fn set(&mut self, flag: u32, on: bool) {
        if on {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }

    /// Whether its controls move it (`unit_controllable`).
    pub fn controllable(self) -> bool {
        self.has(Self::CONTROLLABLE)
    }

    /// Lets its controls move it, or not (`unit_set_controllable`).
    pub fn set_controllable(&mut self, controllable: bool) {
        self.set(Self::CONTROLLABLE, controllable);
    }

    /// Lets a recording play it, or not (`unit_set_possessed`).
    pub fn set_possessed(&mut self, possessed: bool) {
        self.set(Self::POSSESSED_BY_RECORDING, possessed);
    }

    /// Wishes its integrated light on or off; the wish is kept until the
    /// unit's update answers it, and a wish one way leaves one the other
    /// way standing (`unit_set_desired_flashlight_state`).
    pub fn desire_flashlight(&mut self, on: bool) {
        let flag = if on {
            Self::DESIRED_INTEGRATED_LIGHT_ON
        } else {
            Self::DESIRED_INTEGRATED_LIGHT_OFF
        };
        self.set(flag, true);
    }

    /// Whether its integrated light is on (`unit_get_current_flashlight_state`).
    pub fn flashlight_on(self) -> bool {
        self.has(Self::INTEGRATED_LIGHT_ON)
    }

    /// Stops it running blindly (`unit_stop_running_blindly`).
    pub fn stop_running_blindly(&mut self) {
        self.set(Self::RUNNING_BLINDLY, false);
    }

    /// Lets it blink, or not (`unit_scripting_can_blink`).
    pub fn set_can_blink(&mut self, can_blink: bool) {
        self.set(Self::CANNOT_BLINK, !can_blink);
    }

    /// Makes it actively controlled, and controllable, when it is `wanted`
    /// to be or an actor, a swarm or a player controls it (`someone`),
    /// unless it is `dead`, answering whether it is. The caller then tells
    /// each of its weapons it is in its inventory and updates its driver
    /// and gunner (`unit_set_actively_controlled`).
    pub fn set_actively_controlled(&mut self, wanted: bool, someone: bool, dead: bool) -> bool {
        let active = !dead && (wanted || someone);
        self.set(Self::ACTIVELY_CONTROLLED, active);
        self.set(Self::CONTROLLABLE, active);
        active
    }
}

/// The units in a unit's driver's and gunner's seats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Crew<U> {
    /// Its driver (`driver_object_index`).
    pub driver: Option<U>,
    /// Its gunner (`gunner_object_index`).
    pub gunner: Option<U>,
}

/// Sets whether a unit aims without turning to face its aim
/// (`unit_aim_without_turning`); no unit is left as it is.
pub fn aim_without_turning(unit: Option<&mut UnitFlags>, on: bool) {
    if let Some(flags) = unit {
        flags.set(UnitFlags::AIM_WITHOUT_TURNING, on);
    }
}

/// A script's question of a unit's driver and gunner: none of no unit
/// (`unit_scripting_unit_driver`, `unit_scripting_unit_gunner`).
pub fn scripting_crew<U: Copy>(unit: Option<&Crew<U>>) -> Crew<U> {
    unit.copied().unwrap_or(Crew {
        driver: None,
        gunner: None,
    })
}

/// The children of a unit (each with whether it is a unit and its seat)
/// that are units in a seat, in its children's order: a script's list of
/// its riders, none of no unit (`unit_scripting_unit_riders`).
pub fn seated_riders<O>(
    children: Option<impl IntoIterator<Item = (O, bool, Option<i16>)>>,
) -> Option<Vec<O>> {
    children.map(|children| {
        children
            .into_iter()
            .filter(|(_, is_unit, seat)| *is_unit && seat.is_some())
            .map(|(child, _, _)| child)
            .collect()
    })
}

/// The mouth aperture after one tick from `current` towards `wanted`,
/// moving at most 0.3 (`unit_set_mouth_aperture`).
pub fn mouth_aperture(current: f32, wanted: f32) -> f32 {
    // the game's PIN; clamp too keeps a NaN step
    current + (wanted - current).clamp(-0.3, 0.3)
}

/// Whether AI drives (or guns) a unit: its rider's actor when it has one,
/// otherwise its own (`unit_driven_by_ai`, `unit_gunned_by_ai`).
/// `own_actor` is whether the unit has an actor, `rider_actor` whether its
/// driver (or gunner) has one, none when it has none.
pub fn ridden_by_ai(own_actor: bool, rider_actor: Option<bool>) -> bool {
    rider_actor.unwrap_or(own_actor)
}

/// Whether any of `riders` (each unit's parent and seat, none for none) is
/// in seat `seat` of `unit`; a unit with no parent and no seat fills a
/// none unit's none seat, as the game's NONE does (`unit_seat_filled`).
pub fn seat_filled<U: PartialEq>(
    riders: impl IntoIterator<Item = (Option<U>, Option<i16>)>,
    unit: Option<U>,
    seat: Option<i16>,
) -> bool {
    riders
        .into_iter()
        .any(|(parent, parent_seat)| parent == unit && parent_seat == seat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_flashlight_wish_leaves_the_other_standing() {
        let mut flags = UnitFlags::default();
        flags.desire_flashlight(true);
        flags.desire_flashlight(false);
        assert!(flags.has(UnitFlags::DESIRED_INTEGRATED_LIGHT_ON));
        assert!(flags.has(UnitFlags::DESIRED_INTEGRATED_LIGHT_OFF));
        assert!(!flags.flashlight_on());
    }

    #[test]
    fn the_mouth_moves_at_most_three_tenths() {
        assert_eq!(mouth_aperture(0.0, 1.0), 0.3);
        assert_eq!(mouth_aperture(1.0, 0.0), 0.7);
        assert_eq!(mouth_aperture(0.5, 0.6), 0.6);
    }
}
