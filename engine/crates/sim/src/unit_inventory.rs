//! A unit's inventory as the game keeps it in its unit datum (units.h): four
//! weapon slots, each with the game time its weapon was last used, a
//! current and desired weapon, a signed byte of grenades of each kind, a
//! current and desired grenade and a zoom level, with units.c's functions
//! that pick the next weapon and grenade and answer what it holds.
//!
//! Each is checked against the game's code by
//! tests/unit_inventory_reference.rs. The weapon itself is the caller's
//! (`W`): the functions that ask whether one can be used or must be readied
//! take the question as a closure, which units.c asks of the unit's seat
//! and the weapon's definition.

/// The weapons a unit carries at most (`MAXIMUM_WEAPONS_PER_UNIT`).
pub const WEAPONS_PER_UNIT: usize = 4;
/// The kinds of grenade a unit carries (`NUMBER_OF_UNIT_GRENADE_TYPES`).
pub const GRENADE_TYPES: usize = 2;

/// A unit's weapons, grenades and zoom, as its unit datum holds them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitInventory<W> {
    /// The weapon in each slot (`weapon_object_indices`).
    pub weapons: [Option<W>; WEAPONS_PER_UNIT],
    /// The game time each slot's weapon was last readied
    /// (`weapon_last_used_at_game_time`).
    pub last_used: [i32; WEAPONS_PER_UNIT],
    /// The slot in hand (`current_weapon_index`).
    pub current_weapon: Option<usize>,
    /// The slot it is changing to (`desired_weapon_index`).
    pub desired_weapon: Option<usize>,
    /// The grenades of each kind, a signed byte as the game's
    /// (`grenade_counts`).
    pub grenades: [i8; GRENADE_TYPES],
    /// The kind it throws (`current_grenade_index`).
    pub current_grenade: Option<usize>,
    /// The kind it is changing to (`desired_grenade_index`).
    pub desired_grenade: Option<usize>,
    /// Its zoom level, none unzoomed (`current_zoom_level`).
    pub current_zoom: Option<u8>,
    /// The zoom level it is changing to (`desired_zoom_level`).
    pub desired_zoom: Option<u8>,
}

impl<W> Default for UnitInventory<W> {
    fn default() -> Self {
        Self {
            weapons: std::array::from_fn(|_| None),
            last_used: [0; WEAPONS_PER_UNIT],
            current_weapon: None,
            desired_weapon: None,
            grenades: [0; GRENADE_TYPES],
            current_grenade: None,
            desired_grenade: None,
            current_zoom: None,
            desired_zoom: None,
        }
    }
}

/// The grenades a unit holds of every kind, as a script asks: none for
/// no unit (`unit_scripting_get_grenade_count`).
pub fn scripting_grenade_count<W>(unit: Option<&UnitInventory<W>>) -> i16 {
    unit.map_or(0, |unit| {
        unit.grenades.iter().map(|count| i16::from(*count)).sum()
    })
}

/// The index after `index` of `count`, by `delta`'s sign (none counts as
/// forward), wrapping.
fn step(index: usize, delta: i16, count: usize) -> usize {
    if delta < 0 {
        if index == 0 {
            count - 1
        } else {
            index - 1
        }
    } else if index + 1 == count {
        0
    } else {
        index + 1
    }
}

impl<W> UnitInventory<W> {
    /// The weapon in slot `index`, none for none or past the slots
    /// (`unit_get_weapon`, `unit_inventory_get_weapon`).
    pub fn weapon(&self, index: Option<usize>) -> Option<&W> {
        self.weapons.get(index?)?.as_ref()
    }

    /// The slot to change to from `current` (none as the first) by
    /// `delta` (`unit_weapon_next_index`, `unit_inventory_next_weapon`).
    ///
    /// Stepping by `delta` from `current`, it takes the first usable weapon
    /// past `current`. With no `delta` it takes, of the usable weapons, the
    /// one used last, stopping at the first past `current`. A weapon that
    /// must be readied ends the search where it is found.
    pub fn next_weapon(
        &self,
        current: Option<usize>,
        delta: i16,
        usable: impl Fn(&W) -> bool,
        must_be_readied: impl Fn(&W) -> bool,
    ) -> Option<usize> {
        let start = current.unwrap_or(0);
        if start >= WEAPONS_PER_UNIT {
            return None;
        }
        let mut selected: Option<usize> = None;
        let mut index = start;
        loop {
            if let Some(weapon) = self.weapons[index].as_ref().filter(|weapon| usable(weapon)) {
                let later = match selected {
                    None => true,
                    Some(selected) => self.last_used[selected] < self.last_used[index],
                };
                if delta != 0 || later {
                    selected = Some(index);
                }
                if must_be_readied(weapon) || index != start {
                    break;
                }
            }
            index = step(index, delta, WEAPONS_PER_UNIT);
            if index == start {
                break;
            }
        }
        selected
    }

    /// The kind of grenade to change to from `current` (none as the first)
    /// by `delta`: the first past it with any, or `current` itself when no
    /// other has any or `delta` is none (`unit_inventory_next_grenade`).
    pub fn next_grenade(&self, current: Option<usize>, delta: i16) -> Option<usize> {
        let start = current.unwrap_or(0);
        if start >= GRENADE_TYPES {
            return None;
        }
        let mut next = None;
        let mut index = start;
        loop {
            if self.grenades[index] > 0 {
                next = Some(index);
                if index != start || delta == 0 {
                    break;
                }
            }
            index = step(index, delta, GRENADE_TYPES);
            if index == start {
                break;
            }
        }
        next
    }

    /// The first empty slot (`unit_first_free_weapon_index`).
    pub fn first_free_weapon(&self) -> Option<usize> {
        self.weapons.iter().position(Option::is_none)
    }

    /// Adds `count` grenades of `kind`, makes it the kind thrown and
    /// answers how many it now has; a kind the unit doesn't carry (a map's
    /// actor variant can name one) adds none and answers none
    /// (`unit_add_grenade_type_to_inventory`). The count is a signed byte,
    /// and wraps as the game's does.
    pub fn add_grenades(&mut self, kind: i16, count: i16) -> i16 {
        let Some(kind) = usize::try_from(kind)
            .ok()
            .filter(|kind| *kind < GRENADE_TYPES)
        else {
            return 0;
        };
        // the game's char += short: the sum, truncated to a byte
        let sum = i16::from(self.grenades[kind]).wrapping_add(count);
        self.grenades[kind] = sum.to_le_bytes()[0] as i8;
        self.desired_grenade = Some(kind);
        self.current_grenade = Some(kind);
        i16::from(self.grenades[kind])
    }

    /// Whether any slot holds a weapon `test` accepts
    /// (`unit_has_weapon_definition_index`, `unit_has_weapon_with_flag`).
    pub fn has_weapon(&self, test: impl Fn(&W) -> bool) -> bool {
        self.weapons.iter().flatten().any(test)
    }

    /// Whether any slot holds `weapon`, none matching an empty slot as the
    /// game's NONE does (`unit_has_weapon`).
    pub fn holds(&self, weapon: Option<&W>) -> bool
    where
        W: PartialEq,
    {
        self.weapons.iter().any(|slot| slot.as_ref() == weapon)
    }

    /// The grenades of `kind`, none for none or a kind the unit doesn't
    /// carry (`unit_get_grenade_count`).
    pub fn grenade_count(&self, kind: Option<i16>) -> i16 {
        kind.and_then(|kind| usize::try_from(kind).ok())
            .and_then(|kind| self.grenades.get(kind))
            .map_or(0, |count| i16::from(*count))
    }

    /// The kind of grenade it throws (`unit_get_current_grenade_type`).
    pub fn current_grenade_type(&self) -> Option<usize> {
        self.current_grenade
    }

    /// The first slot whose weapon must be readied
    /// (`unit_inventory_get_must_be_readied_weapon`).
    pub fn must_be_readied_weapon(&self, must_be_readied: impl Fn(&W) -> bool) -> Option<usize> {
        self.weapons
            .iter()
            .position(|weapon| weapon.as_ref().is_some_and(&must_be_readied))
    }

    /// Its zoom level, none unzoomed (`unit_get_zoom_level`).
    pub fn zoom_level(&self) -> Option<u8> {
        self.current_zoom
    }

    /// The weapon in hand (`unit_get_current_weapon_index`).
    pub fn current(&self) -> Option<&W> {
        self.weapon(self.current_weapon)
    }

    /// The magnification of the weapon in hand, by `magnification`, and 1
    /// with none (`unit_get_zoom_magnification`).
    pub fn zoom_magnification(&self, magnification: impl Fn(&W) -> f32) -> f32 {
        self.current().map_or(1.0, magnification)
    }

    /// Whether the weapon in hand is overcharged (`unit_overcharged`).
    pub fn overcharged(&self, overcharged: impl Fn(&W) -> bool) -> bool {
        self.current().is_some_and(overcharged)
    }

    /// The label of the weapon in hand, by `label`, and "unarmed" with none
    /// (`unit_get_current_weapon_label`).
    pub fn current_weapon_label<'a>(&self, label: impl Fn(&W) -> &'a str) -> &'a str {
        self.current().map_or("unarmed", label)
    }

    /// Whether its integrated night vision is on: it is zoomed with a weapon
    /// that `enables` it (`unit_integrated_night_vision_is_active`).
    pub fn night_vision_active(&self, enables: impl Fn(&W) -> bool) -> bool {
        self.current_zoom.is_some() && self.current().is_some_and(enables)
    }

    /// Unzooms it, answering the zoom-out sound of the weapon in hand (by
    /// `zoom_out_sound`) when a local player zoomed it; the caller plays it,
    /// turns its night vision power to 0 and unzooms the player's control
    /// (`unit_unzoom`).
    pub fn unzoom<S>(
        &mut self,
        local_player: bool,
        zoom_out_sound: impl Fn(&W) -> Option<S>,
    ) -> Option<S> {
        let sound = if local_player && self.current_zoom.is_some() {
            self.current().and_then(zoom_out_sound)
        } else {
            None
        };
        self.current_zoom = None;
        self.desired_zoom = None;
        sound
    }
}

/// Whether the one local player's unit (none for none) has its integrated
/// night vision on; with more or fewer local players none has
/// (`unit_solo_player_integrated_night_vision_is_active`).
pub fn solo_player_night_vision<W>(
    local_players: usize,
    unit: Option<&UnitInventory<W>>,
    enables: impl Fn(&W) -> bool,
) -> bool {
    local_players == 1 && unit.is_some_and(|unit| unit.night_vision_active(enables))
}

/// The weapon a unit melees with unarmed: the first of the game globals'
/// weapon list, none with no globals or an empty list
/// (`unarmed_melee_weapon_definition_index`).
pub fn unarmed_melee_weapon<D: Copy>(weapon_list: Option<&[D]>) -> Option<D> {
    weapon_list.and_then(|list| list.first().copied())
}

/// Whether a unit (none for none) holds a weapon whose definition, by
/// `definition_of`, is `definition` (none for none): a script's question,
/// which asks no unit or no definition as none (`unit_scripting_has_weapon`,
/// asking `unit_has_weapon_definition_index`).
pub fn scripting_has_weapon<W, D: PartialEq>(
    inventory: Option<&UnitInventory<W>>,
    definition: Option<D>,
    definition_of: impl Fn(&W) -> D,
) -> bool {
    match (inventory, definition) {
        (Some(inventory), Some(definition)) => {
            inventory.has_weapon(|weapon| definition_of(weapon) == definition)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_no_delta_the_weapon_used_last_is_readied() {
        let inventory = UnitInventory {
            weapons: [Some(()), Some(()), None, None],
            last_used: [30, 90, 0, 0],
            ..UnitInventory::default()
        };
        assert_eq!(
            inventory.next_weapon(Some(0), 0, |_| true, |_| false),
            Some(1)
        );
        assert_eq!(
            inventory.next_weapon(Some(0), 1, |_| true, |_| false),
            Some(1)
        );
    }

    #[test]
    fn adding_grenades_wraps_a_signed_byte() {
        let mut inventory: UnitInventory<()> = UnitInventory {
            grenades: [120, 0],
            ..UnitInventory::default()
        };
        assert_eq!(inventory.add_grenades(0, 10), -126);
        assert_eq!(inventory.current_grenade, Some(0));
        assert_eq!(inventory.add_grenades(7, 3), 0);
    }
}
