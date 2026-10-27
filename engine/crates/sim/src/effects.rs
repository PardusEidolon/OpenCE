//! Effects (`effects.c`): what shots, impacts, detonations and objects
//! make. An effect runs its events in turn; each event, unless it is
//! skipped, waits its delay, makes its parts where its locations are, and
//! lasts its duration before the next begins. An effect on an object loops
//! from its loop's first event while the object lasts; any other ends after
//! its last event. The parts made are handed out each tick to what draws
//! them (lights, decals); effects change nothing the game's state depends
//! on, so they draw on a seed of their own.

use std::collections::BTreeSet;

use glam::{Quat, Vec3};
use opence_content::{
    Effect, EffectPart, Level, MaterialEffects, MaterialResponse, Projectile, TagRef,
};

use crate::objects::ObjectHandle;
use crate::random::RandomSeed;
use crate::Simulation;

/// The most effects that run at once; the oldest end first.
pub const MAX_EFFECTS: usize = 256;

/// The game's ticks a second.
const TICKS_PER_SECOND: f32 = 30.0;

/// The seed effects draw on, apart from the game's.
const EFFECT_SEED: u32 = 0x00EF_FEC7;

/// The surface an effect was made on: where a shot struck.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Surface {
    /// The point.
    pub point: Vec3,
    /// The surface's normal there.
    pub normal: Vec3,
    /// The way what struck it was going.
    pub direction: Vec3,
    /// The object struck, if one was.
    pub object: Option<ObjectHandle>,
}

/// Where an effect is made.
#[derive(Clone, Debug, PartialEq)]
pub struct Origin {
    /// Its position.
    pub position: Vec3,
    /// Its turn: its x axis the way it faces.
    pub rotation: Quat,
    /// How fast what made it moves, in world units a second.
    pub velocity: Vec3,
    /// The object it is on and the marker it is at; its locations are that
    /// object's markers.
    pub on: Option<(ObjectHandle, String)>,
    /// The surface it was made on, if any.
    pub surface: Option<Surface>,
    /// Its scales A and B, each 0 to 1, which scale what its parts say
    /// they scale by.
    pub scale: [f32; 2],
    /// Its colour, red, green and blue, which tints parts tinted by it.
    pub color: [f32; 3],
    /// Its own locations by name, a point and a turn, which come before
    /// its object's markers (a thruster's blast's incident, normal,
    /// reflected and midpoint, as `effect_new_unattached_from_markers`
    /// gives them).
    pub locations: Vec<(String, Vec3, Quat)>,
}

impl Origin {
    /// An effect at `position`, facing along `rotation`'s x axis.
    pub fn at(position: Vec3, rotation: Quat) -> Self {
        Self {
            position,
            rotation,
            velocity: Vec3::ZERO,
            on: None,
            surface: None,
            scale: [1.0; 2],
            color: [1.0; 3],
            locations: Vec::new(),
        }
    }
}

/// An effect running.
#[derive(Clone, Debug, PartialEq)]
pub struct Running {
    /// Its number, which it is known by from tick to tick.
    pub number: u32,
    /// Its tag.
    pub tag: TagRef,
    /// Where it was made.
    pub origin: Origin,
    /// The event it is in.
    pub event: usize,
    /// The ticks since the event began.
    pub clock: u32,
    /// The ticks the event waits before its parts are made.
    pub delay: u32,
    /// The ticks the event lasts after its delay.
    pub duration: u32,
    /// The event is skipped: it makes nothing.
    pub skipped: bool,
    /// The event's parts have been made.
    pub fired: bool,
    /// The ticks it has run.
    pub age: u32,
}

/// A part an event made.
#[derive(Clone, Debug, PartialEq)]
pub struct FiredPart {
    /// The number of the effect that made it.
    pub effect: u32,
    /// The event's index.
    pub event: usize,
    /// The part's index in the event.
    pub part: usize,
    /// What it is: a light, a decal, a particle system, a sound...
    pub tag: TagRef,
    /// Where it is made.
    pub position: Vec3,
    /// Its turn: its x axis the way it faces.
    pub rotation: Quat,
    /// How fast it goes, in world units a second.
    pub velocity: Vec3,
    /// The object its effect is on.
    pub on: Option<ObjectHandle>,
    /// The surface its effect was made on.
    pub surface: Option<Surface>,
    /// How much a decal it lays is grown, drawn between the part's bounds
    /// after its velocity, as the game draws it (0 for parts not decals).
    pub radius_modifier: f32,
    /// The scale it is made at, 0 to 1: one, by the effect's scales that
    /// scale it (a sound's volume between its zero and one modifiers).
    pub scale: f32,
}

/// The effects running.
#[derive(Clone, Debug)]
pub struct Effects {
    running: Vec<Running>,
    fired: Vec<FiredPart>,
    emitting: Vec<Running>,
    seed: RandomSeed,
    made: u32,
    /// The objects' attachments that have been started, by object and
    /// attachment.
    attached: BTreeSet<(ObjectHandle, usize)>,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            running: Vec::new(),
            fired: Vec::new(),
            emitting: Vec::new(),
            seed: RandomSeed(EFFECT_SEED),
            made: 0,
            attached: BTreeSet::new(),
        }
    }
}

/// The ticks in `seconds`.
fn ticks(seconds: f32) -> u32 {
    libm::roundf(seconds.max(0.0) * TICKS_PER_SECOND) as u32
}

/// A number between `bounds`, drawn from `seed`.
fn between(seed: &mut RandomSeed, [lower, upper]: [f32; 2]) -> f32 {
    lower + (upper - lower) * seed.next_real()
}

/// The effects a projectile makes where it strikes a surface of
/// `material` (-1: one not known), on an object or not (`on_object`): its
/// response's effect (`projectile_collision`), and if the response
/// detonates it, the response's detonation effect (`projectile_detonate`).
/// A material past its responses takes the default response, which makes
/// none.
pub fn response_effects(projectile: &Projectile, material: i16, on_object: bool) -> Vec<&TagRef> {
    let Some(response) = usize::try_from(material)
        .ok()
        .and_then(|material| projectile.material_responses.get(material))
    else {
        return Vec::new();
    };
    // (overpenetrating the level, a projectile with no timer detonates)
    let detonates = response.default_response == MaterialResponse::DETONATE
        || (response.default_response == MaterialResponse::OVERPENETRATE
            && !on_object
            && projectile.timer[1] == 0.0);
    response
        .default_effect
        .iter()
        .chain(response.detonation_effect.iter().filter(|_| detonates))
        .collect()
}

/// The turn whose x axis is `forward`.
pub fn facing(forward: Vec3) -> Quat {
    let forward = forward.normalize_or_zero();
    if forward == Vec3::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_rotation_arc(Vec3::X, forward)
}

impl Effects {
    /// The effects running, oldest first.
    pub fn running(&self) -> &[Running] {
        &self.running
    }

    /// The parts made in the last tick.
    pub fn fired(&self) -> &[FiredPart] {
        &self.fired
    }

    /// The effects as each was in each event the last tick ran through
    /// after its delay: an event that lasts no time is followed by the
    /// next in the same tick, and the effect may end in it, but its
    /// particles are still made (as the original makes them for each
    /// event it updates).
    pub fn emitting(&self) -> &[Running] {
        &self.emitting
    }

    /// Starts `tag` at `origin`; its number, or none if the level lacks it
    /// or it has no events.
    pub fn start(&mut self, level: &Level, tag: &TagRef, origin: Origin) -> Option<u32> {
        let effect = level
            .effect(tag)
            .filter(|effect| !effect.events.is_empty())?;
        self.made = self.made.wrapping_add(1);
        let mut running = Running {
            number: self.made,
            tag: tag.clone(),
            origin,
            event: 0,
            clock: 0,
            delay: 0,
            duration: 0,
            skipped: false,
            fired: false,
            age: 0,
        };
        self.enter(effect, &mut running, 0);
        self.running.push(running);
        let excess = self.running.len().saturating_sub(MAX_EFFECTS);
        self.running.drain(..excess);
        Some(self.made)
    }

    /// Starts attachment `slot` of the object `handle`, `tag` at `marker`,
    /// once in the object's life.
    pub fn attach(
        &mut self,
        level: &Level,
        handle: ObjectHandle,
        slot: usize,
        tag: &TagRef,
        marker: &str,
    ) {
        if !self.attached.insert((handle, slot)) {
            return;
        }
        let origin = Origin {
            on: Some((handle, marker.to_owned())),
            ..Origin::at(Vec3::ZERO, Quat::IDENTITY)
        };
        self.start(level, tag, origin);
    }

    /// Starts the creation effect of the object `handle`, just made as
    /// `object` (`object_new`): on it, at its origin.
    pub fn created(&mut self, level: &Level, handle: ObjectHandle, object: &TagRef) {
        let Some(effect) = level
            .object(object)
            .and_then(|definition| definition.creation_effect.as_ref())
        else {
            return;
        };
        let origin = Origin {
            on: Some((handle, String::new())),
            ..Origin::at(Vec3::ZERO, Quat::IDENTITY)
        };
        self.start(level, effect, origin);
    }

    /// Begins `effect`'s event `event` for `running`: whether it is
    /// skipped, its delay and its duration.
    fn enter(&mut self, effect: &Effect, running: &mut Running, event: usize) {
        running.event = event;
        running.clock = 0;
        running.fired = false;
        let Some(event) = effect.events.get(event) else {
            return;
        };
        running.skipped = event.skip_fraction > 0.0 && self.seed.next_real() < event.skip_fraction;
        running.delay = ticks(between(&mut self.seed, event.delay));
        running.duration = ticks(between(&mut self.seed, event.duration));
    }

    /// Starts the effects this tick of `simulation` made: its weapons'
    /// shots, its projectiles' impacts (each by the material it struck) and
    /// its detonations.
    pub fn trigger(&mut self, level: &Level, simulation: &Simulation) {
        for shot in simulation.effects() {
            let origin = Origin {
                velocity: shot.velocity,
                locations: shot.locations.clone(),
                ..Origin::at(shot.position, shot.rotation)
            };
            self.start(level, &shot.effect, origin);
        }
        for impact in simulation.impacts() {
            let Some(projectile) = level
                .object(&impact.projectile)
                .and_then(|object| object.projectile.as_ref())
            else {
                continue;
            };
            for effect in response_effects(projectile, impact.material, impact.object.is_some()) {
                let origin = Origin {
                    surface: Some(Surface {
                        point: impact.point,
                        normal: impact.normal,
                        direction: impact.direction,
                        object: impact.object,
                    }),
                    ..Origin::at(impact.point, facing(impact.normal))
                };
                self.start(level, effect, origin);
            }
        }
        for detonation in simulation.detonations() {
            let Some(effect) = detonation.effect.as_ref() else {
                continue;
            };
            // (projectile_detonate: made in the air, at the projectile and
            // at its "gravity", the same point facing down, where the
            // decals it lays look for the ground)
            let origin = Origin {
                locations: vec![("gravity".into(), detonation.point, facing(Vec3::NEG_Z))],
                ..Origin::at(detonation.point, facing(Vec3::Z))
            };
            self.start(level, effect, origin);
        }
        // (material_effect_new: a slip's effect, a centimetre off the
        // ground, scaled by how hard it slips)
        for slipped in simulation.slips() {
            let slip = &slipped.slip;
            let event = if slip.metallic {
                MaterialEffects::METALLIC_SLIP
            } else {
                MaterialEffects::VEHICLE_SLIP
            };
            let Some(effect) = level
                .material_effects(&slipped.material_effects)
                .and_then(|effects| effects.of(event, slip.material))
                .and_then(|made| made.effect.as_ref())
            else {
                continue;
            };
            let origin = Origin {
                scale: [slip.scale, 0.0],
                ..Origin::at(slip.position + slip.normal * 0.01, facing(slip.normal))
            };
            self.start(level, effect, origin);
        }
        // (create_ghost_effect and create_pelican_effect: a thruster's
        // blast's effect where it meets the ground, at its locations)
        for thrust in simulation.thrusts() {
            let Some(&(_, position, forward)) = thrust.locations.first() else {
                continue;
            };
            let origin = Origin {
                scale: [thrust.scale; 2],
                locations: thrust
                    .locations
                    .iter()
                    .map(|&(name, point, way)| (name.to_owned(), point, facing(way)))
                    .collect(),
                ..Origin::at(position, facing(forward))
            };
            self.start(level, &thrust.effect, origin);
        }
    }

    /// One tick: effects on objects that are gone (by `exists`) end, and
    /// each other's events go on, making their parts where `locate` says
    /// their objects' markers are (a position and a turn; none for an
    /// object without the marker).
    pub fn tick(
        &mut self,
        level: &Level,
        exists: impl Fn(ObjectHandle) -> bool,
        mut locate: impl FnMut(ObjectHandle, &str) -> Option<(Vec3, Quat)>,
    ) {
        self.fired.clear();
        self.emitting.clear();
        self.attached.retain(|(handle, _)| exists(*handle));
        let mut running = std::mem::take(&mut self.running);
        running.retain_mut(|running| {
            if running
                .origin
                .on
                .as_ref()
                .is_some_and(|(handle, _)| !exists(*handle))
            {
                return false;
            }
            let Some(effect) = level.effect(&running.tag) else {
                return false;
            };
            // (an effect on an object is where its marker is)
            if let Some(at) = running
                .origin
                .on
                .as_ref()
                .and_then(|(handle, marker)| locate(*handle, marker))
            {
                (running.origin.position, running.origin.rotation) = at;
            }
            let going = self.advance(effect, running, &mut locate);
            running.clock = running.clock.saturating_add(1);
            running.age = running.age.saturating_add(1);
            going
        });
        self.running = running;
    }

    /// Runs `running`'s events as far as this tick takes them; false once
    /// it has ended.
    fn advance(
        &mut self,
        effect: &Effect,
        running: &mut Running,
        locate: &mut impl FnMut(ObjectHandle, &str) -> Option<(Vec3, Quat)>,
    ) -> bool {
        // (an event without delay or duration is followed by the next in
        // the same tick, each at most once)
        for _ in 0..=effect.events.len() {
            if !running.fired {
                if running.clock < running.delay {
                    return true;
                }
                running.fired = true;
                if !running.skipped {
                    self.fire(effect, running, locate);
                }
            }
            if !running.skipped
                && (running.duration == 0 || running.clock < running.delay + running.duration)
            {
                self.emitting.push(running.clone());
            }
            if running.clock < running.delay + running.duration {
                return true;
            }
            let mut next = running.event + 1;
            if next >= effect.events.len() {
                let looping = usize::try_from(effect.loop_start_event)
                    .ok()
                    .filter(|start| *start < effect.events.len());
                match (looping, &running.origin.on) {
                    (Some(start), Some(_)) => next = start,
                    _ => return false,
                }
            }
            self.enter(effect, running, next);
        }
        true
    }

    /// Makes the parts of `running`'s event.
    fn fire(
        &mut self,
        effect: &Effect,
        running: &Running,
        locate: &mut impl FnMut(ObjectHandle, &str) -> Option<(Vec3, Quat)>,
    ) {
        let Some(event) = effect.events.get(running.event) else {
            return;
        };
        for (index, part) in event.parts.iter().enumerate() {
            // (effects are made in the air, and violently)
            if matches!(part.create_in, EffectPart::IN_WATER | EffectPart::IN_SPACE)
                || part.create_in_mode == EffectPart::NOT_VIOLENT
            {
                continue;
            }
            let location = usize::try_from(part.location)
                .ok()
                .and_then(|location| effect.locations.get(location))
                .map_or("", String::as_str);
            let (position, mut rotation) = place(&running.origin, location, locate);
            if part.flags & EffectPart::FACE_DOWN != 0 {
                rotation = facing(Vec3::NEG_Z);
            }
            let velocity = self.velocity(part, rotation);
            let radius_modifier = if part.tag.group.0 == *b"deca" {
                between(&mut self.seed, part.radius_modifier)
            } else {
                0.0
            };
            self.fired.push(FiredPart {
                effect: running.number,
                event: running.event,
                part: index,
                tag: part.tag.clone(),
                position,
                rotation,
                velocity,
                on: running.origin.on.as_ref().map(|(handle, _)| *handle),
                surface: running.origin.surface,
                radius_modifier,
                scale: part_scale(part, running.origin.scale),
            });
        }
    }

    /// A part's velocity: a speed between its bounds, inside its cone
    /// about the way `rotation` faces.
    fn velocity(&mut self, part: &EffectPart, rotation: Quat) -> Vec3 {
        let speed = between(&mut self.seed, part.velocity);
        let tilt = part.velocity_cone_angle * self.seed.next_real();
        let around = core::f32::consts::TAU * self.seed.next_real();
        let local = Vec3::new(
            libm::cosf(tilt),
            libm::sinf(tilt) * libm::cosf(around),
            libm::sinf(tilt) * libm::sinf(around),
        );
        rotation * local * speed
    }
}

/// Where `location` is for an effect made at `origin`: its own location
/// of that name, its object's marker of that name (else the effect's own
/// marker), or the origin itself.
fn place(
    origin: &Origin,
    location: &str,
    locate: &mut impl FnMut(ObjectHandle, &str) -> Option<(Vec3, Quat)>,
) -> (Vec3, Quat) {
    if let Some(&(_, position, rotation)) = origin
        .locations
        .iter()
        .find(|(name, _, _)| !location.is_empty() && name.eq_ignore_ascii_case(location))
    {
        return (position, rotation);
    }
    let Some((handle, marker)) = &origin.on else {
        return (origin.position, origin.rotation);
    };
    let named = (!location.is_empty())
        .then(|| locate(*handle, location))
        .flatten();
    named
        .or_else(|| locate(*handle, marker))
        .unwrap_or((origin.position, origin.rotation))
}

/// The scale `part` is made at by an effect of scales `by`: one, times
/// each scale whose mask scales it (`effect_scale`).
fn part_scale(part: &EffectPart, by: [f32; 2]) -> f32 {
    part.scales
        .iter()
        .zip(by)
        .filter(|(mask, _)| *mask & EffectPart::SCALES_SCALE != 0)
        .fold(1.0, |scale, (_, by)| scale * by)
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_content::{EffectEvent, Group, ObjectDefinition, ObjectKind};

    fn tag(group: &[u8; 4], path: &str) -> TagRef {
        TagRef {
            group: Group(*group),
            path: path.into(),
        }
    }

    fn part(path: &str, location: i16) -> EffectPart {
        EffectPart {
            create_in: 0,
            create_in_mode: 0,
            location,
            flags: 0,
            tag: tag(b"ligh", path),
            velocity: [0.0, 0.0],
            velocity_cone_angle: 0.0,
            angular_velocity: [0.0, 0.0],
            radius_modifier: [1.0, 1.0],
            scales: [0; 2],
        }
    }

    fn event(delay: f32, duration: f32, parts: Vec<EffectPart>) -> EffectEvent {
        EffectEvent {
            skip_fraction: 0.0,
            delay: [delay, delay],
            duration: [duration, duration],
            parts,
            particles: Vec::new(),
        }
    }

    /// A level whose effect `two` makes `first` at once, lasting a second,
    /// then `second` half a second later at the marker `muzzle`.
    fn level(loop_start_event: i16) -> Level {
        let mut level = opence_content::devmap::level(None);
        level.effects.push(Effect {
            loop_start_event,
            locations: vec![String::new(), "muzzle".into()],
            events: vec![
                event(0.0, 1.0, vec![part("first", 0)]),
                event(0.5, 0.0, vec![part("second", 1)]),
            ],
            ..Effect::new(tag(b"effe", "two"))
        });
        level
    }

    fn object(tag: TagRef, creation_effect: Option<TagRef>) -> ObjectDefinition {
        ObjectDefinition {
            widgets: Vec::new(),
            tag,
            kind: ObjectKind::Scenery,
            flags: 0,
            bounding_radius: 1.0,
            bounding_offset: [0.0, 0.0, 0.0],
            acceleration_scale: 1.0,
            render_bounding_radius: 1.0,
            icon_text_index: -1,
            model: None,
            animation_graph: None,
            collision_model: None,
            physics: None,
            modifier_shader: None,
            creation_effect,
            function_modes: Default::default(),
            functions: Vec::new(),
            change_colors: Vec::new(),
            unit_hud: None,
            material_effects: None,
            unit_melee_damage: None,
            biped: None,
            weapon: None,
            projectile: None,
            equipment: None,
            vehicle: None,
            device: None,
            attachments: Vec::new(),
        }
    }

    fn names(effects: &Effects) -> Vec<&str> {
        effects
            .fired()
            .iter()
            .map(|part| part.tag.path.as_str())
            .collect()
    }

    #[test]
    fn a_part_is_made_at_its_origins_own_location_of_its_name() {
        let mut level = level(-1);
        level.effects.push(Effect {
            locations: vec!["NORMAL".into(), "nowhere".into()],
            events: vec![event(0.0, 0.0, vec![part("up", 0), part("there", 1)])],
            ..Effect::new(tag(b"effe", "blast"))
        });
        let mut effects = Effects::default();
        let at = Vec3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(1.0, 2.0, 0.0);
        let origin = Origin {
            locations: vec![("normal".into(), normal, facing(Vec3::Z))],
            ..Origin::at(at, Quat::IDENTITY)
        };
        effects.start(&level, &tag(b"effe", "blast"), origin);
        effects.tick(&level, |_| true, |_, _| None);
        let made: Vec<(&str, Vec3)> = effects
            .fired()
            .iter()
            .map(|part| (part.tag.path.as_str(), part.position))
            .collect();
        assert_eq!(made, [("up", normal), ("there", at)]);
    }

    #[test]
    fn a_part_is_made_at_one_scaled_by_the_scales_that_scale_it() {
        // (`effect_scale(effect, 1.0f, part->scale_a_flags,
        // part->scale_b_flags, _effect_part_type_scale_bit)`)
        let mut level = level(-1);
        let mut by_a = part("by a", 0);
        by_a.scales = [EffectPart::SCALES_SCALE, 0];
        let mut by_both = part("by both", 0);
        by_both.scales = [EffectPart::SCALES_SCALE | 1, EffectPart::SCALES_SCALE];
        let mut by_another = part("by another value", 0);
        by_another.scales = [1, 1 << 6];
        level.effects.push(Effect {
            locations: vec![String::new()],
            events: vec![event(
                0.0,
                0.0,
                vec![part("by none", 0), by_a, by_both, by_another],
            )],
            ..Effect::new(tag(b"effe", "impact"))
        });
        let mut effects = Effects::default();
        let origin = Origin {
            scale: [0.5, 0.25],
            ..Origin::at(Vec3::ZERO, Quat::IDENTITY)
        };
        effects.start(&level, &tag(b"effe", "impact"), origin);
        effects.tick(&level, |_| true, |_, _| None);
        let made: Vec<(&str, f32)> = effects
            .fired()
            .iter()
            .map(|part| (part.tag.path.as_str(), part.scale))
            .collect();
        assert_eq!(
            made,
            [
                ("by none", 1.0),
                ("by a", 0.5),
                ("by both", 0.125),
                ("by another value", 1.0)
            ]
        );
    }

    #[test]
    fn a_decal_part_draws_its_radius_modifier_between_its_bounds() {
        let mut level = level(-1);
        let mut decal = part("char", 0);
        decal.tag.group = opence_content::Group(*b"deca");
        decal.radius_modifier = [1.0, 1.5];
        level.effects.push(Effect {
            locations: vec![String::new()],
            events: vec![event(0.0, 0.0, vec![decal, part("flash", 0)])],
            ..Effect::new(tag(b"effe", "blast"))
        });
        let mut effects = Effects::default();
        let mut drawn = Vec::new();
        for _ in 0..20 {
            effects.start(
                &level,
                &tag(b"effe", "blast"),
                Origin::at(Vec3::ZERO, Quat::IDENTITY),
            );
            effects.tick(&level, |_| true, |_, _| None);
            drawn.extend(
                effects
                    .fired()
                    .iter()
                    .map(|part| (part.tag.group.0, part.radius_modifier)),
            );
        }
        assert!(drawn
            .iter()
            .filter(|(group, _)| group == b"deca")
            .all(|(_, modifier)| (1.0..=1.5).contains(modifier)));
        assert!(drawn
            .iter()
            .filter(|(group, _)| group == b"deca")
            .any(|(_, modifier)| *modifier != 1.0));
        // (a part not a decal has none)
        assert!(drawn
            .iter()
            .filter(|(group, _)| group == b"ligh")
            .all(|(_, modifier)| *modifier == 0.0));
        assert_eq!(drawn.len(), 40);
    }

    #[test]
    fn an_effects_events_make_their_parts_at_their_times() {
        let level = level(-1);
        let mut effects = Effects::default();
        let at = Vec3::new(1.0, 2.0, 3.0);
        let number = effects.start(&level, &tag(b"effe", "two"), Origin::at(at, Quat::IDENTITY));
        assert_eq!(number, Some(1));
        let mut made = Vec::new();
        for tick in 0..60 {
            effects.tick(&level, |_| true, |_, _| None);
            for name in names(&effects) {
                made.push((tick, name.to_owned()));
            }
        }
        // (the first at once; the second after the first's second and its
        // own half second)
        assert_eq!(made, [(0, "first".into()), (45, "second".into())]);
        assert!(effects.running().is_empty(), "the effect did not end");
        assert!(effects
            .start(
                &level,
                &tag(b"effe", "none"),
                Origin::at(at, Quat::IDENTITY)
            )
            .is_none());
    }

    #[test]
    fn each_event_a_tick_runs_through_is_kept_for_its_particles() {
        let level = level(-1);
        let mut effects = Effects::default();
        let at = Vec3::new(1.0, 2.0, 3.0);
        effects.start(&level, &tag(b"effe", "two"), Origin::at(at, Quat::IDENTITY));
        let mut seen = Vec::new();
        for tick in 0..60 {
            effects.tick(&level, |_| true, |_, _| None);
            for running in effects.emitting() {
                seen.push((tick, running.event, running.clock, running.duration));
            }
        }
        // (the first event for each of its thirty ticks; the second, which
        // lasts no time and ends the effect, once, in the tick it ends)
        assert_eq!(seen.len(), 31, "{seen:?}");
        assert_eq!(seen[0], (0, 0, 0, 30));
        assert_eq!(seen[29], (29, 0, 29, 30));
        assert_eq!(seen[30], (45, 1, 15, 0));
        assert!(effects.running().is_empty());
        assert!(effects.emitting().is_empty());
    }

    #[test]
    fn an_object_starts_its_creation_effect_on_itself_when_it_is_made() {
        let mut level = level(-1);
        let made = tag(b"scen", "made with smoke");
        let plain = tag(b"scen", "plain");
        level
            .objects
            .push(object(made.clone(), Some(tag(b"effe", "two"))));
        level.objects.push(object(plain.clone(), None));
        let mut effects = Effects::default();
        let handle = ObjectHandle { index: 3, salt: 1 };
        effects.created(&level, handle, &made);
        effects.created(&level, ObjectHandle { index: 4, salt: 1 }, &plain);
        let running: Vec<_> = effects
            .running()
            .iter()
            .map(|r| r.origin.on.clone())
            .collect();
        // (at the object's origin: no marker)
        assert_eq!(running, [Some((handle, String::new()))]);
    }

    #[test]
    fn parts_are_made_at_their_locations_markers_and_effects_on_objects_loop() {
        let level = level(1);
        let mut effects = Effects::default();
        let handle = ObjectHandle { index: 3, salt: 1 };
        effects.attach(&level, handle, 0, &tag(b"effe", "two"), "root");
        // (started once in the object's life)
        effects.attach(&level, handle, 0, &tag(b"effe", "two"), "root");
        assert_eq!(effects.running().len(), 1);
        let locate = |_: ObjectHandle, marker: &str| match marker {
            "root" => Some((Vec3::new(5.0, 0.0, 0.0), Quat::IDENTITY)),
            "muzzle" => Some((Vec3::new(6.0, 0.0, 1.0), Quat::IDENTITY)),
            _ => None,
        };
        let mut made = Vec::new();
        for _ in 0..120 {
            effects.tick(&level, |_| true, locate);
            made.extend(
                effects
                    .fired()
                    .iter()
                    .map(|part| (part.tag.path.clone(), part.position, part.on)),
            );
        }
        assert_eq!(
            made[0],
            ("first".into(), Vec3::new(5.0, 0.0, 0.0), Some(handle))
        );
        assert_eq!(
            made[1],
            ("second".into(), Vec3::new(6.0, 0.0, 1.0), Some(handle))
        );
        // (it loops from its second event, every half second)
        assert!(
            made[2..].iter().all(|(name, ..)| name == "second"),
            "{made:?}"
        );
        assert!(made.len() > 3, "{made:?}");
        // (it ends with its object)
        effects.tick(&level, |_| false, locate);
        assert!(effects.running().is_empty());
        assert!(effects.fired().is_empty());
    }

    #[test]
    fn skipped_events_and_parts_made_elsewhere_make_nothing() {
        let mut level = level(-1);
        let effect = &mut level.effects[0];
        effect.events[0].skip_fraction = 1.0;
        effect.events[1].parts[0].create_in = EffectPart::IN_WATER;
        let mut face = part("down", 0);
        face.flags = EffectPart::FACE_DOWN;
        face.velocity = [2.0, 2.0];
        effect.events[1].parts.push(face);
        let mut gentle = part("gentle", 0);
        gentle.create_in_mode = EffectPart::NOT_VIOLENT;
        effect.events[1].parts.push(gentle);
        let mut effects = Effects::default();
        effects.start(
            &level,
            &tag(b"effe", "two"),
            Origin::at(Vec3::ZERO, Quat::IDENTITY),
        );
        let mut made = Vec::new();
        for _ in 0..60 {
            effects.tick(&level, |_| true, |_, _| None);
            made.extend(effects.fired().iter().cloned());
        }
        assert_eq!(made.len(), 1, "{made:?}");
        assert_eq!(made[0].tag.path, "down");
        // (facing down, and going that way at its speed)
        assert!((made[0].rotation * Vec3::X - Vec3::NEG_Z).length() < 1e-5);
        assert!((made[0].velocity - Vec3::new(0.0, 0.0, -2.0)).length() < 1e-4);
    }

    #[test]
    fn impacts_choose_their_effects_by_the_material_struck() {
        let response =
            |kind: i16, effect: Option<&str>, detonation: Option<&str>| MaterialResponse {
                default_response: kind,
                default_effect: effect.map(|path| tag(b"effe", path)),
                detonation_effect: detonation.map(|path| tag(b"effe", path)),
                ..MaterialResponse::default()
            };
        let projectile = Projectile {
            material_responses: vec![
                response(MaterialResponse::DISAPPEAR, Some("dirt"), Some("unmade")),
                response(MaterialResponse::DETONATE, Some("metal"), Some("sparks")),
                response(MaterialResponse::OVERPENETRATE, None, Some("through")),
                response(MaterialResponse::REFLECT, None, Some("unmade")),
            ],
            ..Projectile::default()
        };
        let paths = |material, on_object| -> Vec<&str> {
            response_effects(&projectile, material, on_object)
                .into_iter()
                .map(|tag| tag.path.as_str())
                .collect()
        };
        // (the response's effect, and its detonation's only where it
        // detonates)
        assert_eq!(paths(0, false), ["dirt"]);
        assert_eq!(paths(1, false), ["metal", "sparks"]);
        assert_eq!(paths(2, false), ["through"]);
        assert!(paths(2, true).is_empty());
        assert!(paths(3, false).is_empty());
        // (past its responses, or none known, the default makes none)
        assert!(paths(4, false).is_empty());
        assert!(paths(-1, false).is_empty());
        // (an effect made where it strikes faces out of the surface)
        assert!((facing(Vec3::Z) * Vec3::X - Vec3::Z).length() < 1e-5);
        assert_eq!(facing(Vec3::ZERO), Quat::IDENTITY);
    }
}
