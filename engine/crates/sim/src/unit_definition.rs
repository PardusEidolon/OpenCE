//! What a unit's definition answers of its seats and HUDs: whether a seat
//! drives, guns or lets noncombatants ride (units.c), and which of its HUDs
//! a screen shows (unit_definitions.c).
//!
//! Each is checked against the game's code by
//! tests/unit_tables_reference.rs.

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
}
