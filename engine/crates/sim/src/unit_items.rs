//! The game's unit picking up, dropping and deleting its weapons, grenades
//! and equipment (units.c): the weapons deleted or dropped, the one in hand
//! dropped, a grenade, equipment or weapon added, the grenades dropped as
//! new objects and the unit's initial weapons made.
//!
//! What they do to the world's objects (delete, disconnect, show or hide,
//! drop, create) and what they ask of it is the caller's, an
//! [`ItemWorld`]; what they leave in the unit is its [`UnitInventory`] and
//! [`UnitEquipment`].
//!
//! Each is checked against the game's code by
//! tests/unit_items_reference.rs.

use crate::unit_inventory::UnitInventory;
use crate::unit_life::UnitEquipment;

/// The first person weapon's message that the weapon in hand is dropped.
pub const FIRST_PERSON_DROP_MESSAGE: i32 = 13;

/// The objects a unit's items are and the world they are in, as units.c
/// asks of them and acts on them.
pub trait ItemWorld {
    /// Whether the object is a weapon (`_object_type_weapon`).
    fn is_weapon(&self, object: u32) -> bool;
    /// Whether the object is in the map (`_object_connected_to_map_bit`).
    fn connected_to_map(&self, object: u32) -> bool;
    /// Whether the object is not drawn (`_object_invisible_bit`).
    fn invisible(&self, object: u32) -> bool;
    /// Whether the object rides or is held by another.
    fn has_parent(&self, object: u32) -> bool;
    /// The object's definition (tag index).
    fn definition(&self, object: u32) -> u32;
    /// The grenade type of an equipment's definition.
    fn grenade_type(&self, equipment: u32) -> i16;
    /// Whether the unit's seat takes the weapon's label
    /// (`unit_set_or_test_seat_and_weapon_label`, testing).
    fn seat_allows(&self, weapon: u32) -> bool;
    /// Whether the game engine lets the unit pick it up
    /// (game_engine_allow_pick_up).
    fn engine_allows_pick_up(&self, weapon: u32) -> bool;
    /// Whether the game engine lets the unit take it now
    /// (game_engine_picking_up).
    fn engine_picking_up(&self, weapon: u32) -> bool;
    /// Whether the weapon must be readied (weapon_must_be_readied).
    fn must_be_readied(&self, weapon: u32) -> bool;
    /// Whether the weapon can be fired (weapon_can_be_fired).
    fn can_be_fired(&self, weapon: u32) -> bool;
    /// Whether the unit is a local player's.
    fn local_player(&self) -> bool;
    /// Whether a game engine (multiplayer) is running
    /// (game_engine_running).
    fn engine_running(&self) -> bool;
    /// Whether this is a client making the host's unit, whose weapons are
    /// the host's (`network_objects_creating_host_object`).
    fn creating_host_object(&self) -> bool;

    /// Deletes the object (`object_delete`).
    fn delete(&mut self, object: u32);
    /// Takes the object out of the map (object_disconnect_from_map).
    fn disconnect(&mut self, object: u32);
    /// Shows or hides the object (object_set_visibility).
    fn set_visible(&mut self, object: u32, visible: bool);
    /// The equipment's pickup, for a local player
    /// (`equipment_handle_pickup`).
    fn handle_pickup(&mut self, equipment: u32);
    /// The item is now carried by the unit (item_in_unit_inventory).
    fn carried(&mut self, item: u32);
    /// Throws the item from the unit (unit_drop_item).
    fn drop_item(&mut self, item: u32);
    /// Puts the weapon away, answering whether it is
    /// (weapon_put_away).
    fn put_away(&mut self, weapon: u32, immediate: bool) -> bool;
    /// A message to the unit's first person weapon
    /// (first_person_weapon_message_from_unit).
    fn first_person_message(&mut self, message: i32);
    /// The player's desired weapon slot
    /// (player_control_set_desired_weapon).
    fn set_player_desired_weapon(&mut self, slot: usize);
    /// A new object of the definition, beside the unit, none if it can't
    /// be made (object_placement_data_new, `object_new`).
    fn create(&mut self, definition: Option<u32>) -> Option<u32>;
}

/// A kind of grenade of the game's globals: the most a unit carries and
/// the item dropped (`game_globals_grenade`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrenadeGlobals {
    /// The most a unit carries (`maximum_count`).
    pub maximum_count: i16,
    /// The equipment dropped for each (`item`).
    pub item: Option<u32>,
}

/// How a weapon is added (`_unit_add_weapon_normal` and the rest).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddWeapon {
    /// Picked up: the weapon used last is the one wanted.
    Normal,
    /// A starting weapon, the one wanted and the player's.
    Starting,
    /// Replacing all the others, the one wanted.
    Replace,
}

/// Whether the unit can use the weapon: its seat takes it, and then the
/// game engine allows it (`unit_can_use_weapon`).
pub fn can_use_weapon(world: &impl ItemWorld, weapon: u32) -> bool {
    world.seat_allows(weapon) && world.engine_allows_pick_up(weapon)
}

/// The slot to change to from `current` by `delta`, of the weapons the
/// unit can use (`unit_weapon_next_index`).
pub fn next_weapon_index(
    inventory: &UnitInventory<u32>,
    world: &impl ItemWorld,
    current: Option<usize>,
    delta: i16,
) -> Option<usize> {
    inventory.next_weapon(
        current,
        delta,
        |weapon| can_use_weapon(world, *weapon),
        |weapon| world.must_be_readied(*weapon),
    )
}

/// Whether a slot holds a weapon of the definition
/// (`unit_has_weapon_definition_index`).
pub fn has_weapon_definition(
    inventory: &UnitInventory<u32>,
    world: &impl ItemWorld,
    definition: u32,
) -> bool {
    inventory.has_weapon(|weapon| world.definition(*weapon) == definition)
}

/// Deletes every weapon but the one in hand; a wanted one deleted leaves
/// none wanted (`unit_delete_all_weapons`).
pub fn delete_all_weapons(inventory: &mut UnitInventory<u32>, world: &mut impl ItemWorld) {
    for slot in 0..inventory.weapons.len() {
        if Some(slot) == inventory.current_weapon {
            continue;
        }
        if let Some(weapon) = inventory.weapons[slot].take() {
            world.delete(weapon);
            if Some(slot) == inventory.desired_weapon {
                inventory.desired_weapon = None;
            }
        }
    }
}

/// Picks up a grenade, if the unit carries its type and has fewer than the
/// most: one more (the signed byte wrapping as the game's does), its pickup
/// for a local player and the grenade deleted
/// (`unit_add_grenade_to_inventory`).
pub fn add_grenade(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl ItemWorld,
    equipment: u32,
    grenades: &[GrenadeGlobals],
) -> bool {
    let grenade_type = world.grenade_type(equipment);
    let Ok(kind) = usize::try_from(grenade_type) else {
        return false;
    };
    let (Some(grenade), Some(count)) = (grenades.get(kind), inventory.grenades.get_mut(kind))
    else {
        return false;
    };
    if i16::from(*count) >= grenade.maximum_count {
        return false;
    }
    *count = count.wrapping_add(1);
    if world.local_player() {
        world.handle_pickup(equipment);
    }
    world.delete(equipment);
    true
}

/// Picks up equipment: the one held deleted first when `replace` is the
/// game's TRUE, and then, holding none, the new one taken out of the map,
/// hidden and carried (`unit_add_equipment_to_inventory`).
pub fn add_equipment(
    held: &mut UnitEquipment,
    world: &mut impl ItemWorld,
    equipment: u32,
    replace: i16,
) -> bool {
    if replace == 1 {
        if let Some(old) = held.0.take() {
            world.delete(old);
        }
    }
    if held.0.is_some() {
        return false;
    }
    world.disconnect(equipment);
    world.set_visible(equipment, false);
    if world.local_player() {
        world.handle_pickup(equipment);
    }
    world.carried(equipment);
    held.0 = Some(equipment);
    true
}

/// Drops the equipment held, if any (`unit_drop_current_equipment`).
pub fn drop_current_equipment(held: &mut UnitEquipment, world: &mut impl ItemWorld) {
    if let Some(equipment) = held.0.take() {
        world.drop_item(equipment);
    }
}

/// Drops every weapon but the one in hand; a wanted one dropped makes the
/// one in hand wanted, and one that can't be fired is deleted
/// (`unit_drop_inventory_weapons`).
pub fn drop_inventory_weapons(inventory: &mut UnitInventory<u32>, world: &mut impl ItemWorld) {
    for slot in 0..inventory.weapons.len() {
        if Some(slot) == inventory.current_weapon {
            continue;
        }
        if let Some(weapon) = inventory.weapons[slot].take() {
            world.drop_item(weapon);
            if Some(slot) == inventory.desired_weapon {
                inventory.desired_weapon = inventory.current_weapon;
            }
            if !world.can_be_fired(weapon) {
                world.delete(weapon);
            }
        }
    }
}

/// Drops each grenade carried as a new object of its type's item, taken
/// out of the map and thrown; a count below one drops none and stays
/// (`unit_drop_grenades`). The game's globals have both types; a type they
/// lack (which the game would read past) is emptied with none dropped.
pub fn drop_grenades(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl ItemWorld,
    grenades: &[GrenadeGlobals],
) {
    for (kind, count) in inventory.grenades.iter_mut().enumerate() {
        let Some(grenade) = grenades.get(kind) else {
            *count = (*count).min(0);
            continue;
        };
        while *count > 0 {
            if let Some(object) = world.create(grenade.item) {
                world.disconnect(object);
                world.drop_item(object);
            }
            *count -= 1;
        }
    }
}

/// Drops the weapon in hand, if there is another to change to or it is
/// `immediate`, it is drawn and it puts away: its first person weapon told,
/// the weapon thrown, none in hand, the next wanted, and deleted if it
/// can't be fired (`unit_drop_current_weapon`).
pub fn drop_current_weapon(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl ItemWorld,
    immediate: bool,
) -> bool {
    let current = inventory.current_weapon;
    let next = next_weapon_index(inventory, world, current, 1);
    let Some((slot, weapon)) = current.zip(inventory.weapon(current).copied()) else {
        return false;
    };
    if !(next != current || immediate)
        || world.invisible(weapon)
        || !world.put_away(weapon, immediate)
    {
        return false;
    }
    world.first_person_message(FIRST_PERSON_DROP_MESSAGE);
    world.drop_item(weapon);
    inventory.weapons[slot] = None;
    inventory.current_weapon = None;
    inventory.desired_weapon = next_weapon_index(inventory, world, None, 0);
    if !world.can_be_fired(weapon) {
        world.delete(weapon);
    }
    true
}

/// Picks up a weapon: a weapon in the map, held by none, the unit can use
/// and the engine lets it take, into the first free slot (after the rest
/// are deleted, replacing), taken out of the map, hidden and carried, never
/// used, and wanted as `mode` says (`unit_add_weapon_to_inventory`).
/// `trigger_down` is the unit's primary trigger, which keeps a starting
/// weapon into a one-weapon loadout from being the player's.
pub fn add_weapon(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl ItemWorld,
    weapon: u32,
    mode: AddWeapon,
    trigger_down: bool,
) -> bool {
    if !world.is_weapon(weapon) {
        return false;
    }
    if !(world.connected_to_map(weapon)
        && !world.has_parent(weapon)
        && can_use_weapon(world, weapon)
        && world.engine_picking_up(weapon))
    {
        return false;
    }
    if mode == AddWeapon::Replace {
        delete_all_weapons(inventory, world);
    }
    let Some(slot) = inventory.first_free_weapon() else {
        return false;
    };
    world.disconnect(weapon);
    world.set_visible(weapon, false);
    world.carried(weapon);
    inventory.weapons[slot] = Some(weapon);
    inventory.last_used[slot] = 0;
    match mode {
        AddWeapon::Normal => {
            inventory.desired_weapon =
                next_weapon_index(inventory, world, inventory.current_weapon, 0);
        }
        AddWeapon::Starting => {
            let holding_another = inventory
                .weapons
                .iter()
                .enumerate()
                .any(|(other, held)| other != slot && held.is_some());
            if holding_another || !trigger_down {
                world.set_player_desired_weapon(slot);
            }
            inventory.desired_weapon = Some(slot);
        }
        AddWeapon::Replace => inventory.desired_weapon = Some(slot),
    }
    true
}

/// Makes the unit's initial weapons and picks each up; one that can't be
/// picked up, or whose definition the unit holds while a game engine runs,
/// is deleted. A client making the host's unit makes none
/// (`unit_add_initial_weapons`).
pub fn add_initial_weapons(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl ItemWorld,
    initial_weapons: &[Option<u32>],
) {
    if world.creating_host_object() {
        return;
    }
    for definition in initial_weapons.iter().flatten() {
        let Some(weapon) = world.create(Some(*definition)) else {
            continue;
        };
        let held = world.engine_running()
            && has_weapon_definition(inventory, world, world.definition(weapon));
        if held || !add_weapon(inventory, world, weapon, AddWeapon::Normal, false) {
            world.delete(weapon);
        }
    }
}
