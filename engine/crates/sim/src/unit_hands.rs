//! What the game's unit does with its hands (units.c): the weapon it wants
//! readied (the one in hand put away first), the weapon wanted again after
//! a change of seat, an item dropped and thrown from it, a grenade made in
//! its left hand as it throws, and the actors it spawns.
//!
//! What they do to the world's objects and what they ask of it is the
//! caller's, a [`HandWorld`] (which is also the unit's [`ItemWorld`]);
//! what they draw at random is from the game's [`RandomSeed`].
//!
//! Each is checked against the game's code by
//! tests/unit_hands_reference.rs.

use glam::Vec3;

use crate::random::{normalize, RandomSeed};
use crate::unit_inventory::UnitInventory;
use crate::unit_items::{next_weapon_index, ItemWorld};

/// A grenade's throw with the grenade in the hand
/// (`_unit_grenade_throw_in_hand`).
pub const GRENADE_THROW_IN_HAND: i8 = 2;

/// The most an item dropped is turned from the unit's aim, in radians.
const DROP_CONE: f32 = core::f32::consts::FRAC_PI_8;

/// The least and most speed an item is dropped with, in world units a
/// tick.
const DROP_SPEED: (f32, f32) = (0.026_666_667, 0.040_000_003);

/// A second's speed in a tick's (1/30 of a second).
const TICK: f32 = 0.033_333_335;

/// The world a unit's hands are in, as units.c asks of it and acts on it,
/// beyond its items'.
pub trait HandWorld: ItemWorld {
    /// The game's time, in ticks.
    fn game_time(&self) -> i32;
    /// The unit's velocity (object_get_velocities).
    fn unit_velocity(&self) -> Vec3;
    /// Where the unit's camera is (unit_get_camera_position).
    fn camera_position(&self) -> Vec3;
    /// Whether the unit is a player's.
    fn has_player(&self) -> bool;
    /// Whether the infinite ammunition cheat is on.
    fn cheat_infinite_ammo(&self) -> bool;
    /// Whether the game engine gives the unit's player infinite grenades
    /// (game_engine_infinite_grenades).
    fn engine_infinite_grenades(&self) -> bool;
    /// Whether the unit is an actor's.
    fn has_actor(&self) -> bool;
    /// Whether the unit's actor has unlimited grenades
    /// (actor_has_unlimited_grenades).
    fn actor_unlimited_grenades(&self) -> bool;
    /// Whether this is a distributed game's client, whose grenade counts
    /// are the host's (network_game_distributed_client).
    fn distributed_client(&self) -> bool;
    /// The unit's left hand: where it is and its node
    /// (object_get_marker_by_name).
    fn left_hand(&self) -> (Vec3, i16);

    /// Takes the object off its parent (object_detach).
    fn detach(&mut self, object: u32);
    /// Wakes the object (object_activate).
    fn activate(&mut self, object: u32);
    /// Puts the object back in the map (object_reconnect_to_map).
    fn reconnect(&mut self, object: u32);
    /// The unit's seat label and the weapon's, or unarmed's for none
    /// (unit_set_or_test_seat_and_weapon_label, setting).
    fn set_weapon_label(&mut self, weapon: Option<u32>);
    /// Puts the weapon in the unit's hand, at the hand marker of its seat
    /// and weapon class, by its grip (object_attach_to_marker).
    fn attach_to_hand(&mut self, weapon: u32);
    /// Readies the weapon (weapon_ready).
    fn ready(&mut self, weapon: u32);
    /// Unzooms the unit, its inventory's zoom and the rest (unit_unzoom).
    fn unzoom(&mut self, inventory: &mut UnitInventory<u32>);
    /// The item is no longer carried (item_in_unit_inventory, of none).
    fn released(&mut self, item: u32);
    /// Puts the item at the unit's left hand marker
    /// (object_attach_to_marker).
    fn attach_to_left_hand(&mut self, item: u32);
    /// The item thrown: still, it ignores the unit, then is given the
    /// velocity (item_accelerate).
    fn throw(&mut self, item: u32, velocity: Vec3);
    /// Moves the object inside the map's bsp from `from`, answering
    /// whether it could be (object_force_inside_bsp).
    fn force_inside_bsp(&mut self, object: u32, from: Vec3) -> bool;
    /// A new grenade of the definition, at `position` facing `forward` and
    /// `up`, none if it can't be made (object_placement_data_new,
    /// object_new).
    fn create_grenade(
        &mut self,
        definition: Option<u32>,
        position: Vec3,
        forward: Vec3,
        up: Vec3,
    ) -> Option<u32>;
    /// Holds the object at the unit's node (object_attach_to_node).
    fn attach_to_node(&mut self, object: u32, node: i16);
    /// The grenade of the kind thrown, for the host's damage reports
    /// (network_damage_note_grenade).
    fn note_grenade(&mut self, kind: Option<usize>);
    /// Spawns actors of the variant from the unit, thrown at `velocity`,
    /// answering how many were (actors_spawn_from_unit).
    fn spawn_actors(&mut self, variant: u32, count: i16, velocity: f32) -> i16;
}

/// Readies the weapon the unit wants: the one in hand put away (taken off,
/// out of the map, woken, hidden and carried) if it can be, then, with
/// none in hand, the wanted one labelled, put in the map, shown, held,
/// used now and readied, or the unit labelled unarmed; then the unit is
/// unzoomed (`unit_ready_desired_weapon`).
pub fn ready_desired_weapon(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl HandWorld,
    immediate: bool,
) {
    let desired = inventory.weapon(inventory.desired_weapon).copied();
    if let Some(current) = inventory
        .current()
        .copied()
        .filter(|current| world.put_away(*current, immediate))
    {
        world.detach(current);
        world.disconnect(current);
        world.activate(current);
        world.set_visible(current, false);
        world.carried(current);
        inventory.current_weapon = None;
    }
    if inventory.current_weapon.is_none() {
        if let Some(weapon) = desired {
            world.set_weapon_label(Some(weapon));
            world.reconnect(weapon);
            world.set_visible(weapon, true);
            world.attach_to_hand(weapon);
            inventory.current_weapon = inventory.desired_weapon;
            if let Some(used) = inventory
                .current_weapon
                .and_then(|slot| inventory.last_used.get_mut(slot))
            {
                *used = world.game_time();
            }
            world.ready(weapon);
        } else {
            world.set_weapon_label(None);
            inventory.current_weapon = None;
        }
    }
    world.unzoom(inventory);
}

/// After a change of seat, the unit wants the weapon in hand if it can
/// still use it, or the next it can, and readies it at once
/// (`unit_adjust_for_seat_change`).
pub fn adjust_for_seat_change(inventory: &mut UnitInventory<u32>, world: &mut impl HandWorld) {
    inventory.desired_weapon = next_weapon_index(inventory, world, inventory.current_weapon, 0);
    ready_desired_weapon(inventory, world, true);
}

/// Drops an item from the unit: one riding nothing is put in the map,
/// shown and put at its left hand first; it is let go, taken off and
/// thrown within a cone of the unit's aim at a speed at random, plus the
/// unit's velocity; it is deleted when it can't be put inside the bsp
/// outside a game engine's game, and when the unit doesn't drop items
/// (`unit_drop_item`).
pub fn drop_item(
    world: &mut impl HandWorld,
    random: &mut RandomSeed,
    item: u32,
    aiming: Vec3,
    doesnt_drop_items: bool,
) {
    if !world.has_parent(item) {
        world.reconnect(item);
        world.set_visible(item, true);
        world.attach_to_left_hand(item);
    }
    world.released(item);
    world.detach(item);
    let direction = random.vector_in_cone(aiming, 0.0, DROP_CONE);
    let speed = random.next_real_range(DROP_SPEED.0, DROP_SPEED.1);
    let velocity = direction * speed + world.unit_velocity();
    world.throw(item, velocity);
    let camera = world.camera_position();
    if !world.force_inside_bsp(item, camera) && !world.engine_running() {
        world.delete(item);
    }
    if doesnt_drop_items {
        world.delete(item);
    }
}

/// A grenade made in the unit's left hand as it throws: one of the kind it
/// throws counted off (a distributed client's only while it has one) unless
/// a cheat, the game engine or its actor gives it infinite grenades; the
/// grenade made of the kind's `projectile` at the hand, facing its aim, and
/// held there. The grenade made, its throw now in hand; none, and its
/// throw ending (`unit_throw_grenade_move_to_hand`).
pub fn throw_grenade_move_to_hand(
    inventory: &mut UnitInventory<u32>,
    world: &mut impl HandWorld,
    projectile: Option<u32>,
    aiming: Vec3,
) -> Option<u32> {
    let counted = (!world.has_player()
        || !world.cheat_infinite_ammo() && !world.engine_infinite_grenades())
        && (!world.has_actor() || !world.actor_unlimited_grenades());
    let kind = inventory.current_grenade;
    if counted {
        let client = world.distributed_client();
        if let Some(count) = kind.and_then(|kind| inventory.grenades.get_mut(kind)) {
            if !client || *count > 0 {
                *count = count.wrapping_sub(1);
            }
        }
    }
    let (position, node) = world.left_hand();
    let mut up = perpendicular(aiming);
    normalize(&mut up);
    let grenade = world.create_grenade(projectile, position, aiming, up)?;
    world.attach_to_node(grenade, node);
    world.note_grenade(kind);
    Some(grenade)
}

/// What a unit's definition spawns as it dies: the actor variant, the
/// least and most of it, and the speed they are thrown at, a second's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpawnedActors {
    /// The actor variant (`spawned_actor_variant`).
    pub variant: Option<u32>,
    /// The least spawned (`spawn_actor_lower_bound`).
    pub lower: i16,
    /// The most spawned (`spawn_actor_upper_bound`).
    pub upper: i16,
    /// Their speed, in world units a second (`spawn_throw_velocity`).
    pub velocity: f32,
}

/// The actors a unit spawns, once: a number from the least to the most at
/// random, spawned from it if more than none; answers how many were
/// (`unit_test_spawning`).
pub fn test_spawning(
    spawned: &mut bool,
    actors: &SpawnedActors,
    world: &mut impl HandWorld,
    random: &mut RandomSeed,
) -> i16 {
    let mut count = 0;
    if *spawned {
        return 0;
    }
    if let Some(variant) = actors.variant {
        count = random.next_range(actors.lower, actors.upper.wrapping_add(1));
        if count > 0 {
            count = world.spawn_actors(variant, count, actors.velocity * TICK);
        }
        *spawned = true;
    }
    count
}

/// A vector at right angles to `v` (`perpendicular3d`).
fn perpendicular(v: Vec3) -> Vec3 {
    let (i, j, k) = (v.x.abs(), v.y.abs(), v.z.abs());
    if i <= j && i <= k {
        Vec3::new(0.0, v.z, -v.y)
    } else if j <= k {
        Vec3::new(-v.z, 0.0, v.x)
    } else {
        Vec3::new(v.y, -v.x, 0.0)
    }
}
