//! The engine's unit picking up, dropping and deleting its weapons,
//! grenades and equipment against the game's: what it does to the world's
//! objects, in order, what it answers and what it leaves in the unit's
//! inventory, as tools/engine_unit_items_reference.py records the game's
//! code doing it over random units and worlds.

use std::error::Error;
use std::fmt::Write as _;

use opence_sim::unit_inventory::UnitInventory;
use opence_sim::unit_items::{
    add_equipment, add_grenade, add_initial_weapons, add_weapon, can_use_weapon,
    delete_all_weapons, drop_current_equipment, drop_current_weapon, drop_grenades,
    drop_inventory_weapons, AddWeapon, GrenadeGlobals, ItemWorld,
};
use opence_sim::unit_life::UnitEquipment;

const REFERENCE: &str = include_str!("unit_items_reference.txt");

type Failure = Box<dyn Error>;

/// The fake world's objects (the unit is the first).
const OBJECTS: usize = 16;
/// The first of the objects the functions make.
const FIRST_NEW: usize = 12;
const CONNECTED_TO_MAP: u32 = 1 << 11;
const INVISIBLE: u32 = 1 << 0;
const WEAPON: i32 = 2;

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

/// An index as the game writes it, -1 for none.
fn game<T: ToString>(value: Option<T>) -> String {
    value.map_or_else(|| "-1".to_owned(), |value| value.to_string())
}

fn flag(field: &str) -> Result<bool, Failure> {
    Ok(field.parse::<i32>()? != 0)
}

/// One object of the fake world and its answers.
#[derive(Default)]
struct Object {
    definition: i32,
    kind: i32,
    flags: u32,
    parent: bool,
    fired: bool,
    puts_away: bool,
    seat: bool,
    engine_allows: bool,
    picks_up: bool,
    grenade_type: i16,
}

/// The fake world, writing down what is done to it as the game's does.
struct World {
    objects: Vec<Object>,
    local_player: bool,
    engine_running: bool,
    failures: u32,
    news: u32,
    next_new: usize,
    calls: String,
}

impl World {
    fn get(&self, object: u32) -> Option<&Object> {
        self.objects.get(usize::try_from(object).ok()?)
    }

    fn ask(&self, object: u32, question: impl Fn(&Object) -> bool) -> bool {
        self.get(object).is_some_and(question)
    }

    fn log(&mut self, text: std::fmt::Arguments<'_>) {
        // writing to a string can't fail
        let _ = self.calls.write_fmt(text);
    }
}

fn bit(value: bool) -> i32 {
    i32::from(value)
}

impl ItemWorld for World {
    fn is_weapon(&self, object: u32) -> bool {
        self.ask(object, |object| object.kind == WEAPON)
    }
    fn connected_to_map(&self, object: u32) -> bool {
        self.ask(object, |object| object.flags & CONNECTED_TO_MAP != 0)
    }
    fn invisible(&self, object: u32) -> bool {
        self.ask(object, |object| object.flags & INVISIBLE != 0)
    }
    fn has_parent(&self, object: u32) -> bool {
        self.ask(object, |object| object.parent)
    }
    fn definition(&self, object: u32) -> u32 {
        self.get(object)
            .and_then(|object| u32::try_from(object.definition).ok())
            .unwrap_or(u32::MAX)
    }
    fn grenade_type(&self, equipment: u32) -> i16 {
        self.get(equipment).map_or(-1, |object| object.grenade_type)
    }
    fn seat_allows(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.seat)
    }
    fn engine_allows_pick_up(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.engine_allows)
    }
    fn engine_picking_up(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.picks_up)
    }
    fn must_be_readied(&self, _weapon: u32) -> bool {
        false
    }
    fn can_be_fired(&self, weapon: u32) -> bool {
        self.ask(weapon, |object| object.fired)
    }
    fn local_player(&self) -> bool {
        self.local_player
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
        if let Some(object) = usize::try_from(object)
            .ok()
            .and_then(|index| self.objects.get_mut(index))
        {
            object.flags &= !CONNECTED_TO_MAP;
        }
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
        let fails = self.news < 32 && self.failures & (1 << self.news) != 0;
        let mut made = None;
        if !fails && self.next_new < OBJECTS {
            let index = self.next_new;
            self.next_new += 1;
            if let Some(object) = self.objects.get_mut(index) {
                object.definition = definition.map_or(-1, |definition| definition as i32);
            }
            made = u32::try_from(index).ok();
        }
        self.news += 1;
        self.log(format_args!(" new {} {}", game(definition), game(made)));
        made
    }
}

/// The unit's inventory, as the trace writes it, with its equipment and
/// whether its trigger is down.
fn unit(fields: &[&str]) -> Result<(UnitInventory<u32>, UnitEquipment, bool), Failure> {
    let [current, desired, slots @ .., grenade0, grenade1, equipment, _flags, control] = fields
    else {
        return Err("a unit missing".into());
    };
    let mut inventory = UnitInventory::<u32> {
        current_weapon: index(current)?,
        desired_weapon: index(desired)?,
        ..UnitInventory::default()
    };
    if slots.len() != inventory.weapons.len() {
        return Err("a unit's slots missing".into());
    }
    for (slot, field) in slots.iter().enumerate() {
        let (weapon, last_used) = field.split_once(':').ok_or("a slot")?;
        inventory.weapons[slot] = object(weapon)?;
        inventory.last_used[slot] = last_used.parse()?;
    }
    inventory.grenades = [grenade0.parse()?, grenade1.parse()?];
    let trigger_down = control.parse::<u32>()? != 0;
    Ok((inventory, UnitEquipment(object(equipment)?), trigger_down))
}

fn objects(fields: &[&str]) -> Result<Vec<Object>, Failure> {
    let mut objects = vec![Object::default()];
    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let [definition, kind, flags, parent, fired, puts_away, seat, allows, picks_up, grenade] =
            parts[..]
        else {
            return Err(format!("an object: {field}").into());
        };
        objects.push(Object {
            definition: definition.parse()?,
            kind: kind.parse()?,
            flags: flags.parse()?,
            parent: parent != "-1",
            fired: flag(fired)?,
            puts_away: flag(puts_away)?,
            seat: flag(seat)?,
            engine_allows: flag(allows)?,
            picks_up: flag(picks_up)?,
            grenade_type: grenade.parse()?,
        });
    }
    if objects.len() != OBJECTS {
        return Err("objects missing".into());
    }
    Ok(objects)
}

/// The unit after, as the trace writes it.
fn after(inventory: &UnitInventory<u32>, equipment: UnitEquipment) -> String {
    let mut text = format!(
        "{} {}",
        game(inventory.current_weapon),
        game(inventory.desired_weapon)
    );
    for (weapon, last_used) in inventory.weapons.iter().zip(inventory.last_used) {
        text.push_str(&format!(" {}:{last_used}", game(*weapon)));
    }
    text.push_str(&format!(
        " {} {} {}",
        inventory.grenades[0],
        inventory.grenades[1],
        game(equipment.current())
    ));
    text
}

fn mode(argument: i32) -> Result<AddWeapon, Failure> {
    match argument {
        0 => Ok(AddWeapon::Normal),
        1 => Ok(AddWeapon::Starting),
        2 => Ok(AddWeapon::Replace),
        _ => Err("an add mode".into()),
    }
}

/// One case run by the engine: its calls, answer and unit after.
fn run(line: &str) -> Result<String, Failure> {
    let parts: Vec<&str> = line.split('|').map(str::trim).collect();
    let [head, unit_text, objects_text, game_text, ..] = parts[..] else {
        return Err("a case missing".into());
    };
    let head: Vec<&str> = head.split_whitespace().collect();
    let [name, argument, second] = head[..] else {
        return Err("a case's head".into());
    };
    let argument: i32 = argument.parse()?;
    let second: i32 = second.parse()?;
    let unit_fields: Vec<&str> = unit_text.split_whitespace().collect();
    let (mut inventory, mut equipment, trigger_down) = unit(&unit_fields)?;
    let mut objects = objects(&objects_text.split_whitespace().collect::<Vec<_>>())?;
    let game_fields: Vec<&str> = game_text.split_whitespace().collect();
    let [player, local, running, grenade_count, max0, max1, item0, item1, failures, initial_count, initial @ ..] =
        game_fields.as_slice()
    else {
        return Err("a game missing".into());
    };
    let mut grenades = vec![
        GrenadeGlobals {
            maximum_count: max0.parse()?,
            item: object(item0)?,
        },
        GrenadeGlobals {
            maximum_count: max1.parse()?,
            item: object(item1)?,
        },
    ];
    let item = u32::try_from(argument)?;
    if matches!(name, "add_grenade" | "add_equipment") {
        if let Some(object) = objects.get_mut(usize::try_from(item)?) {
            object.definition = 100 + argument;
        }
    }
    // the objects the functions make are weapons in the map
    for object in objects.iter_mut().skip(FIRST_NEW) {
        object.kind = WEAPON;
        object.flags = CONNECTED_TO_MAP;
    }
    let mut world = World {
        objects,
        local_player: *player != "-1" && *local != "-1",
        engine_running: flag(running)?,
        failures: failures.parse()?,
        news: 0,
        next_new: FIRST_NEW,
        calls: String::new(),
    };
    if name != "drop_grenades" {
        grenades.truncate(grenade_count.parse()?);
    }
    let initial_count: usize = initial_count.parse()?;
    let initial: Vec<Option<u32>> = initial
        .iter()
        .take(initial_count)
        .map(|field| object(field))
        .collect::<Result<_, _>>()?;
    let answer = match name {
        "delete_all_weapons" => {
            delete_all_weapons(&mut inventory, &mut world);
            false
        }
        "add_grenade" => add_grenade(&mut inventory, &mut world, item, &grenades),
        "add_equipment" => add_equipment(&mut equipment, &mut world, item, i16::try_from(second)?),
        "drop_current_equipment" => {
            drop_current_equipment(&mut equipment, &mut world);
            false
        }
        "can_use_weapon" => can_use_weapon(&world, item),
        "drop_inventory_weapons" => {
            drop_inventory_weapons(&mut inventory, &mut world);
            false
        }
        "drop_grenades" => {
            drop_grenades(&mut inventory, &mut world, &grenades);
            false
        }
        "drop_current_weapon" => drop_current_weapon(&mut inventory, &mut world, argument != 0),
        "add_weapon" => add_weapon(
            &mut inventory,
            &mut world,
            item,
            mode(second)?,
            trigger_down,
        ),
        "add_initial_weapons" => {
            add_initial_weapons(&mut inventory, &mut world, &initial);
            false
        }
        _ => return Err(format!("an unknown function {name}").into()),
    };
    Ok(format!(
        "{} -> {} | {}",
        world.calls.trim_start(),
        i32::from(answer),
        after(&inventory, equipment)
    ))
}

#[test]
fn the_engine_handles_a_units_items_as_the_game_does() -> Result<(), Failure> {
    let mut cases = 0;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let (left, right) = line.rsplit_once(" -> ").ok_or("a case without an answer")?;
        let calls = left.rsplit_once('|').map_or("", |(_, calls)| calls).trim();
        let expected = format!("{calls} -> {right}");
        let expected = expected.trim_start();
        let got = run(line)?;
        assert_eq!(got.trim_start(), expected, "{line}");
        cases += 1;
    }
    assert_eq!(cases, 12_000);
    Ok(())
}
