//! The engine's device groups against the game's: the 300 levels of
//! tools/engine_devices_reference.py (random scenario groups, devices
//! placed on them or on groups of their own, and 40 random script calls
//! each) must leave every group's value and flags, every device's power,
//! position, velocities and flags, each call's answer and each effect
//! started as the game's code leaves them.

use std::error::Error;

use opence_campaign::Devices;
use opence_content::{Device, DeviceGroup, Group, Machine, PlacementDevice, TagRef};
use opence_sim::objects::ObjectHandle;

const REFERENCE: &str = include_str!("devices_reference.txt");

type Failure = Box<dyn Error>;

/// The definitions of a level, the first four machines'.
const DEFINITIONS: usize = 8;

fn real(field: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(field, 16)?))
}

fn object(field: &str) -> Result<ObjectHandle, Failure> {
    Ok(ObjectHandle {
        index: field.parse()?,
        salt: 1,
    })
}

fn effect(field: &str) -> Result<Option<TagRef>, Failure> {
    let index: i32 = field.parse()?;
    Ok((index != -1).then(|| TagRef {
        group: Group(*b"effe"),
        path: index.to_string(),
    }))
}

/// A level's definitions and its scenario's groups, from its line.
fn level(fields: &[&str]) -> Result<(Vec<Device>, Vec<DeviceGroup>), Failure> {
    let [count, rest @ ..] = fields else {
        return Err("a level without its groups' count".into());
    };
    let count: usize = count.parse()?;
    let (effects, groups) = rest.split_at(DEFINITIONS.min(rest.len()));
    let definitions = effects
        .iter()
        .enumerate()
        .map(|(at, field)| {
            let (depowered, repowered) = field.split_once(':').ok_or("an effect pair")?;
            Ok(Device {
                machine: (at < 4).then(Machine::default),
                power_effects: [effect(depowered)?, effect(repowered)?],
                ..Device::default()
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let groups = groups
        .iter()
        .map(|field| {
            let (value, flags) = field.split_once(':').ok_or("a group pair")?;
            Ok(DeviceGroup {
                name: String::new(),
                initial_value: real(value)?,
                flags: flags.parse()?,
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    assert_eq!(groups.len(), count);
    Ok((definitions, groups))
}

/// The groups as the game prints them.
fn groups(devices: &Devices) -> String {
    devices
        .groups()
        .map(|(index, group)| {
            let flags = u8::from(group.once)
                | u8::from(group.changed_once) << 1
                | u8::from(group.runtime) << 2;
            format!(" {index}:{:08x}:{flags}", group.value.to_bits())
        })
        .collect()
}

/// The devices as the game prints them.
fn states(devices: &Devices) -> String {
    devices
        .iter()
        .map(|(handle, state)| {
            let motion = &state.motion;
            let flags = u8::from(state.reversed) | u8::from(state.not_usable) << 1;
            let machine = u8::from(state.manual)
                | u8::from(state.one_sided) << 1
                | u8::from(state.never_appears_locked) << 2;
            format!(
                " {}:{:08x}:{:08x}:{:08x}:{:08x}:{}:{}:{flags}:{machine}",
                handle.index,
                motion.power.to_bits(),
                motion.power_velocity.to_bits(),
                motion.position.to_bits(),
                motion.position_velocity.to_bits(),
                state.power_group,
                state.position_group,
            )
        })
        .collect()
}

/// The effects the devices started since they were last taken.
fn effects(devices: &mut Devices) -> Vec<String> {
    let handles: Vec<ObjectHandle> = devices.iter().map(|(handle, _)| handle).collect();
    let mut started = Vec::new();
    for handle in handles {
        if let Some(state) = devices.get_mut(handle) {
            for effect in state.effects.drain(..) {
                started.push(format!("effect {} {}", handle.index, effect.path));
            }
        }
    }
    started
}

#[test]
fn the_engine_keeps_device_groups_as_the_game_keeps_them() -> Result<(), Failure> {
    let mut devices = Devices::default();
    let mut definitions = Vec::new();
    let mut expected_effects = Vec::new();
    let (mut levels, mut calls, mut refused) = (0, 0, 0);
    for (number, line) in REFERENCE.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }
        let at = number + 1;
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["level", rest @ ..] => {
                let (level_definitions, scenario) = level(rest)?;
                definitions = level_definitions;
                devices = Devices::of(&scenario);
                levels += 1;
            }
            ["place", at, definition, power, position, flags, kind] => {
                let definition: usize = definition.parse()?;
                let placement = PlacementDevice {
                    power_group: power.parse()?,
                    position_group: position.parse()?,
                    flags: flags.parse()?,
                    kind_flags: kind.parse()?,
                };
                let device = definitions.get(definition).cloned();
                devices.place_device(object(at)?, &placement, device, 0.0);
            }
            ["effect", ..] => expected_effects.push(line.to_owned()),
            ["call", name, arguments @ ..] => {
                calls += 1;
                let answer = match (*name, arguments) {
                    ("group_set", [group, value, answer]) => {
                        let changed = devices.group_change(group.parse()?, real(value)?);
                        refused += usize::from(!changed);
                        Some((u8::from(changed).to_string(), answer.to_string()))
                    }
                    ("group_set_immediate", [group, value]) => {
                        devices.group_set_actual(group.parse()?, real(value)?);
                        None
                    }
                    ("group_get", [group, value]) => {
                        let got = devices.group_value(group.parse()?);
                        Some((format!("{:08x}", got.to_bits()), value.to_string()))
                    }
                    ("group_once", [group, once]) => {
                        devices.group_change_only_once_more(group.parse()?, *once == "1");
                        None
                    }
                    ("set_position", [at, value, answer]) => {
                        let changed = devices.set_desired_position(object(at)?, real(value)?);
                        refused += usize::from(!changed);
                        Some((u8::from(changed).to_string(), answer.to_string()))
                    }
                    ("set_position_immediate", [at, value]) => {
                        devices.set_actual_position(object(at)?, real(value)?);
                        None
                    }
                    ("set_power", [at, value]) => {
                        devices.set_power(object(at)?, real(value)?);
                        None
                    }
                    ("get", [at, position, power]) => {
                        let state = devices.get(object(at)?).ok_or("a device not placed")?;
                        let got = format!(
                            "{:08x} {:08x}",
                            state.position().to_bits(),
                            state.power().to_bits()
                        );
                        Some((got, format!("{position} {power}")))
                    }
                    ("one_sided", [at, value]) => {
                        let state = devices.get_mut(object(at)?).ok_or("a device")?;
                        if state.is_machine() {
                            state.one_sided = *value == "1";
                        }
                        None
                    }
                    ("operates_automatically", [at, value]) => {
                        let state = devices.get_mut(object(at)?).ok_or("a device")?;
                        if state.is_machine() {
                            state.manual = *value != "1";
                        }
                        None
                    }
                    ("never_appears_locked", [at, value]) => {
                        let state = devices.get_mut(object(at)?).ok_or("a device")?;
                        if state.is_machine() {
                            state.never_appears_locked = *value == "1";
                        }
                        None
                    }
                    ("delete", [at]) => {
                        devices.delete(object(at)?);
                        None
                    }
                    _ => return Err(format!("an unknown call: {line}").into()),
                };
                if let Some((got, expected)) = answer {
                    assert_eq!(got, expected, "line {at}: {line}");
                }
                assert_eq!(
                    effects(&mut devices),
                    std::mem::take(&mut expected_effects),
                    "line {at}: {line}"
                );
            }
            ["groups", ..] => assert_eq!(
                groups(&devices),
                &line["groups".len()..],
                "line {at}: {line}"
            ),
            ["devices", ..] => assert_eq!(
                states(&devices),
                &line["devices".len()..],
                "line {at}: {line}"
            ),
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!((levels, calls), (300, 11_031));
    assert!(refused > 1000, "{refused}");
    Ok(())
}
