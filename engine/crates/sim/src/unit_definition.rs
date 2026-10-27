//! What a unit's definition answers of its seats and HUDs: whether a seat
//! drives, guns or lets noncombatants ride, which seat label a rider's
//! animations use, which seats a script finds or tests (units.c), and which
//! of its HUDs a screen shows (unit_definitions.c).
//!
//! Each is checked against the game's code by
//! tests/unit_tables_reference.rs and tests/unit_seats_reference.rs. A
//! seat is its flags and its label, as `opence_content::VehicleSeat` holds
//! them.

use opence_content::VehicleSeat;

/// Whether seat `seat` of a unit whose seats carry `seats`' flags has
/// `flag`; no seat past them does (`unit_seat_is_driver`,
/// `unit_seat_is_gunner`, `unit_seat_allow_noncombatants`, with
/// `VehicleSeat::DRIVER`, `GUNNER` and `ALLOWS_NONCOMBATANTS`).
pub fn seat_is(seats: &[u32], seat: i16, flag: u32) -> bool {
    usize::try_from(seat)
        .ok()
        .and_then(|seat| seats.get(seat))
        .is_some_and(|flags| flags & flag != 0)
}

/// Which of `count` HUDs a screen shows: the second in multiplayer when
/// there is one, otherwise the first; none with none
/// (`unit_definition_get_active_hud_index`,
/// `unit_definition_get_seat_active_hud_index`).
pub fn active_hud(count: usize, multiplayer: bool) -> Option<usize> {
    let wanted = usize::from(multiplayer);
    count.checked_sub(1).map(|last| wanted.min(last))
}

/// The seat labels of a unit that rides nothing, by its base seat
/// (`base_seat_labels`, by `_unit_base_seat_asleep` and the rest).
pub const BASE_SEAT_LABELS: [&str; 6] = ["asleep", "alert", "stand", "crouch", "flee", "flaming"];

/// The seat label a unit's animations use: its seat's in the unit it rides,
/// given that unit's seat labels and the seat, otherwise its base seat's;
/// none for a seat past them (`unit_get_seat_label`).
pub fn seat_label<'a>(riding: Option<(&[&'a str], usize)>, base_seat: usize) -> Option<&'a str> {
    match riding {
        Some((labels, seat)) => labels.get(seat).copied(),
        None => BASE_SEAT_LABELS.get(base_seat).copied(),
    }
}

/// Whether a unit sits in a seat of a vehicle named `name`, in any case,
/// given the vehicle's seat labels and the seat the unit sits in in it
/// (none when it rides another or nothing, or there is no vehicle or no
/// unit) (`unit_scripting_vehicle_test_seat`).
pub fn test_seat(labels: &[&str], name: &str, seat: Option<i16>) -> bool {
    seat.and_then(|seat| usize::try_from(seat).ok())
        .and_then(|seat| labels.get(seat))
        .is_some_and(|label| label.eq_ignore_ascii_case(name))
}

/// Which seats a script wants (`_vehicle_seat_desire_not_driver` and the
/// rest).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeatDesire {
    /// Any seat but the driver's.
    NotDriver,
    /// A gunner's seat.
    Gunner,
    /// A seat that neither drives nor guns.
    Passenger,
    /// A driver's seat.
    Driver,
    /// Any seat.
    Any,
}

impl SeatDesire {
    /// The desire of the game's index, none past them.
    pub fn from_index(index: i16) -> Option<Self> {
        Some(match index {
            0 => Self::NotDriver,
            1 => Self::Gunner,
            2 => Self::Passenger,
            3 => Self::Driver,
            4 => Self::Any,
            _ => return None,
        })
    }

    /// Whether a seat with `flags` meets it.
    fn met_by(self, flags: u32) -> bool {
        let driver = flags & VehicleSeat::DRIVER != 0;
        let gunner = flags & VehicleSeat::GUNNER != 0;
        match self {
            Self::NotDriver => !driver,
            Self::Gunner => gunner,
            Self::Passenger => !driver && !gunner,
            Self::Driver => driver,
            Self::Any => true,
        }
    }
}

/// The seats, at most `maximum`, whose labels in lower case hold `name` (as
/// it is given, in its case; every seat for an empty name), that meet
/// `desire` (every seat for none) and that are not `filled`
/// (`vehicle_scripting_find_available_seats`).
pub fn available_seats<'a>(
    seats: impl IntoIterator<Item = (u32, &'a str)>,
    name: &str,
    desire: Option<SeatDesire>,
    maximum: i16,
    filled: impl Fn(usize) -> bool,
) -> Vec<usize> {
    let maximum = usize::try_from(maximum).unwrap_or(0);
    let mut found = Vec::new();
    for (index, (flags, label)) in seats.into_iter().enumerate() {
        if found.len() >= maximum {
            break;
        }
        if !label.to_ascii_lowercase().contains(name) {
            continue;
        }
        if desire.is_none_or(|desire| desire.met_by(flags)) && !filled(index) {
            found.push(index);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplayer_falls_back_to_the_only_hud() {
        assert_eq!(active_hud(0, true), None);
        assert_eq!(active_hud(1, true), Some(0));
        assert_eq!(active_hud(2, true), Some(1));
        assert_eq!(active_hud(2, false), Some(0));
    }

    #[test]
    fn a_seat_name_is_found_in_the_label_as_it_is_given() {
        let seats = [(0, "W-Gunner"), (VehicleSeat::DRIVER, "driver")];
        assert_eq!(available_seats(seats, "gunner", None, 4, |_| false), [0]);
        assert!(available_seats(seats, "Gunner", None, 4, |_| false).is_empty());
        assert_eq!(
            available_seats(seats, "", Some(SeatDesire::NotDriver), 4, |_| false),
            [0]
        );
        assert!(test_seat(&["W-Gunner"], "w-gunner", Some(0)));
    }
}
