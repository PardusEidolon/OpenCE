//! Units and vehicles: their vitality, their custom and recorded
//! animations, their weapons and riders (`hs_library_units.c`).

use std::collections::BTreeSet;
use std::sync::Arc;

use opence_content::{Model, ObjectKind, TagRef};
use opence_script::{Native, Value};
use opence_sim::animation::AnimationPlayer;
use opence_sim::damage::Vitality;
use opence_sim::objects::ObjectHandle;

use crate::{boolean, element, index, real, short, tag, text, Mission, Playing};

pub(crate) const NATIVES: &[(&str, Native<Mission>)] = &[
    ("unit_get_health", unit_get_health),
    ("unit_get_shield", unit_get_shield),
    ("unit_set_current_vitality", unit_set_current_vitality),
    ("units_set_current_vitality", units_set_current_vitality),
    ("unit_set_maximum_vitality", unit_set_maximum_vitality),
    ("unit_impervious", unit_impervious),
    ("unit_suspended", unit_suspended),
    ("unit_has_weapon_readied", unit_has_weapon_readied),
    ("unit_get_total_grenade_count", unit_get_total_grenade_count),
    (
        "unit_solo_player_integrated_night_vision_is_active",
        unit_solo_player_integrated_night_vision_is_active,
    ),
    ("custom_animation", custom_animation),
    ("custom_animation_list", custom_animation_list),
    (
        "unit_custom_animation_at_frame",
        unit_custom_animation_at_frame,
    ),
    (
        "unit_get_custom_animation_time",
        unit_get_custom_animation_time,
    ),
    ("unit_stop_custom_animation", unit_stop_custom_animation),
    ("scenery_animation_start", scenery_animation_start),
    (
        "scenery_animation_start_at_frame",
        scenery_animation_start_at_frame,
    ),
    ("recording_play", recording_play),
    ("recording_play_and_delete", recording_play_and_delete),
    ("recording_play_and_hover", recording_play_and_hover),
    ("recording_kill", recording_kill),
    ("recording_time", recording_time),
    ("vehicle_riders", vehicle_riders),
    ("vehicle_test_seat_list", vehicle_test_seat_list),
    ("vehicle_load_magic", vehicle_load_magic),
    ("vehicle_unload", vehicle_unload),
];

/// The frames a custom animation asked to interpolate blends in over, from
/// the pose before it (`object_start_interpolation(unit_index, 6)`).
const INTERPOLATION_FRAMES: u16 = 6;

impl Mission {
    /// Plays animation `name` of the graph `graph` on an object from
    /// `frame` (from its first if it has no such frame), until its last
    /// frame, blending in from the pose before it if `interpolate`; whether
    /// there is such an animation.
    pub fn play_animation(
        &mut self,
        handle: ObjectHandle,
        graph: &TagRef,
        name: &str,
        frame: u16,
        interpolate: bool,
    ) -> bool {
        let Some(place) = self
            .level
            .animation_graphs
            .iter()
            .position(|g| &g.tag == graph)
        else {
            return false;
        };
        let Some(graph) = self.graph(place) else {
            return false;
        };
        let Some((animation, frames)) = graph
            .animations
            .iter()
            .enumerate()
            .find(|(_, a)| a.name == name)
            .and_then(|(i, a)| Some((u16::try_from(i).ok()?, a.frame_count)))
        else {
            return false;
        };
        let model = self.model(handle);
        let mut player = AnimationPlayer::new(graph, model, animation);
        let first = if player.start_at(frame) { frame } else { 0 };
        if interpolate {
            if let Some(now) = self.sim.component::<AnimationPlayer>(handle) {
                player.blend_from(now.blended_pose(), INTERPOLATION_FRAMES);
            }
        }
        if !self.sim.animate(handle, player) {
            return false;
        }
        let ends = self.game_tick() + u64::from(frames.saturating_sub(first));
        self.animations.insert(
            handle,
            Playing {
                ends,
                delete: false,
            },
        );
        true
    }

    /// An object's model, shared.
    fn model(&self, handle: ObjectHandle) -> Option<Arc<Model>> {
        let tag = self.definition(handle)?.model.clone()?;
        let model = self.level.models.iter().find(|m| m.tag == tag)?;
        Some(Arc::new(model.clone()))
    }

    /// Sets an object's vitality to so much body and shield, as a script
    /// does. (unit_scripting_commands.c's
    /// `unit_scripting_set_current_vitality_of`)
    fn set_vitality(&mut self, handle: ObjectHandle, body: f32, shield: f32) {
        let maximums = self.maximum_vitality(handle);
        let Some(mut vitality) = self.sim.component::<Vitality>(handle) else {
            return;
        };
        vitality.set_current(body, shield, maximums);
        self.sim.insert(handle, vitality);
    }

    /// Gives a living object new maximums and fills it to them, as a
    /// script does. (unit_scripting_commands.c's
    /// `unit_scripting_set_maximum_vitality`)
    fn set_maximums(&mut self, handle: ObjectHandle, maximums: [f32; 2]) {
        let Some(mut vitality) = self.sim.component::<Vitality>(handle) else {
            return;
        };
        if vitality.dead {
            return;
        }
        vitality.set_maximums(maximums);
        self.maximums.insert(handle, maximums);
        self.sim.insert(handle, vitality);
    }

    /// Whether an object is a unit: a biped or a vehicle.
    fn is_unit(&self, handle: ObjectHandle) -> bool {
        self.sim
            .object(handle)
            .is_some_and(|o| matches!(o.kind, ObjectKind::Biped | ObjectKind::Vehicle))
    }

    /// Plays recording `recording` on a unit; whether there is one.
    /// (the original's `recorded_animation_play_internal`, in part)
    fn play_recording(&mut self, handle: ObjectHandle, recording: i32, delete: bool) -> bool {
        let Some(length) =
            element(&self.level.scenario.campaign.recordings, recording).map(|r| r.length_in_ticks)
        else {
            return false;
        };
        let ends = self.game_tick() + u64::try_from(length).unwrap_or(0);
        self.recordings.insert(handle, Playing { ends, delete });
        true
    }

    /// The players seated in a vehicle.
    fn riders(&self, vehicle: ObjectHandle) -> Vec<ObjectHandle> {
        self.sim
            .players()
            .iter()
            .filter(|p| p.seated().is_some_and(|s| s.vehicle == vehicle))
            .map(|p| p.handle())
            .collect()
    }
}

fn unit(mission: &Mission, arguments: &[Value]) -> Option<ObjectHandle> {
    mission.handle(index(arguments, 0))
}

/// Holds a unit still (true) or lets it go (`unit_suspended`).
fn unit_suspended(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = unit(mission, arguments) {
        mission.sim.suspend(handle, boolean(arguments, 1));
    }
    Value::Void
}

fn unit_get_health(mission: &mut Mission, arguments: &[Value]) -> Value {
    let health = unit(mission, arguments)
        .and_then(|h| mission.sim.component::<Vitality>(h))
        .map_or(-1.0, |v| v.script_health());
    Value::Real(health)
}

fn unit_get_shield(mission: &mut Mission, arguments: &[Value]) -> Value {
    let shield = unit(mission, arguments)
        .and_then(|h| mission.sim.component::<Vitality>(h))
        .map_or(-1.0, |v| v.script_shield());
    Value::Real(shield)
}

fn unit_set_current_vitality(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = unit(mission, arguments) {
        mission.set_vitality(handle, real(arguments, 1), real(arguments, 2));
    }
    Value::Void
}

fn units_set_current_vitality(mission: &mut Mission, arguments: &[Value]) -> Value {
    for handle in mission.list(index(arguments, 0)) {
        mission.set_vitality(handle, real(arguments, 1), real(arguments, 2));
    }
    Value::Void
}

fn unit_set_maximum_vitality(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = unit(mission, arguments) {
        mission.set_maximums(handle, [real(arguments, 1), real(arguments, 2)]);
    }
    Value::Void
}

fn unit_impervious(mission: &mut Mission, arguments: &[Value]) -> Value {
    let units: Vec<ObjectHandle> = mission
        .list(index(arguments, 0))
        .into_iter()
        .filter(|&handle| mission.is_unit(handle))
        .collect();
    set_impervious(&mut mission.impervious, units, boolean(arguments, 1));
    Value::Void
}

/// Makes each of `units` impervious or not. An impervious unit is still
/// hurt by damage, which only stops pushing it and making it flinch: it
/// is not made invulnerable. (unit_scripting_commands.c's
/// `unit_scripting_impervious`)
pub fn set_impervious<T: Ord>(
    impervious: &mut BTreeSet<T>,
    units: impl IntoIterator<Item = T>,
    on: bool,
) {
    for unit in units {
        if on {
            impervious.insert(unit);
        } else {
            impervious.remove(&unit);
        }
    }
}

fn unit_has_weapon_readied(mission: &mut Mission, arguments: &[Value]) -> Value {
    let current = unit(mission, arguments)
        .and_then(|h| mission.sim.players().iter().find(|p| p.handle() == h))
        .and_then(|p| p.weapon())
        .map(|(weapon, _)| weapon);
    Value::Boolean(weapon_readied(current, tag(arguments, 1)))
}

/// Whether the weapon in a unit's hands, `current`, is of the definition
/// `asked`: never when either is none. (unit_scripting_commands.c's
/// `unit_scripting_has_weapon_readied`)
pub fn weapon_readied<T: PartialEq>(current: Option<T>, asked: Option<T>) -> bool {
    asked.is_some() && current == asked
}

fn unit_get_total_grenade_count(mission: &mut Mission, arguments: &[Value]) -> Value {
    let count = unit(mission, arguments)
        .and_then(|h| mission.sim.players().iter().find(|p| p.handle() == h))
        .map_or(0, |p| grenade_total(&p.inventory().grenades));
    Value::Short(count)
}

/// A unit's grenades of every kind together, summed into a short as the
/// game sums them. (unit_scripting_commands.c's
/// `unit_scripting_get_grenade_count`)
pub fn grenade_total(counts: &[i16]) -> i16 {
    counts
        .iter()
        .fold(0, |total: i16, &count| total.wrapping_add(count))
}

fn unit_solo_player_integrated_night_vision_is_active(_: &mut Mission, _: &[Value]) -> Value {
    Value::Boolean(false)
}

fn custom_animation(mission: &mut Mission, arguments: &[Value]) -> Value {
    let started = match (unit(mission, arguments), tag(arguments, 1).cloned()) {
        (Some(handle), Some(graph)) => {
            mission.play_animation(handle, &graph, text(arguments, 2), 0, boolean(arguments, 3))
        }
        _ => false,
    };
    Value::Boolean(started)
}

fn custom_animation_list(mission: &mut Mission, arguments: &[Value]) -> Value {
    let Some(graph) = tag(arguments, 1).cloned() else {
        return Value::Boolean(false);
    };
    let handles = mission.list(index(arguments, 0));
    let started = start_in_turn(handles, |handle| {
        mission.is_unit(handle).then(|| {
            mission.play_animation(handle, &graph, text(arguments, 2), 0, boolean(arguments, 3))
        })
    });
    Value::Boolean(started)
}

/// Starts each of `items` in turn, while every one before it has started,
/// and whether all did: `start` gives whether one started, or `None` for
/// one that is not a unit, which is passed over. Once one fails no more
/// start, as the game's `result = result && unit_start_user_animation(...)`
/// stops calling. (unit_scripting_commands.c's
/// `unit_scripting_start_user_animation_list`)
pub fn start_in_turn<T>(
    items: impl IntoIterator<Item = T>,
    mut start: impl FnMut(T) -> Option<bool>,
) -> bool {
    let mut started = true;
    for item in items {
        if started {
            started = start(item).unwrap_or(true);
        }
    }
    started
}

fn unit_custom_animation_at_frame(mission: &mut Mission, arguments: &[Value]) -> Value {
    let frame = u16::try_from(short(arguments, 4)).unwrap_or(0);
    let started = match (unit(mission, arguments), tag(arguments, 1).cloned()) {
        (Some(handle), Some(graph)) => {
            let interpolate = boolean(arguments, 3);
            mission.play_animation(handle, &graph, text(arguments, 2), frame, interpolate)
        }
        _ => false,
    };
    Value::Boolean(started)
}

/// The ticks left of a unit's custom animation or recording.
fn remaining(mission: &Mission, ends: Option<u64>) -> Value {
    let tick = mission.game_tick();
    let left = ends.map_or(0, |ends| ends.saturating_sub(tick));
    Value::Short(i16::try_from(left).unwrap_or(i16::MAX))
}

fn unit_get_custom_animation_time(mission: &mut Mission, arguments: &[Value]) -> Value {
    let tick = mission.game_tick();
    let left = unit(mission, arguments)
        .and_then(|h| mission.animations.get(&h))
        .map(|p| p.ends.saturating_sub(tick));
    Value::Short(custom_animation_time(left))
}

/// What a script is told is left of a unit's custom animation, from the
/// frames it has left (`None` when none plays): two fewer, and never
/// below nothing, as the game counts `frame_count - frame_index - 2`.
/// (units.c's `unit_get_custom_animation_time`)
pub fn custom_animation_time(frames_left: Option<u64>) -> i16 {
    frames_left.map_or(0, |left| {
        i16::try_from(left.saturating_sub(2)).unwrap_or(i16::MAX)
    })
}

fn unit_stop_custom_animation(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = unit(mission, arguments) {
        mission.animations.remove(&handle);
    }
    Value::Void
}

fn scenery_animation_start(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let (Some(handle), Some(graph)) = (unit(mission, arguments), tag(arguments, 1).cloned()) {
        mission.play_animation(handle, &graph, text(arguments, 2), 0, false);
    }
    Value::Void
}

fn scenery_animation_start_at_frame(mission: &mut Mission, arguments: &[Value]) -> Value {
    let frame = u16::try_from(short(arguments, 3)).unwrap_or(0);
    if let (Some(handle), Some(graph)) = (unit(mission, arguments), tag(arguments, 1).cloned()) {
        mission.play_animation(handle, &graph, text(arguments, 2), frame, false);
    }
    Value::Void
}

fn recording_play(mission: &mut Mission, arguments: &[Value]) -> Value {
    let played = unit(mission, arguments)
        .is_some_and(|h| mission.play_recording(h, index(arguments, 1), false));
    Value::Boolean(played)
}

fn recording_play_and_delete(mission: &mut Mission, arguments: &[Value]) -> Value {
    let played = unit(mission, arguments)
        .is_some_and(|h| mission.play_recording(h, index(arguments, 1), true));
    Value::Boolean(played)
}

fn recording_play_and_hover(mission: &mut Mission, arguments: &[Value]) -> Value {
    recording_play(mission, arguments)
}

fn recording_kill(mission: &mut Mission, arguments: &[Value]) -> Value {
    if let Some(handle) = unit(mission, arguments) {
        mission.recordings.remove(&handle);
    }
    Value::Void
}

fn recording_time(mission: &mut Mission, arguments: &[Value]) -> Value {
    let ends = unit(mission, arguments)
        .and_then(|h| mission.recordings.get(&h))
        .map(|p| p.ends);
    remaining(mission, ends)
}

fn vehicle_riders(mission: &mut Mission, arguments: &[Value]) -> Value {
    let riders = unit(mission, arguments)
        .map(|h| mission.riders(h))
        .unwrap_or_default();
    Value::Index(mission.list_of(riders))
}

fn vehicle_test_seat_list(mission: &mut Mission, arguments: &[Value]) -> Value {
    let Some(vehicle) = unit(mission, arguments) else {
        return Value::Boolean(false);
    };
    let mut riders: Vec<(ObjectHandle, usize)> = mission
        .sim
        .players()
        .iter()
        .filter_map(|p| {
            p.seated()
                .filter(|s| s.vehicle == vehicle)
                .map(|s| (p.handle(), s.seat))
        })
        .collect();
    riders.sort_by_key(|(handle, _)| handle.index);
    let labels = mission.sim.rides().seat_labels(vehicle);
    let objects = mission.list(index(arguments, 2));
    Value::Boolean(seat_rider_listed(
        &labels,
        &riders,
        text(arguments, 1),
        &objects,
    ))
}

/// Whether the rider of a seat labelled `asked` (in any case) is among
/// `objects`. `riders` are the vehicle's, each with its seat, in the
/// order of their objects; of a seat's riders only the first counts, as
/// the game stops at the first unit it finds in the seat. (units.c's
/// `unit_scripting_vehicle_test_seat_list`)
pub fn seat_rider_listed<T: PartialEq + Copy>(
    labels: &[&str],
    riders: &[(T, usize)],
    asked: &str,
    objects: &[T],
) -> bool {
    labels
        .iter()
        .enumerate()
        .filter(|(_, label)| label.eq_ignore_ascii_case(asked))
        .filter_map(|(seat, _)| riders.iter().find(|(_, at)| *at == seat))
        .any(|(rider, _)| objects.contains(rider))
}

fn vehicle_load_magic(mission: &mut Mission, arguments: &[Value]) -> Value {
    mission.present("vehicle_load_magic", arguments);
    Value::Short(0)
}

fn vehicle_unload(mission: &mut Mission, arguments: &[Value]) -> Value {
    mission.present("vehicle_unload", arguments);
    Value::Short(0)
}
