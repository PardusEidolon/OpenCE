//! The engine's unit readying the weapon it wants, dropping an item, making
//! a grenade in its hand and spawning actors against the game's: what it
//! does to the world's objects, in order, what it draws from the game's seed,
//! what it answers and what it leaves in the unit, as
//! tools/engine_unit_hands_reference.py records the game's code doing it
//! over random units and worlds.

use std::error::Error;
use std::fmt::Write as _;

use glam::Vec3;
use opence_sim::random::RandomSeed;
use opence_sim::unit_hands::{
    adjust_for_seat_change, drop_item, ready_desired_weapon, test_spawning,
    throw_grenade_move_to_hand, HandWorld, SpawnedActors, GRENADE_THROW_IN_HAND,
};
use opence_sim::unit_inventory::UnitInventory;
use opence_sim::unit_items::ItemWorld;
use opence_sim::unit_questions::GRENADE_THROW_ENDING;

const REFERENCE: &str = include_str!("unit_hands_reference.txt");

type Failure = Box<dyn Error>;

/// The fake world's objects (the unit is the first).
const OBJECTS: usize = 16;
/// The animation seats' weapon classes.
const WEAPON_CLASSES: u32 = 3;
/// `_unit_spawned_actors_bit`.
const SPAWNED_ACTORS: u32 = 1 << 17;
/// `_unit_doesnt_drop_items_bit`.
const DOESNT_DROP_ITEMS: u32 = 1 << 20;
/// `_new_object_never_automatically_delete_bit`, the flag each grenade is
/// made with.
const NEVER_AUTOMATICALLY_DELETE: u32 = 1 << 1;

/// The game's index (or -1) as an index.
fn index(field: &str) -> Result<Option<usize>, Failure> {
    let value: i32 = field.parse()?;
    Ok(usize::try_from(value).ok())
}

/// The game's object index (or -1) as an object.
fn object(field: &str) -> Result<Option<u32>, Failure> {
    let value: i32 = field.parse()?;
    Ok(u32::try_from(value).ok())
}

/// A zoom level (or -1).
fn zoom(field: &str) -> Result<Option<u8>, Failure> {
    let value: i32 = field.parse()?;
    Ok(u8::try_from(value).ok())
}

/// An index as the game writes it, -1 for none.
fn game<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "-1".to_owned(), |value| value.to_string())
}

fn flag(field: &str) -> Result<bool, Failure> {
    Ok(field.parse::<i32>()? != 0)
}

fn bit(value: bool) -> i32 {
    i32::from(value)
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    let [i, j, k] = fields else {
        return Err("a vector missing".into());
    };
    Ok(Vec3::new(i.parse()?, j.parse()?, k.parse()?))
}

/// A vector's bits, as the trace writes the calls'.
fn bits(v: Vec3) -> String {
    format!(
        "{:08x} {:08x} {:08x}",
        v.x.to_bits(),
        v.y.to_bits(),
        v.z.to_bits()
    )
}

/// One object of the fake world and its answers.
#[derive(Default)]
struct Object {
    parent: bool,
    puts_away: bool,
    seat: bool,
    engine_allows: bool,
    must_be_readied: bool,
    inside_bsp: bool,
}

/// The fake world, writing down what is done to it as the game's does.
#[derive(Default)]
struct World {
    objects: Vec<Object>,
    seat: i16,
    weapon_class: u32,
    game_time: i32,
    engine_running: bool,
    cheat: bool,
    engine_infinite: bool,
    actor_unlimited: bool,
    client: bool,
    player: bool,
    actor: bool,
    unit_velocity: Vec3,
    camera: Vec3,
    hand: Vec3,
    node: i16,
    new_grenade: Option<u32>,
    spawn_cap: i16,
    calls: String,
}

impl World {
    fn ask(&self, object: u32, question: impl Fn(&Object) -> bool) -> bool {
        usize::try_from(object)
            .ok()
            .and_then(|index| self.objects.get(index))
            .is_some_and(question)
    }

    fn log(&mut self, text: std::fmt::Arguments<'_>) {
        // writing to a string can't fail
        let _ = self.calls.write_fmt(text);
    }
}

impl ItemWorld for World {
    fn is_weapon(&self, _object: u32) -> bool {
        true
    }
    fn connected_to_map(&self, _object: u32) -> bool {
        true
    }
    fn invisible(&self, _object: u32) -> bool {
        false
    }
    fn has_parent(&self, object: u32) -> bool {
        self.ask(object, |object| object.parent)
    }
    fn definition(&self, object: u32) -> u32 {
        object
    }
    fn grenade_type(&self, _equipment: u32) -> i16 {
        -1
    }
    fn seat_allows(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.seat)
    }
    fn engine_allows_pick_up(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.engine_allows)
    }
    fn engine_picking_up(&self, _weapon: u32) -> bool {
        false
    }
    fn must_be_readied(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.must_be_readied)
    }
    fn can_be_fired(&self, _weapon: u32) -> bool {
        false
    }
    fn local_player(&self) -> bool {
        false
    }
    fn engine_running(&self) -> bool {
        self.engine_running
    }
    fn creating_host_object(&self) -> bool {
        false
    }
    fn delete(&mut self, object: u32) {
        self.log(format_args!(" delete {object}"));
    }
    fn disconnect(&mut self, object: u32) {
        self.log(format_args!(" disconnect {object}"));
    }
    fn set_visible(&mut self, object: u32, visible: bool) {
        self.log(format_args!(" visible {object} {}", bit(visible)));
    }
    fn handle_pickup(&mut self, equipment: u32) {
        self.log(format_args!(" pickup {equipment}"));
    }
    fn carried(&mut self, item: u32) {
        self.log(format_args!(" carried {item} 0"));
    }
    fn drop_item(&mut self, item: u32) {
        self.log(format_args!(" drop {item}"));
    }
    fn put_away(&mut self, weapon: u32, immediate: bool) -> bool {
        self.log(format_args!(" put_away {weapon} {}", bit(immediate)));
        self.ask(weapon, |object| object.puts_away)
    }
    fn first_person_message(&mut self, message: i32) {
        self.log(format_args!(" message {message}"));
    }
    fn set_player_desired_weapon(&mut self, slot: usize) {
        self.log(format_args!(" player_desired {slot}"));
    }
    fn create(&mut self, definition: Option<u32>) -> Option<u32> {
        self.log(format_args!(" new {}", game(definition)));
        None
    }
}

impl HandWorld for World {
    fn game_time(&self) -> i32 {
        self.game_time
    }
    fn unit_velocity(&self) -> Vec3 {
        self.unit_velocity
    }
    fn camera_position(&self) -> Vec3 {
        self.camera
    }
    fn has_player(&self) -> bool {
        self.player
    }
    fn cheat_infinite_ammo(&self) -> bool {
        self.cheat
    }
    fn engine_infinite_grenades(&self) -> bool {
        self.engine_infinite
    }
    fn has_actor(&self) -> bool {
        self.actor
    }
    fn actor_unlimited_grenades(&self) -> bool {
        self.actor_unlimited
    }
    fn distributed_client(&self) -> bool {
        self.client
    }
    fn left_hand(&self) -> (Vec3, i16) {
        (self.hand, self.node)
    }
    fn detach(&mut self, object: u32) {
        self.log(format_args!(" detach {object}"));
    }
    fn activate(&mut self, object: u32) {
        self.log(format_args!(" activate {object}"));
    }
    fn reconnect(&mut self, object: u32) {
        self.log(format_args!(" reconnect {object}"));
    }
    fn set_weapon_label(&mut self, weapon: Option<u32>) {
        match weapon {
            Some(weapon) => {
                self.log(format_args!(" label w{weapon}"));
                self.weapon_class = weapon % WEAPON_CLASSES;
            }
            None => {
                self.log(format_args!(" label unarmed"));
                self.weapon_class = 0;
            }
        }
    }
    fn attach_to_hand(&mut self, weapon: u32) {
        let (seat, class) = (self.seat, self.weapon_class);
        self.log(format_args!(
            " attach {weapon} hand{seat}{class}:grip{seat}{class}"
        ));
    }
    fn ready(&mut self, weapon: u32) {
        self.log(format_args!(" ready {weapon}"));
    }
    fn unzoom(&mut self, inventory: &mut UnitInventory<u32>) {
        self.log(format_args!(" unzoom"));
        inventory.current_zoom = None;
        inventory.desired_zoom = None;
    }
    fn released(&mut self, item: u32) {
        self.log(format_args!(" carried {item} -1"));
    }
    fn attach_to_left_hand(&mut self, item: u32) {
        self.log(format_args!(" attach {item} left hand:"));
    }
    fn throw(&mut self, item: u32, velocity: Vec3) {
        // still and ignoring the unit, then thrown
        self.log(format_args!(" throw {item} 0 1 {}", bits(velocity)));
    }
    fn force_inside_bsp(&mut self, object: u32, from: Vec3) -> bool {
        self.log(format_args!(" bsp {object} {}", bits(from)));
        self.ask(object, |object| object.inside_bsp)
    }
    fn create_grenade(
        &mut self,
        definition: Option<u32>,
        position: Vec3,
        forward: Vec3,
        up: Vec3,
    ) -> Option<u32> {
        let made = self.new_grenade;
        self.log(format_args!(
            " grenade {} {NEVER_AUTOMATICALLY_DELETE} {} {} {} {}",
            game(definition),
            bits(position),
            bits(forward),
            bits(up),
            game(made)
        ));
        made
    }
    fn attach_to_node(&mut self, object: u32, node: i16) {
        self.log(format_args!(" node {object} {node}"));
    }
    fn note_grenade(&mut self, kind: Option<usize>) {
        self.log(format_args!(" note {}", game(kind)));
    }
    fn spawn_actors(&mut self, variant: u32, count: i16, velocity: f32) -> i16 {
        let spawned = count.min(self.spawn_cap);
        self.log(format_args!(
            " spawn {variant} {count} {:08x} {spawned}",
            velocity.to_bits()
        ));
        spawned
    }
}

/// The unit as the trace writes it: its inventory, flags and aim, with the
/// world's seat and whose unit it is.
struct Unit {
    inventory: UnitInventory<u32>,
    flags: u32,
    aiming: Vec3,
}

fn unit(fields: &[&str], world: &mut World) -> Result<Unit, Failure> {
    let [current, desired, s0, s1, s2, s3, grenade, g0, g1, zoom_, desired_zoom, player, actor, flags, seat, aim @ ..] =
        fields
    else {
        return Err("a unit missing".into());
    };
    let mut inventory = UnitInventory::<u32> {
        current_weapon: index(current)?,
        desired_weapon: index(desired)?,
        current_grenade: index(grenade)?,
        grenades: [g0.parse()?, g1.parse()?],
        current_zoom: zoom(zoom_)?,
        desired_zoom: zoom(desired_zoom)?,
        ..UnitInventory::default()
    };
    for (slot, field) in [s0, s1, s2, s3].iter().enumerate() {
        let (weapon, last_used) = field.split_once(':').ok_or("a slot")?;
        inventory.weapons[slot] = object(weapon)?;
        inventory.last_used[slot] = last_used.parse()?;
    }
    world.player = *player != "-1";
    world.actor = *actor != "-1";
    world.seat = seat.parse()?;
    Ok(Unit {
        inventory,
        flags: flags.parse()?,
        aiming: vector(aim)?,
    })
}

fn objects(fields: &[&str]) -> Result<Vec<Object>, Failure> {
    let mut objects = vec![Object::default()];
    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let [parent, puts_away, seat, allows, readied, bsp] = parts[..] else {
            return Err(format!("an object: {field}").into());
        };
        objects.push(Object {
            parent: parent != "-1",
            puts_away: flag(puts_away)?,
            seat: flag(seat)?,
            engine_allows: flag(allows)?,
            must_be_readied: flag(readied)?,
            inside_bsp: flag(bsp)?,
        });
    }
    if objects.len() != OBJECTS {
        return Err("objects missing".into());
    }
    Ok(objects)
}

/// The rest of the game, into the world; the grenades' projectiles, the
/// actors spawned and the seed.
fn rest(
    fields: &[&str],
    world: &mut World,
) -> Result<([Option<u32>; 2], SpawnedActors, RandomSeed), Failure> {
    let [time, running, cheat, engine_infinite, actor_unlimited, client, velocity @ .., p0, p1, new, variant, lower, upper, speed, cap, seed] =
        fields
    else {
        return Err("a game missing".into());
    };
    let [velocity @ .., node] = velocity else {
        return Err("a hand missing".into());
    };
    let (velocity, rest) = velocity.split_at_checked(3).ok_or("a velocity")?;
    let (camera, hand) = rest.split_at_checked(3).ok_or("a camera")?;
    world.game_time = time.parse()?;
    world.engine_running = flag(running)?;
    world.cheat = flag(cheat)?;
    world.engine_infinite = flag(engine_infinite)?;
    world.actor_unlimited = flag(actor_unlimited)?;
    world.client = flag(client)?;
    world.unit_velocity = vector(velocity)?;
    world.camera = vector(camera)?;
    world.hand = vector(hand)?;
    world.node = node.parse()?;
    world.new_grenade = object(new)?;
    world.spawn_cap = cap.parse()?;
    let actors = SpawnedActors {
        variant: object(variant)?,
        lower: lower.parse()?,
        upper: upper.parse()?,
        velocity: speed.parse()?,
    };
    Ok((
        [object(p0)?, object(p1)?],
        actors,
        RandomSeed(seed.parse()?),
    ))
}

/// The unit after, as the trace writes it.
fn after(unit: &Unit, grenade_object: Option<u32>, throw_state: i8, random: RandomSeed) -> String {
    let inventory = &unit.inventory;
    let mut text = format!(
        "{} {}",
        game(inventory.current_weapon),
        game(inventory.desired_weapon)
    );
    for (weapon, last_used) in inventory.weapons.iter().zip(inventory.last_used) {
        text.push_str(&format!(" {}:{last_used}", game(*weapon)));
    }
    text.push_str(&format!(
        " {} {} {} {} {} {} {throw_state} {}",
        inventory.grenades[0],
        inventory.grenades[1],
        game(inventory.current_zoom),
        game(inventory.desired_zoom),
        game(grenade_object),
        unit.flags,
        random.0
    ));
    text
}

/// One case run by the engine: its calls, answer and unit after.
fn run(line: &str) -> Result<String, Failure> {
    let parts: Vec<&str> = line.split('|').map(str::trim).collect();
    let [head, unit_text, objects_text, game_text, ..] = parts[..] else {
        return Err("a case missing".into());
    };
    let head: Vec<&str> = head.split_whitespace().collect();
    let [name, argument] = head[..] else {
        return Err("a case's head".into());
    };
    let argument: u32 = argument.parse()?;
    let mut world = World::default();
    let unit_fields: Vec<&str> = unit_text.split_whitespace().collect();
    let mut unit = unit(&unit_fields, &mut world)?;
    let object_fields: Vec<&str> = objects_text.split_whitespace().collect();
    world.objects = objects(&object_fields)?;
    let game_fields: Vec<&str> = game_text.split_whitespace().collect();
    let (projectiles, actors, mut random) = rest(&game_fields, &mut world)?;
    let mut grenade_object = None;
    let mut throw_state = 0;
    let mut answer = 0;
    match name {
        "ready_desired_weapon" => {
            ready_desired_weapon(&mut unit.inventory, &mut world, argument != 0);
        }
        "adjust_for_seat_change" => adjust_for_seat_change(&mut unit.inventory, &mut world),
        "drop_item" => drop_item(
            &mut world,
            &mut random,
            argument,
            unit.aiming,
            unit.flags & DOESNT_DROP_ITEMS != 0,
        ),
        "throw_grenade_move_to_hand" => {
            let projectile = unit
                .inventory
                .current_grenade
                .and_then(|kind| projectiles.get(kind).copied())
                .flatten();
            grenade_object = throw_grenade_move_to_hand(
                &mut unit.inventory,
                &mut world,
                projectile,
                unit.aiming,
            );
            throw_state = if grenade_object.is_some() {
                GRENADE_THROW_IN_HAND
            } else {
                GRENADE_THROW_ENDING
            };
        }
        "test_spawning" => {
            let mut spawned = unit.flags & SPAWNED_ACTORS != 0;
            answer = test_spawning(&mut spawned, &actors, &mut world, &mut random);
            if spawned {
                unit.flags |= SPAWNED_ACTORS;
            }
        }
        _ => return Err(format!("an unknown function {name}").into()),
    }
    Ok(format!(
        "{} -> {answer} | {}",
        world.calls.trim_start(),
        after(&unit, grenade_object, throw_state, random)
    ))
}

#[test]
fn the_engine_handles_a_units_hands_as_the_game_does() -> Result<(), Failure> {
    let mut cases = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (left, right) = line.rsplit_once(" -> ").ok_or("a case without an answer")?;
        let calls = left.rsplit_once('|').map_or("", |(_, calls)| calls).trim();
        let expected = format!("{calls} -> {right}");
        let got = run(line)?;
        assert_eq!(got.trim_start(), expected.trim_start(), "{line}");
        cases += 1;
    }
    assert_eq!(cases, 10_000);
    Ok(())
}
