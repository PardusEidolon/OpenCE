//! Devices and their groups: machines and controls whose power and
//! position the scripts set, each accelerating toward its groups' values
//! at its definition's speeds, doors opening for the bipeds near them and
//! gears turning (`hs_library_devices.c`, `devices.c`, `device_machines.c`).

use std::collections::BTreeMap;

use glam::Vec3;
use opence_content::{
    Device, DeviceGroup as ScenarioDeviceGroup, Machine, ObjectDefinition, PlacementDevice, TagRef,
};
use opence_script::{Native, Value};
use opence_sim::device_animation::{locked, DeviceMotion, DevicePose};
use opence_sim::objects::ObjectHandle;

use crate::{boolean, index, real, Mission};

/// A device's speeds where its definition is not in the level: a whole
/// opening in about a second, its power at once.
fn fallback() -> Device {
    Device {
        maximum_power_acceleration: 1.0,
        maximum_power_velocity: 1.0,
        depowered_position_acceleration: 1.0 / 30.0,
        depowered_position_velocity: 1.0 / 30.0,
        powered_position_acceleration: 1.0 / 30.0,
        powered_position_velocity: 1.0 / 30.0,
        ..Device::default()
    }
}

/// The distance under which a door's activation radius is taken as none,
/// so that its bounding radius is used (`k_real_epsilon`).
const EPSILON: f32 = 1e-4;

/// The most device groups a level holds (`device_groups_data`'s 1024).
pub const MAXIMUM_DEVICE_GROUPS: usize = 1024;

/// A value devices share, their power or their position
/// (`device_group_datum`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceGroup {
    /// Its value, from 0 to 1.
    pub value: f32,
    /// Whether it changes only once more (then stays).
    pub once: bool,
    /// Whether it has changed since `once` was set.
    pub changed_once: bool,
    /// Whether it was made for one device and goes with it (a device
    /// placed without a group gets its own).
    pub runtime: bool,
}

/// A device's state.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceState {
    /// Its power and position and how they move.
    pub motion: DeviceMotion,
    /// The device group its power follows, or `-1`.
    pub power_group: i16,
    /// The device group its position follows, or `-1`.
    pub position_group: i16,
    /// Whether its position animation plays from its end.
    pub reversed: bool,
    /// Whether a player may not use it.
    pub not_usable: bool,
    /// Whether a machine does not operate by itself.
    pub manual: bool,
    /// Whether a machine opens from its back only.
    pub one_sided: bool,
    /// Whether a machine never shows as locked.
    pub never_appears_locked: bool,
    /// Its definition's device part, if the level has it.
    pub definition: Option<Device>,
    /// Its definition's bounding radius.
    pub bounding_radius: f32,
    /// The effects it started on the last tick, by its motion or by its
    /// power group's change.
    pub effects: Vec<TagRef>,
}

impl DeviceState {
    /// Its power, from 0 to 1.
    pub fn power(&self) -> f32 {
        self.motion.power
    }

    /// Its position, from closed (0) to open (1).
    pub fn position(&self) -> f32 {
        self.motion.position
    }

    /// Its machine part, if it is a machine.
    fn machine(&self) -> Option<Machine> {
        self.definition.as_ref().and_then(|device| device.machine)
    }

    /// Whether it is a machine, whose flags the scripts set
    /// (`machine_try_and_get`).
    pub fn is_machine(&self) -> bool {
        self.machine().is_some()
    }

    /// Whether it shows as locked, its position group as `group`.
    fn locked(&self, group: Option<&DeviceGroup>) -> bool {
        let grouped = self.position_group >= 0 && self.is_machine();
        locked(
            self.power(),
            self.position(),
            grouped.then_some((
                self.manual || self.one_sided,
                group.is_some_and(|group| group.once && group.changed_once),
                self.never_appears_locked,
            )),
        )
    }
}

/// The level's devices and their groups (`device_groups_data`, and the
/// devices of the object table, in its order).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Devices {
    // (a group's index is its place here; a deleted one leaves its place
    // empty, for the next new group)
    groups: Vec<Option<DeviceGroup>>,
    states: BTreeMap<ObjectHandle, DeviceState>,
}

impl Devices {
    /// A level's groups as it begins, before its devices are placed
    /// (`create_initial_device_groups`): each scenario group at its
    /// initial value, changing only once if its flag says so.
    pub fn of(groups: &[ScenarioDeviceGroup]) -> Self {
        let mut devices = Self::default();
        for group in groups.iter().take(MAXIMUM_DEVICE_GROUPS) {
            devices.group_new(group.initial_value, group.flags & 1 != 0, false);
        }
        devices
    }

    /// A new group in the first free place, or `-1` if there is none
    /// (`device_group_new`).
    fn group_new(&mut self, value: f32, once: bool, runtime: bool) -> i16 {
        let group = Some(DeviceGroup {
            value,
            once,
            changed_once: false,
            runtime,
        });
        let at = match self.groups.iter().position(Option::is_none) {
            Some(at) => at,
            None if self.groups.len() < MAXIMUM_DEVICE_GROUPS => {
                self.groups.push(None);
                self.groups.len() - 1
            }
            None => return -1,
        };
        let Ok(index) = i16::try_from(at) else {
            return -1;
        };
        if let Some(slot) = self.groups.get_mut(at) {
            *slot = group;
        }
        index
    }

    /// Deletes a group made for one device; the scenario's stay
    /// (`device_group_delete`).
    fn group_delete(&mut self, group: i16) {
        let slot = usize::try_from(group)
            .ok()
            .and_then(|at| self.groups.get_mut(at));
        if let Some(slot) = slot {
            if slot.is_some_and(|group| group.runtime) {
                *slot = None;
            }
        }
    }

    /// Every group, in the order of their indices.
    pub fn groups(&self) -> impl Iterator<Item = (i16, &DeviceGroup)> {
        self.groups
            .iter()
            .enumerate()
            .filter_map(|(at, group)| Some((i16::try_from(at).ok()?, group.as_ref()?)))
    }

    /// A group.
    pub fn group(&self, group: i16) -> Option<&DeviceGroup> {
        usize::try_from(group)
            .ok()
            .and_then(|at| self.groups.get(at))
            .and_then(Option::as_ref)
    }

    pub(crate) fn group_mut(&mut self, group: i16) -> Option<&mut DeviceGroup> {
        usize::try_from(group)
            .ok()
            .and_then(|at| self.groups.get_mut(at))
            .and_then(Option::as_mut)
    }

    /// A device.
    pub fn get(&self, handle: ObjectHandle) -> Option<&DeviceState> {
        self.states.get(&handle)
    }

    /// A device, to change.
    pub fn get_mut(&mut self, handle: ObjectHandle) -> Option<&mut DeviceState> {
        self.states.get_mut(&handle)
    }

    /// Every device, in the order of their handles.
    pub fn iter(&self) -> impl Iterator<Item = (ObjectHandle, &DeviceState)> {
        self.states.iter().map(|(&handle, state)| (handle, state))
    }

    /// Places a device of `definition` (`device_new`,
    /// `device_add_scenario_information`, `machine_place`): it follows its
    /// placement's groups, or groups of its own at its placement's
    /// values, and begins at their values.
    pub fn place(
        &mut self,
        handle: ObjectHandle,
        device: &PlacementDevice,
        definition: Option<&ObjectDefinition>,
    ) {
        self.place_device(
            handle,
            device,
            definition.and_then(|definition| definition.device.clone()),
            definition.map_or(0.0, |definition| definition.bounding_radius),
        );
    }

    /// Places a device of the device part `definition` and its bounding
    /// radius, as [`Devices::place`] does.
    pub fn place_device(
        &mut self,
        handle: ObjectHandle,
        device: &PlacementDevice,
        definition: Option<Device>,
        bounding_radius: f32,
    ) {
        self.delete(handle);
        let power_group = if device.power_group != -1 {
            device.power_group
        } else {
            let off = device.flags & 2 != 0;
            self.group_new(if off { 0.0 } else { 1.0 }, false, true)
        };
        let position_group = if device.position_group != -1 {
            device.position_group
        } else {
            let open = device.flags & 1 != 0;
            self.group_new(if open { 1.0 } else { 0.0 }, device.flags & 4 != 0, true)
        };
        let value = |group: i16| self.group(group).map_or(0.0, |group| group.value);
        let mut state = DeviceState {
            motion: DeviceMotion::at(value(power_group), value(position_group)),
            power_group,
            position_group,
            reversed: device.flags & 8 != 0,
            not_usable: device.flags & 16 != 0,
            manual: false,
            one_sided: false,
            never_appears_locked: false,
            definition,
            bounding_radius,
            effects: Vec::new(),
        };
        if state.is_machine() {
            state.manual = device.kind_flags & 1 != 0;
            state.one_sided = device.kind_flags & 2 != 0;
            state.never_appears_locked = device.kind_flags & 4 != 0;
        }
        self.states.insert(handle, state);
    }

    /// Takes a device out, and the groups made for it (`device_delete`).
    pub fn delete(&mut self, handle: ObjectHandle) {
        if let Some(state) = self.states.remove(&handle) {
            self.group_delete(state.power_group);
            self.group_delete(state.position_group);
        }
    }

    /// A group's value (`device_group_get_value`), 0 for none.
    pub fn group_value(&self, group: i16) -> f32 {
        self.group(group).map_or(0.0, |group| group.value)
    }

    /// Changes a group's value, unless it is its value or it has made its
    /// one change; each device whose power follows it starts its
    /// repowered or depowered effect. Whether it changed
    /// (`device_group_change`, `device_group_set_desired_value`: the
    /// engine has no co-op client, which leaves the devices to its host).
    pub fn group_change(&mut self, group: i16, desired: f32) -> bool {
        let desired = clamp(desired);
        let Some(slot) = self.group_mut(group) else {
            return false;
        };
        if slot.value == desired || (slot.once && slot.changed_once) {
            return false;
        }
        slot.value = desired;
        slot.changed_once = true;
        for state in self.states.values_mut() {
            if state.power_group == group {
                let effect = state
                    .definition
                    .as_ref()
                    .and_then(|device| device.power_effects[usize::from(desired > 0.0)].clone());
                state.effects.extend(effect);
            }
        }
        true
    }

    /// Sets a group's value, and its devices' power or position to it at
    /// once (`device_group_set_actual_value`).
    pub fn group_set_actual(&mut self, group: i16, value: f32) {
        let value = clamp(value);
        let Some(slot) = self.group_mut(group) else {
            return;
        };
        slot.value = value;
        for state in self.states.values_mut() {
            if state.power_group == group {
                state.motion.power = value;
                state.motion.power_velocity = 0.0;
            }
            if state.position_group == group {
                state.motion.position = value;
                state.motion.position_velocity = 0.0;
            }
        }
    }

    /// Lets a group change only once more, or as often as it is told; it
    /// has not changed since (`device_group_change_only_once_more_set`).
    pub fn group_change_only_once_more(&mut self, group: i16, once: bool) {
        if let Some(slot) = self.group_mut(group) {
            slot.once = once;
            slot.changed_once = false;
        }
    }

    /// Changes a device's position group; whether it changed
    /// (`device_set_desired_position`).
    pub fn set_desired_position(&mut self, handle: ObjectHandle, position: f32) -> bool {
        match self.get(handle) {
            Some(state) if state.position_group != -1 => {
                self.group_change(state.position_group, position)
            }
            _ => false,
        }
    }

    /// Sets a device's position group, and its devices, at once
    /// (`device_set_actual_position`).
    pub fn set_actual_position(&mut self, handle: ObjectHandle, position: f32) {
        if let Some(group) = self.get(handle).map(|state| state.position_group) {
            if group != -1 {
                self.group_set_actual(group, position);
            }
        }
    }

    /// Sets a device's power at once, as it is given, and changes its
    /// power group (`device_set_power`).
    pub fn set_power(&mut self, handle: ObjectHandle, power: f32) {
        if let Some(state) = self.get_mut(handle) {
            state.motion.power = power;
            let group = state.power_group;
            self.group_change(group, power);
        }
    }
}

/// A value from 0 to 1, as the game pins a group's (`PIN`, which keeps a
/// NaN as `clamp` does).
fn clamp(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

pub(crate) const NATIVES: &[(&str, Native<Mission>)] = &[
    ("device_get_position", device_get_position),
    ("device_get_power", device_get_power),
    ("device_set_position", device_set_position),
    (
        "device_set_position_immediate",
        device_set_position_immediate,
    ),
    ("device_set_power", device_set_power),
    ("device_group_get", device_group_get),
    ("device_group_set", device_group_set),
    ("device_group_set_immediate", device_group_set_immediate),
    (
        "device_group_change_only_once_more_set",
        device_group_change_only_once_more_set,
    ),
    ("device_one_sided_set", device_one_sided_set),
    (
        "device_operates_automatically_set",
        device_operates_automatically_set,
    ),
    (
        "device_set_never_appears_locked",
        device_set_never_appears_locked,
    ),
];

/// Moves each device on a tick (`device_update`, `machine_update`): its
/// power and position accelerate toward their groups' values, a gear
/// turns its group, a door opens for the bipeds near it and closes in its
/// time, through its group as a script would; then each is posed.
pub(crate) fn advance(mission: &mut Mission) {
    let tick = mission.game_tick();
    let bipeds = bipeds(mission);
    let handles: Vec<ObjectHandle> = mission.devices.iter().map(|(handle, _)| handle).collect();
    let fallback = fallback();
    for handle in handles {
        let transform = mission.sim.transform(handle);
        let devices = &mut mission.devices;
        let (power, position) = match devices.get(handle) {
            Some(state) => (
                devices.group_value(state.power_group),
                devices.group_value(state.position_group),
            ),
            None => continue,
        };
        let Some(device) = devices.get_mut(handle) else {
            continue;
        };
        let definition = device
            .definition
            .clone()
            .unwrap_or_else(|| fallback.clone());
        let machine = definition.machine;
        let gear = machine.is_some_and(|m| m.machine_type == Machine::GEAR);
        let update = device
            .motion
            .update(&definition, Some(power), (!gear).then_some(position));
        device.effects.clear();
        device.effects.extend(update.effects);
        let group = device.position_group;
        if gear {
            let turned = device.motion.turn_gear(&definition);
            if let Some(slot) = devices.group_mut(group) {
                slot.value = turned;
            }
        } else if let Some(machine) = machine.filter(|m| m.machine_type == Machine::DOOR) {
            // (a door looks for bipeds every fourth tick)
            let looks = (tick + u64::from(handle.index)) & 3 == 0 && !device.manual;
            let activated = looks
                && transform.is_some_and(|transform| {
                    let radius = if definition.automatic_activation_radius < EPSILON {
                        device.bounding_radius
                    } else {
                        definition.automatic_activation_radius
                    };
                    let forward = transform.rotation * Vec3::X;
                    let closed_one_side = device.one_sided && device.position() == 0.0;
                    bipeds.iter().any(|&(at, enemy)| {
                        let way = at - transform.position;
                        way.length() <= radius
                            && !(closed_one_side && !enemy && way.dot(forward) > 0.0)
                    })
                });
            if let Some(value) = device.motion.update_door(&machine, activated) {
                devices.group_change(group, value);
            }
        }
        let Some(device) = devices.get(handle) else {
            continue;
        };
        let pose = DevicePose {
            position: device.position(),
            power: device.power(),
            reversed: device.reversed,
            flags: definition.flags,
            exports: device
                .motion
                .exports(&definition, device.locked(devices.group(group))),
        };
        mission.sim.pose_device(handle, pose);
    }
}

/// The living bipeds that may open doors: where each stands, and whether
/// it is the players' enemy.
fn bipeds(mission: &Mission) -> Vec<(Vec3, bool)> {
    crate::actors::others(mission)
        .into_iter()
        .filter(|other| other.alive)
        .filter_map(|other| {
            let at = mission.sim.transform(other.unit)?.position;
            Some((at, mission.enemies(crate::actors::PLAYER_TEAM, other.team)))
        })
        .collect()
}

/// The device a script names.
fn handle(mission: &Mission, arguments: &[Value]) -> Option<ObjectHandle> {
    mission.handle(index(arguments, 0))
}

/// The machine a script names (`machine_try_and_get`): the machine flags
/// are set on machines only.
fn machine<'a>(mission: &'a mut Mission, arguments: &[Value]) -> Option<&'a mut DeviceState> {
    let handle = handle(mission, arguments)?;
    mission
        .devices
        .get_mut(handle)
        .filter(|state| state.is_machine())
}

/// The group a script names, or `-1`.
fn group(arguments: &[Value]) -> i16 {
    i16::try_from(index(arguments, 0)).unwrap_or(-1)
}

fn device_get_position(mission: &mut Mission, arguments: &[Value]) -> Value {
    let state = handle(mission, arguments).and_then(|handle| mission.devices.get(handle));
    Value::Real(state.map_or(0.0, DeviceState::position))
}

fn device_get_power(mission: &mut Mission, arguments: &[Value]) -> Value {
    let state = handle(mission, arguments).and_then(|handle| mission.devices.get(handle));
    Value::Real(state.map_or(0.0, DeviceState::power))
}

fn device_set_position(mission: &mut Mission, arguments: &[Value]) -> Value {
    let changed = handle(mission, arguments).is_some_and(|handle| {
        mission
            .devices
            .set_desired_position(handle, real(arguments, 1))
    });
    Value::Boolean(changed)
}

fn device_set_position_immediate(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = handle(mission, arguments) {
        mission
            .devices
            .set_actual_position(handle, real(arguments, 1));
    }
    Value::Void
}

fn device_set_power(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = handle(mission, arguments) {
        mission.devices.set_power(handle, real(arguments, 1));
    }
    Value::Void
}

fn device_group_get(mission: &mut Mission, arguments: &[Value]) -> Value {
    Value::Real(mission.devices.group_value(group(arguments)))
}

fn device_group_set(mission: &mut Mission, arguments: &[Value]) -> Value {
    let changed = mission
        .devices
        .group_change(group(arguments), real(arguments, 1));
    Value::Boolean(changed)
}

fn device_group_set_immediate(mission: &mut Mission, arguments: &[Value]) -> Value {
    mission
        .devices
        .group_set_actual(group(arguments), real(arguments, 1));
    Value::Void
}

fn device_group_change_only_once_more_set(mission: &mut Mission, arguments: &[Value]) -> Value {
    mission
        .devices
        .group_change_only_once_more(group(arguments), boolean(arguments, 1));
    Value::Void
}

fn device_one_sided_set(mission: &mut Mission, arguments: &[Value]) -> Value {
    let value = boolean(arguments, 1);
    if let Some(device) = machine(mission, arguments) {
        device.one_sided = value;
    }
    Value::Void
}

fn device_operates_automatically_set(mission: &mut Mission, arguments: &[Value]) -> Value {
    let value = !boolean(arguments, 1);
    if let Some(device) = machine(mission, arguments) {
        device.manual = value;
    }
    Value::Void
}

fn device_set_never_appears_locked(mission: &mut Mission, arguments: &[Value]) -> Value {
    let value = boolean(arguments, 1);
    if let Some(device) = machine(mission, arguments) {
        device.never_appears_locked = value;
    }
    Value::Void
}
