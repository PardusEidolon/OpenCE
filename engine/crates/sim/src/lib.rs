//! The simulation: a `bevy_ecs` world stepped on a fixed tick.
//!
//! Systems run on one thread, in the order of [`SimSet`], so the same
//! inputs from the same state give the same world on every run; the maths
//! is `libm`'s and glam's (built on `libm`), never the platform's, and Rust
//! never fuses a multiply and an add unless asked. Only the simulation's
//! own types cross into `ffi`, which turns them into the snapshot; Swift
//! never sees the world.

pub mod ai_impulse;
pub mod animation;
pub mod animation_blend;
pub mod animation_graph;
pub mod announcer;
pub mod armory;
pub mod cached;
pub mod clock;
pub mod crates;
pub mod damage;
mod dead_camera;
pub mod debris;
pub mod device_animation;
pub mod effects;
pub mod facing;
pub mod first_person;
pub mod first_person_states;
pub mod forces;
pub mod functions;
pub mod game;
pub mod hash;
pub mod hud;
pub mod input;
pub mod item_physics;
pub mod objects;
pub mod particle_collision;
pub mod periodic;
pub mod physics_variables;
pub mod pickup;
pub mod platforms;
pub mod player;
pub mod point_physics;
pub mod projectile;
pub mod ragdoll;
pub mod random;
pub mod real_math;
pub mod safety;
pub mod scenery;
pub mod seats;
pub mod unit_aiming;
pub mod unit_animator;
pub mod unit_control;
pub mod unit_definition;
pub mod unit_dialogue;
pub mod unit_inventory;
pub mod unit_items;
pub mod unit_life;
pub mod unit_questions;
pub mod unit_scripting;
pub mod unit_states;
pub mod units;
pub mod vehicle;
pub mod vehicle_animation;
pub mod weapon;
pub mod weapon_markers;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SingleThreadedExecutor;
use cached::Cached;
pub use glam::{Mat3, Quat, Vec3};

use animation::{AnimationPlayer, NodeMatrices};
use hash::WorldHasher;
use input::{Buttons, InputRecord, TickInput};
use objects::{
    kind_index, Object, ObjectHandle, ObjectKinds, ObjectName, ObjectSpawn, ObjectTable,
    Permutation, TableFull, Team,
};
use opence_content::{Level, MaterialEffects, ObjectKind, TagRef};
use player::{Player, PlayerError, PlayerPoses, View};
use units::{Radians, WorldUnits};

/// A position and an orientation in the world, in world units.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// The position, in world units.
    pub position: Vec3,
    /// The orientation.
    pub rotation: Quat,
    /// The uniform scale.
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: 1.0,
        }
    }
}

impl Transform {
    fn hash(&self, hasher: &mut WorldHasher) {
        for value in self
            .position
            .to_array()
            .into_iter()
            .chain(self.rotation.to_array())
            .chain([self.scale])
        {
            hasher.f32(value);
        }
    }
}

/// An object's id: the same from tick to tick, and the order the world
/// hash takes objects in.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub u32);

/// The camera the world is seen from.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    /// Where it is and where it looks.
    pub transform: Transform,
    /// The vertical field of view; the horizontal one grows with the
    /// aspect ratio (Hor+).
    pub vertical_fov: Radians,
}

impl Default for Camera {
    fn default() -> Self {
        // (70 degrees vertical: about the stock 70 horizontal at 4:3,
        // widened for today's displays)
        Self {
            transform: Transform::default(),
            vertical_fov: Radians(70.0_f32.to_radians()),
        }
    }
}

/// The free camera's state: where it is and which way it faces. Halo's
/// world is Z up, and a yaw of zero looks along +X.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FreeCamera {
    /// Where it is.
    pub position: Vec3,
    /// The turn to the left from +X.
    pub yaw: Radians,
    /// The turn up from level.
    pub pitch: Radians,
}

impl FreeCamera {
    /// Its speed, in world units per second (a world unit is about 3 m).
    pub const SPEED: WorldUnits = WorldUnits(2.0);
    /// Its speed with the sprint button.
    pub const SPRINT_SPEED: WorldUnits = WorldUnits(8.0);
    /// How far it looks up or down: just short of straight, where the yaw
    /// would flip.
    pub const MAX_PITCH: Radians = Radians(std::f32::consts::FRAC_PI_2 - 0.001);

    /// Its orientation: a yaw about Z, then a pitch about the camera's Y.
    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_z(self.yaw.0) * Quat::from_rotation_y(-self.pitch.0)
    }
}

/// The number of ticks run since the world started.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u64);

/// This tick's input.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CurrentInput(pub TickInput);

/// The stages of a tick, run in this order. A new system goes in its stage
/// with `in_set`; nothing else needs to know of it.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimSet {
    /// What the tick's input sets directly (the camera's look).
    Input,
    /// The world moving.
    Simulate,
    /// What follows from the world's new state (the camera's transform,
    /// the tick counter).
    Finish,
}

/// A running world.
pub struct Simulation {
    world: World,
    schedule: Schedule,
    /// The players, the local player (the camera's, the HUD's) first
    /// (their physics worlds are not `Sync`, so they are kept beside the
    /// bevy world, not in it).
    players: Vec<Player>,
    /// Where projectiles hit in the last tick.
    impacts: Vec<projectile::Impact>,
    /// Where grenades detonated in the last tick.
    detonations: Vec<projectile::Detonation>,
    /// What damage did in the last tick.
    damage: Vec<damage::DamageEvent>,
    /// The player's motion sensor (shown, not the world's state).
    sensor: hud::MotionSensor,
    /// Its settings, from the level's HUD globals.
    sensor_settings: Option<hud::SensorSettings>,
    /// What the local player's action button would do, this tick.
    prompt: Option<hud::ActionPrompt>,
    /// The player's first-person weapon and hands (shown, not the
    /// world's state).
    first_person: first_person::FirstPersonWeapon,
    /// The dead bipeds' ragdolls, in the player's physics world.
    ragdolls: ragdoll::Ragdolls,
    /// The spent casings, in the player's physics world.
    debris: debris::Debris,
    /// The level's weapons' markers, where their effects are made.
    markers: weapon_markers::WeaponMarkers,
    /// The physics station's crates, in the player's physics world.
    crates: crates::Crates,
    /// The machines, as platforms in the player's physics world.
    platforms: platforms::Platforms,
    /// The scenery, in the level's collision in the players' worlds.
    scenery: scenery::SceneryCollision,
    /// Detonations to do in the next tick (a script's).
    queued_detonations: Vec<projectile::Detonation>,
    /// What the items lying about met this tick.
    item_impacts: Vec<item_physics::ItemImpact>,
    /// The effects the players' shots made this tick.
    effects: Vec<debris::Ejection>,
    /// The sounds played by themselves in the last tick.
    sounds: Vec<player::PlayedSound>,
    /// The vehicles, and the seat the player sits in.
    rides: seats::Rides,
    /// The AI's actors: bipeds moved and fired as the players are, by the
    /// inputs the AI gives them, in the local player's physics world.
    actors: Vec<Player>,
    /// The inputs the actors take on the next tick.
    actor_inputs: Vec<TickInput>,
    /// The queries asked each frame, kept.
    queries: Queries,
    /// The local player's camera while it is dead.
    dead_camera: Option<dead_camera::DeadCamera>,
    /// The random numbers no other machine shares (`local_random_seed`),
    /// which the camera draws from.
    local_random: random::RandomSeed,
}

/// What [`Simulation::visit_lit`] reads of each object.
type LitQuery = (
    &'static ObjectId,
    &'static objects::Object,
    &'static Transform,
    Option<&'static PreviousTransform>,
    Option<&'static NodeMatrices>,
    Option<&'static functions::FunctionValues>,
    Option<&'static objects::Appearance>,
);

/// What [`Simulation::visit_drawn`] reads of each object.
type DrawnQuery = (
    &'static ObjectId,
    &'static Transform,
    Option<&'static PreviousTransform>,
    Option<&'static NodeMatrices>,
    Option<&'static objects::Appearance>,
);

/// The queries [`Simulation`] asks each tick or frame, kept from one to the
/// next (see [`cached`]).
#[derive(Default)]
struct Queries {
    lit: Cached<LitQuery>,
    drawn: Cached<DrawnQuery>,
    handles: Cached<&'static ObjectHandle>,
    events: Cached<(
        &'static ObjectHandle,
        &'static objects::Object,
        &'static Transform,
        &'static AnimationPlayer,
    )>,
    functions: functions::Objects,
}

/// An object as the renderer draws it.
#[derive(Clone, Copy, Debug)]
pub struct Drawn<'a> {
    /// Its id.
    pub id: ObjectId,
    /// Where it was at the tick before the last.
    pub previous: &'a Transform,
    /// Where it was at the last tick.
    pub current: &'a Transform,
    /// Its nodes, if it is animated.
    pub nodes: Option<&'a NodeMatrices>,
    /// How it is drawn, if it has been told.
    pub appearance: Option<&'a objects::Appearance>,
}

/// An object that may carry lights, as [`Simulation::visit_lit`] gives
/// it.
pub struct Lit<'a> {
    /// Its id.
    pub id: ObjectId,
    /// Its tag.
    pub tag: &'a TagRef,
    /// Where it was at the tick before the last.
    pub previous: &'a Transform,
    /// Where it was at the last tick.
    pub current: &'a Transform,
    /// Its nodes, if it is animated.
    pub nodes: Option<&'a NodeMatrices>,
    /// Its functions' values, if it has any.
    pub functions: Option<&'a functions::FunctionValues>,
    /// How it is drawn, if that has been set.
    pub appearance: Option<&'a objects::Appearance>,
    /// It is the body the player sees through.
    pub viewer: bool,
}

impl Simulation {
    /// A world with nothing in it but the free camera.
    pub fn empty() -> Self {
        let mut world = World::new();
        world.insert_resource(Tick::default());
        world.insert_resource(Camera::default());
        world.insert_resource(FreeCamera::default());
        world.insert_resource(CurrentInput::default());
        world.insert_resource(ObjectTable::default());
        world.insert_resource(ObjectKinds::default());
        world.insert_resource(random::RandomSeed::default());
        world.insert_resource(Snaps::default());
        world.insert_resource(unit_animator::ExitedSeats::default());
        let mut schedule = Schedule::default();
        // (one thread: systems in parallel would be ordered by chance)
        schedule.set_executor(SingleThreadedExecutor::new());
        schedule.configure_sets((SimSet::Input, SimSet::Simulate, SimSet::Finish).chain());
        schedule.add_systems((
            (look_free_camera, remember_transforms, damage::give_vitality).in_set(SimSet::Input),
            (
                move_free_camera,
                player::move_player.before(animation::play_animations),
                player::move_projectiles,
                seats::move_rides.before(animation::play_animations),
                damage::recover_vitality,
                unit_animator::animate_units
                    .after(player::move_player)
                    .after(seats::move_rides)
                    .before(animation::play_animations),
                animation::play_animations,
            )
                .in_set(SimSet::Simulate),
            (snap_transforms, place_camera, advance_tick)
                .chain()
                .in_set(SimSet::Finish),
        ));
        Self {
            world,
            schedule,
            players: Vec::new(),
            impacts: Vec::new(),
            detonations: Vec::new(),
            damage: Vec::new(),
            sensor: hud::MotionSensor::default(),
            sensor_settings: None,
            prompt: None,
            first_person: first_person::FirstPersonWeapon::default(),
            ragdolls: ragdoll::Ragdolls::default(),
            debris: debris::Debris::default(),
            markers: weapon_markers::WeaponMarkers::default(),
            crates: crates::Crates::default(),
            platforms: platforms::Platforms::default(),
            scenery: scenery::SceneryCollision::default(),
            queued_detonations: Vec::new(),
            item_impacts: Vec::new(),
            effects: Vec::new(),
            sounds: Vec::new(),
            rides: seats::Rides::default(),
            actors: Vec::new(),
            actor_inputs: Vec::new(),
            queries: Queries::default(),
            dead_camera: None,
            local_random: random::RandomSeed::default(),
        }
    }

    /// Runs one tick with `input` for the local player: the player's
    /// biped moves first, then the world's systems run.
    pub fn step(&mut self, input: TickInput) {
        self.step_all(&[input]);
    }

    /// Runs one tick with each player's input, in the players' order (a
    /// player past the inputs stands still): their bipeds move and fire,
    /// the vehicles move, then the world's systems run. The free camera
    /// takes the first input (`game_tick`).
    /// (the original's `units_update`, in part)
    /// (the original's `players_update_before_game`, in part)
    pub fn step_all(&mut self, inputs: &[TickInput]) {
        // (the actors move and fire after the players, as more of them)
        let humans = self.players.len();
        let first = inputs.first().copied().unwrap_or_default();
        let mut inputs: Vec<TickInput> = (0..humans)
            .map(|index| inputs.get(index).copied().unwrap_or_default())
            .collect();
        inputs.extend(
            (0..self.actors.len())
                .map(|index| self.actor_inputs.get(index).copied().unwrap_or_default()),
        );
        self.actor_inputs.clear();
        self.players.append(&mut self.actors);
        self.step_everyone(first, &inputs);
        self.actors = self.players.split_off(humans.min(self.players.len()));
        self.place_dead_camera();
        self.update_motion_sensor();
        self.update_first_person();
    }

    /// One tick of the players and the actors among them.
    /// (the original's `unit_update`, in part)
    fn step_everyone(&mut self, first: TickInput, inputs: &[TickInput]) {
        self.world.insert_resource(CurrentInput(first));
        self.world
            .insert_resource(player::ProjectilePoses::default());
        self.update_scenery();
        self.impacts.clear();
        self.detonations.clear();
        self.damage.clear();
        let mut ejections = Vec::new();
        self.sounds.clear();
        // (the dead neither move nor fire, though they still fall)
        let inputs: Vec<TickInput> = (0..self.players.len())
            .map(|index| {
                let dead = self
                    .players
                    .get(index)
                    .is_some_and(|player| self.is_dead(player.handle()));
                match inputs.get(index) {
                    Some(input) if !dead => *input,
                    _ => TickInput::default(),
                }
            })
            .collect();
        let mut actions = Vec::with_capacity(self.players.len());
        for (player, input) in self.players.iter_mut().zip(&inputs) {
            player.step(input);
        }
        if !self.players.is_empty() {
            self.rides.adopt(&mut self.world);
        }
        // (the action button enters or leaves a seat before it picks up a
        // weapon; one player at a time, so two do not take one seat)
        let busy: Vec<bool> = self
            .players
            .iter()
            .map(|player| {
                self.read_component(player.handle(), |animator: &unit_animator::UnitAnimator| {
                    unit_states::busy(animator.animation.state)
                })
                .unwrap_or(false)
            })
            .collect();
        for index in 0..self.players.len() {
            let taken: Vec<player::Seated> =
                self.players.iter().filter_map(Player::seated).collect();
            let (Some(player), Some(input)) = (self.players.get_mut(index), inputs.get(index))
            else {
                continue;
            };
            let action = input
                .buttons
                .pressed(player.held_buttons(), Buttons::ACTION);
            let flashlight = input
                .buttons
                .pressed(player.held_buttons(), Buttons::FLASHLIGHT);
            step_flashlight(&mut self.world, player, flashlight);
            if index == 0 {
                // (as `hud_show_action_response` shows the action result:
                // a vehicle, then a weapon, else the reminder to switch)
                self.prompt = self
                    .rides
                    .prompt(&self.world, player, &taken)
                    .or_else(|| {
                        player
                            .swap_offer(&mut self.world)
                            .map(|weapon| hud::ActionPrompt::SwapWeapon { weapon })
                    })
                    .or_else(|| {
                        player
                            .switch_reminder()
                            .map(|weapon| hud::ActionPrompt::RemindToSwitch { weapon })
                    });
            }
            let busy = busy.get(index).copied().unwrap_or(false);
            let seated = self.rides.act(&self.world, player, &taken, action, busy);
            actions.push(action && !seated);
        }
        self.rides.step(&mut self.world, &mut self.players, &inputs);
        if !self.crates.all().is_empty() {
            let pushers = self.rides.pushers();
            if let Some(player) = self.players.first() {
                self.crates.push_by(&mut player.physics(), &pushers);
            }
        }
        for strike in self.rides.strike(&self.world, &mut self.players) {
            self.damage.extend(damage::apply_collision(
                &mut self.world,
                strike.victim,
                strike.owner,
            ));
        }
        let poses = self.players.iter().map(Player::pose).collect();
        self.world.insert_resource(PlayerPoses(poses));
        let drives = self
            .players
            .iter()
            .map(|player| player.unit_drive(&self.rides))
            .collect();
        self.world
            .insert_resource(unit_animator::UnitDrives(drives));
        for ((player, input), pick_up) in self.players.iter_mut().zip(&inputs).zip(actions) {
            let arms = self.rides.arms(player.seated(), pick_up);
            let mut fired = player.step_weapons(input, &mut self.world, arms);
            ejections.append(&mut fired.ejections);
            self.sounds.append(&mut fired.sounds);
            let handle = player.handle();
            let mut damage =
                damage::apply_impacts(&mut self.world, &mut fired.impacts, Some(handle));
            damage.extend(damage::apply_detonations(
                &mut self.world,
                &fired.detonations,
                Some(handle),
                |from, to| player.level_clear(from, to),
            ));
            // (as the game's biped_falling_damage, each tick: a landing's
            // hurt, or a fall faster than any lived through)
            let landing = player.landed().unwrap_or(0.0);
            let upward = player.mover().velocity.z;
            damage.extend(damage::apply_fall(&mut self.world, handle, landing, upward));
            self.impacts.append(&mut fired.impacts);
            self.detonations.append(&mut fired.detonations);
            self.damage.append(&mut damage);
        }
        if let Some(player) = self.players.first() {
            let queued = std::mem::take(&mut self.queued_detonations);
            self.damage.extend(damage::apply_detonations(
                &mut self.world,
                &queued,
                Some(player.handle()),
                |from, to| player.level_clear(from, to),
            ));
            self.detonations.extend(queued);
        }
        self.queued_detonations.clear();
        // (what the damage struck is pushed, after all of it is done)
        let pushes = match self.players.first() {
            Some(player) => {
                let mut pushes = forces::impact_pushes(&self.world, &self.impacts);
                pushes.extend(forces::detonation_pushes(
                    &mut self.world,
                    &self.detonations,
                    |from, to| player.level_clear(from, to),
                ));
                pushes
            }
            None => Vec::new(),
        };
        forces::apply(&mut self.world, &mut self.rides, &mut self.players, &pushes);
        self.schedule.run(&mut self.world);
        self.stand_exited_riders();
        let rides = self.rides.exports();
        functions::update(&mut self.world, &mut self.queries.functions, &rides);
        self.place_ejections(&mut ejections);
        self.update_physics(&ejections);
        self.effects = ejections;
    }

    /// Stands up each rider whose animation of leaving its seat ended this
    /// tick (`unit_exit_seat_end`), and any player leaving its seat whose
    /// biped has no animator to play it.
    fn stand_exited_riders(&mut self) {
        let exited = std::mem::take(&mut self.world.resource_mut::<unit_animator::ExitedSeats>().0);
        let animated: Vec<bool> = self
            .players
            .iter()
            .map(|player| {
                self.read_component(player.handle(), |_: &unit_animator::UnitAnimator| ())
                    .is_some()
            })
            .collect();
        for (player, animated) in self.players.iter_mut().zip(animated) {
            if player.exiting() && (!animated || exited.contains(&player.handle())) {
                self.rides.stand(player);
            }
        }
    }

    /// Puts each weapon's effects at its markers, as `weapon_effect_new`
    /// makes them at the weapon's: the local player's first-person
    /// weapon's, posed from the camera, or else the weapon's own model's,
    /// held at its grip in its holder's hand. Each is at the first of its
    /// locations found, with every one found among its locations.
    fn place_ejections(&self, ejections: &mut [debris::Ejection]) {
        let camera = self.world.resource::<Camera>().transform;
        let local = self.players.first().map(Player::handle);
        for ejection in ejections.iter_mut() {
            let (Some(holder), Some(weapon)) = (ejection.holder, ejection.weapon.as_ref()) else {
                continue;
            };
            let first_person = self
                .first_person
                .posed()
                .filter(|(shown, _)| local == Some(holder) && *shown == weapon)
                .map(|(_, posed)| *posed)
                .filter(|_| {
                    self.players
                        .first()
                        .is_some_and(|player| player.view() == View::FirstPerson)
                });
            let hand = match first_person {
                Some(_) => None,
                None => self.held_hand(holder),
            };
            let mut found = Vec::new();
            for name in self.markers.locations(&ejection.effect) {
                let matrix = match (&first_person, &hand) {
                    (Some(posed), _) => {
                        self.markers
                            .first_person(self.first_person.models(), posed, &camera, name)
                    }
                    (None, Some((matrix, grip))) => self.markers.held(weapon, matrix, grip, name),
                    (None, None) => None,
                };
                if let Some(matrix) = matrix {
                    let (position, rotation) = weapon_markers::frame(&matrix);
                    found.push((name.clone(), position, rotation));
                }
            }
            if let Some(&(_, position, rotation)) = found.first() {
                ejection.position = position;
                ejection.rotation = rotation;
            }
            ejection.locations = found;
        }
    }

    /// The hand marker of the unit `holder` that holds its weapon, in the
    /// world at the last tick, and the weapon's marker it grips.
    fn held_hand(&self, holder: ObjectHandle) -> Option<(animation::NodeMatrix, String)> {
        let entity = self.entity(holder)?;
        let player = entity.get::<AnimationPlayer>()?;
        let model = player.model.as_ref()?;
        let nodes = entity.get::<NodeMatrices>()?;
        let animator = entity.get::<unit_animator::UnitAnimator>()?;
        let seat = animator.animation.seat?;
        let class = player
            .graph
            .unit_seats
            .get(seat.seat)?
            .weapon_classes
            .get(seat.class)?;
        let hand = seats::marker_matrix(model, &nodes.current, &class.hand_marker)?;
        Some((hand, class.grip_marker.clone()))
    }

    /// The physics of the dead, the spent and the dropped: the items lying
    /// about, a game tick every other tick; the machines moved to their
    /// animations; the ragdolls, this tick's casings, the crates and what
    /// the machines carry, stepped once if any moves.
    fn update_physics(&mut self, ejections: &[debris::Ejection]) {
        let tick = self.tick().0;
        let Some(player) = self.players.first_mut() else {
            return;
        };
        let killed: Vec<ObjectHandle> = self
            .damage
            .iter()
            .filter(|event| event.outcome.killed)
            .map(|event| event.object)
            .filter(|&object| {
                let kinds = self.world.resource::<ObjectTable>();
                kinds
                    .entity(object)
                    .and_then(|entity| self.world.get::<objects::Object>(entity))
                    .is_some_and(|object| object.kind == ObjectKind::Biped)
            })
            .collect();
        let mut physics = player.physics();
        let physics = &mut *physics;
        let ragdolls = self
            .ragdolls
            .prepare(&mut self.world, physics, &killed, &self.detonations);
        for ejection in ejections {
            self.debris.eject(physics, ejection);
        }
        self.platforms.update(&mut self.world, physics);
        // (items move a game tick, of 30 a second, every other tick)
        let game_ticks = u64::from(clock::TICKS_PER_SECOND / vehicle::GAME_TICKS_PER_SECOND);
        self.item_impacts = if tick.is_multiple_of(game_ticks) {
            item_physics::step_lying(&mut self.world, physics)
        } else {
            Vec::new()
        };
        if ragdolls || self.debris.moving() || self.crates.moving() || self.platforms.moving() {
            // (a step the physics refuses leaves them where they were)
            let _ = physics.step(clock::TICK.0);
            self.ragdolls.settle(&mut self.world, physics);
            self.debris.settle(physics);
            self.crates.settle(physics);
        }
    }

    /// Plays the campaign's rules (after the players are made, which play
    /// multiplayer's): its headshots, and damage scaled by `values` (the
    /// globals' difficulty values) at `difficulty`, 0 easy to 3 legendary.
    pub fn play_campaign(&mut self, values: &[[f32; 4]], difficulty: usize) {
        if let Some(mut table) = self.world.get_resource_mut::<damage::DamageTable>() {
            table.multiplayer = false;
        }
        self.world
            .insert_resource(damage::DifficultyScale::new(values, difficulty));
    }

    /// Moves the players and the actors over `level`'s structure BSP
    /// `bsp`'s collision; fails, leaving them where they were, for a BSP
    /// the level does not have.
    pub fn switch_bsp(&mut self, level: &Level, bsp: usize) -> Result<(), PlayerError> {
        let structure = level
            .structure_bsps
            .get(bsp)
            .ok_or(PlayerError::NoBsp(bsp))?;
        let mesh = opence_physics::LevelMesh::from_collision(&structure.collision);
        // (the actors share the local player's world)
        for player in &self.players {
            player.physics().set_level(&mesh)?;
        }
        self.rides.set_bsp(level, bsp);
        self.scenery.set_bsp(level, bsp);
        self.world.insert_resource(self.rides.surfaces());
        Ok(())
    }

    /// Builds the level's collision again in each player's physics world
    /// if the scenery placed has changed (each world once: the actors
    /// share the local player's).
    fn update_scenery(&mut self) {
        if !self.scenery.stale(&mut self.world) {
            return;
        }
        let mut worlds: Vec<player::SharedPhysics> = Vec::new();
        for physics in self.players.iter().map(Player::shared_physics) {
            if !worlds
                .iter()
                .any(|other| std::sync::Arc::ptr_eq(other, &physics))
            {
                worlds.push(physics);
            }
        }
        let mut guards: Vec<_> = worlds.iter().map(player::lock).collect();
        // (a mesh the physics refuses leaves the level as it was)
        let _ = self
            .scenery
            .update(&mut self.world, guards.iter_mut().map(|guard| &mut **guard));
    }

    /// Makes the player: its biped standing at `level`'s player start
    /// `start`, moving over that start's structure BSP, the camera at its
    /// eyes. The players there were before are removed.
    /// (the original's `player_new`, in part)
    pub fn spawn_player(
        &mut self,
        level: &Level,
        start: usize,
    ) -> Result<ObjectHandle, PlayerError> {
        for earlier in std::mem::take(&mut self.actors) {
            self.remove(earlier.handle());
            earlier.leave(&mut self.world);
        }
        for earlier in std::mem::take(&mut self.players) {
            self.remove(earlier.handle());
            earlier.leave(&mut self.world);
        }
        self.world.remove_resource::<PlayerPoses>();
        let player = self.new_player(level, start, None)?;
        let handle = player.handle();
        // (multiplayer's rules: the scenario does not yet say which it is)
        self.world
            .insert_resource(damage::DamageTable::new(level, true));
        self.world
            .insert_resource(functions::FunctionTable::new(level));
        self.rides = seats::Rides::for_level(level, player.unit(), player.armory());
        let bsp = level
            .scenario
            .player_starts
            .get(start)
            .and_then(|start| start.structure_bsp)
            .unwrap_or(0);
        self.rides.set_bsp(level, usize::from(bsp));
        self.world.insert_resource(self.rides.surfaces());
        self.scenery = scenery::SceneryCollision::for_level(level);
        self.scenery.set_bsp(level, usize::from(bsp));
        self.players.push(player);
        self.sensor = hud::MotionSensor::default();
        self.sensor_settings = level
            .hud_globals
            .as_ref()
            .map(|globals| hud::SensorSettings {
                range: globals.motion_sensor_range,
                velocity_sensitivity: globals.motion_sensor_velocity_sensitivity,
                scale: globals.motion_sensor_scale,
                multiplayer: true,
            });
        self.first_person = first_person::FirstPersonWeapon::new(level);
        // (the earlier player's physics world took its ragdolls and its
        // casings with it)
        self.ragdolls = self.ragdolls.for_level(level);
        self.debris = self.debris.for_level(level);
        self.markers = weapon_markers::WeaponMarkers::for_level(level);
        self.crates.forget();
        self.platforms = platforms::Platforms::for_level(level);
        self.queued_detonations.clear();
        self.place_player_camera();
        self.update_first_person();
        Ok(handle)
    }

    /// Adds another player, after the local one: its biped standing at
    /// `level`'s player start `start`, on `team`. Its index among the
    /// players is returned with its biped.
    pub fn add_player(
        &mut self,
        level: &Level,
        start: usize,
        team: Option<u16>,
    ) -> Result<(usize, ObjectHandle), PlayerError> {
        let player = self.new_player(level, start, team)?;
        let handle = player.handle();
        self.players.push(player);
        Ok((self.players.len() - 1, handle))
    }

    /// Stands player `index` again at `level`'s player start `start` with
    /// a new biped on `team` and its starting weapons; its old biped stays
    /// where it fell. `None` if there is no such player.
    /// (the original's `player_spawn`, in part)
    pub fn respawn_player(
        &mut self,
        index: usize,
        level: &Level,
        start: usize,
        team: Option<u16>,
    ) -> Option<Result<ObjectHandle, PlayerError>> {
        let world = &mut self.world;
        let player = self.players.get_mut(index)?;
        let respawned = player.respawn(level, start, |tag, transform| {
            biped(world, tag, transform, team)
        });
        let handle = player.handle();
        if index == 0 && respawned.is_ok() {
            self.place_player_camera();
        }
        Some(respawned.map(|()| handle))
    }

    /// Takes player `index` out of the game, with its biped; false if
    /// there is no such player. The local player cannot be removed.
    pub fn remove_player(&mut self, index: usize) -> bool {
        if index == 0 || index >= self.players.len() {
            return false;
        }
        let player = self.players.remove(index);
        self.remove(player.handle());
        player.leave(&mut self.world);
        true
    }

    /// Adds an actor of the AI's: a biped of `unit` with its feet at
    /// `position`, facing `facing` (radians left from +X), on `team`,
    /// with `weapon` in its hands if the level has it, in the local
    /// player's physics world. Its index among the actors is returned
    /// with its biped.
    pub fn add_actor(
        &mut self,
        level: &Level,
        unit: &TagRef,
        weapon: Option<&TagRef>,
        (position, facing): (Vec3, f32),
        team: Option<u16>,
    ) -> Result<(usize, ObjectHandle), PlayerError> {
        let physics = self
            .players
            .first()
            .map(Player::shared_physics)
            .ok_or(PlayerError::NoPlayer)?;
        let body = level
            .object(unit)
            .and_then(|object| object.biped)
            .ok_or(PlayerError::NotBiped)?;
        let world = &mut self.world;
        let mut actor = Player::placed(
            level,
            physics,
            (unit.clone(), body),
            (position, facing),
            |tag, transform| biped(world, tag, transform, team),
        )?;
        if let Some(weapon) = weapon {
            actor.arm(weapon);
        }
        let handle = actor.handle();
        self.actors.push(actor);
        Ok((self.actors.len() - 1, handle))
    }

    /// Takes actor `index` out of the world, with its biped; false if
    /// there is no such actor. The later actors' indices go down by one.
    pub fn remove_actor(&mut self, index: usize) -> bool {
        if index >= self.actors.len() {
            return false;
        }
        let actor = self.actors.remove(index);
        self.actor_inputs.clear();
        self.remove(actor.handle());
        actor.leave(&mut self.world);
        true
    }

    /// The AI's actors.
    pub fn actors(&self) -> &[Player] {
        &self.actors
    }

    /// Sets the inputs the actors take on the next tick, in their order
    /// (an actor past them stands still).
    pub fn steer_actors(&mut self, inputs: &[TickInput]) {
        self.actor_inputs = inputs.to_vec();
    }

    /// Whether nothing of the level stands between `from` and `to`, in
    /// the local player's physics world (true without a player).
    pub fn level_clear(&self, from: Vec3, to: Vec3) -> bool {
        self.players
            .first()
            .is_none_or(|player| player.level_clear(from, to))
    }

    /// Where a point moving from `from` by `motion` first meets the level
    /// or a machine, in the local player's physics world (the particles'
    /// collision); none if it meets nothing, has no motion or there is no
    /// player.
    pub fn collide_point(&self, from: Vec3, motion: Vec3) -> Option<particle_collision::PointHit> {
        if !(from.is_finite() && motion.is_finite()) || motion.length_squared() == 0.0 {
            return None;
        }
        let player = self.players.first()?;
        let hit = player
            .physics()
            .cast_ray(from, motion, opence_physics::Layers::GROUND)
            .ok()??;
        Some(particle_collision::PointHit {
            point: hit.point,
            normal: hit.normal,
            fraction: hit.fraction,
        })
    }

    /// Puts player `index`'s biped at `position`, standing still; false
    /// if there is no such player, it is seated or the physics refused.
    pub fn teleport_player(&mut self, index: usize, position: Vec3) -> bool {
        let Some(player) = self.players.get_mut(index) else {
            return false;
        };
        let moved = player.teleport(position);
        if moved {
            // (its object is put there by the next tick, and drawn there
            // from the first, not blended across from where it was)
            let handle = player.handle();
            self.world.resource_mut::<Snaps>().0.push(handle);
            if index == 0 {
                self.place_player_camera();
            }
        }
        moved
    }

    /// Puts player `index` at `position`, moving at `velocity` and facing
    /// `yaw`, as a network game's host says it is (a prediction
    /// corrected); false if there is no such player, or it is seated.
    pub fn correct_player(
        &mut self,
        index: usize,
        position: Vec3,
        velocity: Vec3,
        yaw: Radians,
    ) -> bool {
        let moved = self
            .players
            .get_mut(index)
            .is_some_and(|player| player.correct(position, velocity, yaw.0));
        if moved && index == 0 {
            self.place_player_camera();
        }
        moved
    }

    /// The players, the local player first.
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// The dangers around the players this tick that the engine weighs:
    /// whether a player's biped is dead. The others (enemies that see a
    /// player, projectiles, items, effects and vehicles near one, units
    /// throwing or dying, a player in the air) are not yet weighed.
    pub fn dangers(&self) -> safety::Dangers {
        safety::Dangers {
            player_dead: self
                .players
                .iter()
                .any(|player| self.is_dead(player.handle())),
            ..safety::Dangers::default()
        }
    }

    /// A player standing at `level`'s player start `start`, its biped on
    /// `team`.
    fn new_player(
        &mut self,
        level: &Level,
        start: usize,
        team: Option<u16>,
    ) -> Result<Player, PlayerError> {
        let world = &mut self.world;
        Player::new(level, start, |tag, transform| {
            biped(world, tag, transform, team)
        })
    }

    /// Where projectiles hit in the last tick.
    pub fn impacts(&self) -> &[projectile::Impact] {
        &self.impacts
    }

    /// What the items lying about met in the last tick.
    pub fn item_impacts(&self) -> &[item_physics::ItemImpact] {
        &self.item_impacts
    }

    /// Where the vehicles' mass points slipped on the ground in the last
    /// tick.
    pub fn slips(&self) -> &[seats::VehicleSlip] {
        self.rides.slips()
    }

    /// Where the vehicles' thrusters' blasts met the ground in the last
    /// tick.
    pub fn thrusts(&self) -> &[seats::VehicleThrust] {
        self.rides.thrusts()
    }

    /// Where grenades detonated in the last tick.
    pub fn detonations(&self) -> &[projectile::Detonation] {
        &self.detonations
    }

    /// The effects the players' shots made this tick (their weapons'
    /// firing effects, at their muzzles).
    pub fn effects(&self) -> &[debris::Ejection] {
        &self.effects
    }

    /// The sounds the players' weapons and pickups made by themselves this
    /// tick (a ready, reload or chamber sound; a pickup's).
    pub fn sounds(&self) -> &[player::PlayedSound] {
        &self.sounds
    }

    /// Detonates `detonation` in the next tick, as a grenade would (a
    /// script's explosion); nothing without a player.
    pub fn detonate(&mut self, detonation: projectile::Detonation) {
        if !self.players.is_empty() {
            self.queued_detonations.push(detonation);
        }
    }

    /// The dead bipeds' ragdolls.
    pub fn ragdolls(&self) -> &ragdoll::Ragdolls {
        &self.ragdolls
    }

    /// Sets the most corpses kept; the oldest goes first.
    pub fn set_corpses(&mut self, corpses: usize) {
        self.ragdolls.set_corpses(corpses);
    }

    /// The spent casings.
    pub fn debris(&self) -> &debris::Debris {
        &self.debris
    }

    /// The crates dropped.
    pub fn crates(&self) -> &crates::Crates {
        &self.crates
    }

    /// The machines as platforms.
    pub fn platforms(&self) -> &platforms::Platforms {
        &self.platforms
    }

    /// Drops a crate of `half_extents` at `position`, turned by
    /// `rotation`, into the player's physics world; false without a
    /// player, past the most crates, or for a size the physics refuses.
    pub fn drop_crate(&mut self, position: Vec3, rotation: Quat, half_extents: Vec3) -> bool {
        let Some(player) = self.players.first() else {
            return false;
        };
        let mut physics = player.physics();
        self.crates
            .drop_crate(&mut physics, position, rotation, half_extents)
    }

    /// Removes every crate.
    pub fn clear_crates(&mut self) {
        let Some(player) = self.players.first() else {
            return;
        };
        let mut physics = player.physics();
        self.crates.clear(&mut physics);
    }

    /// Sets the most casings kept; the oldest goes first.
    pub fn set_debris(&mut self, pieces: usize) {
        self.debris.set_cap(pieces);
    }

    /// Sets whether explosions tear limbs off (off by default: the original
    /// tears none).
    pub fn set_dismemberment(&mut self, dismemberment: bool) {
        self.ragdolls.set_dismemberment(dismemberment);
    }

    /// Gives the player `count` grenades of `kind` (0 frag, 1 plasma), up
    /// to the most it carries; false without a player.
    pub fn give_grenades(&mut self, kind: usize, count: i16) -> bool {
        let Some(player) = self.players.first_mut() else {
            return false;
        };
        player.give_grenades(kind, count);
        true
    }

    /// Takes all the player's weapons, as a level that starts it unarmed
    /// does; false without a player.
    pub fn disarm(&mut self) -> bool {
        let Some(player) = self.players.first_mut() else {
            return false;
        };
        player.disarm();
        self.update_first_person();
        true
    }

    /// Puts `weapon` in the player's hands, full, if the level has it
    /// (dropping what a full hand held); false without a player or for a
    /// weapon the level does not have.
    pub fn give_weapon(&mut self, weapon: &TagRef) -> bool {
        let given = self
            .players
            .first_mut()
            .is_some_and(|player| player.arm(weapon));
        if given {
            self.update_first_person();
        }
        given
    }

    /// Sits the player in seat `seat` of the vehicle `vehicle` at once;
    /// false without a player, for an object that is not a vehicle that
    /// can be ridden, or for a seat it does not have.
    pub fn enter_vehicle(&mut self, vehicle: ObjectHandle, seat: usize) -> bool {
        let Some(player) = self.players.first_mut() else {
            return false;
        };
        self.rides.seat_player(&self.world, player, vehicle, seat)
    }

    /// Makes the player fly through the level where it looks (true), or
    /// fall back into it; false without a player, or while it is seated.
    pub fn set_noclip(&mut self, noclip: bool) -> bool {
        let Some(player) = self.players.first_mut() else {
            return false;
        };
        if player.seated().is_some() {
            return false;
        }
        player.set_noclip(noclip);
        true
    }

    /// Suspends the unit `handle` (true), holding it still where it is,
    /// or lets it go, as `unit_scripting_suspended` does; false if it is
    /// no player's or actor's biped.
    pub fn suspend(&mut self, handle: ObjectHandle, suspended: bool) -> bool {
        let unit = self
            .players
            .iter_mut()
            .chain(self.actors.iter_mut())
            .find(|unit| unit.handle() == handle);
        let Some(unit) = unit else {
            return false;
        };
        unit.set_suspended(suspended);
        true
    }

    /// Makes an object take no damage (true), or take it again; false if
    /// `handle` is not in the world.
    pub fn set_invulnerable(&mut self, handle: ObjectHandle, invulnerable: bool) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let mut entity = self.world.entity_mut(entity);
        if invulnerable {
            entity.insert(damage::Invulnerable);
        } else {
            entity.remove::<damage::Invulnerable>();
        }
        true
    }

    /// What damage did in the last tick.
    pub fn damage_events(&self) -> &[damage::DamageEvent] {
        &self.damage
    }

    /// One tick of the motion sensor: a game tick (of 30 a second) every
    /// other tick, scanning the units other than the player's.
    fn update_motion_sensor(&mut self) {
        let (Some(player), Some(settings)) = (self.players.first(), self.sensor_settings) else {
            return;
        };
        let tick = self.world.resource::<Tick>().0;
        // (the tick has advanced: its first is 1)
        let ticks_per_game_tick = u64::from(clock::TICKS_PER_SECOND / 30);
        if !tick.is_multiple_of(ticks_per_game_tick) {
            return;
        }
        let own = player.handle();
        let viewer = self.world.resource::<Camera>().transform.position;
        let own_team = self
            .world
            .resource::<ObjectTable>()
            .entity(own)
            .and_then(|entity| self.world.get::<Team>(entity))
            .copied();
        let world = &mut self.world;
        let sensor = &mut self.sensor;
        sensor.update(tick / ticks_per_game_tick - 1, || {
            let mut units = world.query_filtered::<(
                &ObjectHandle,
                &Object,
                &Transform,
                &PreviousTransform,
                Option<&Team>,
                Option<&damage::Vitality>,
            ), With<objects::Unit>>();
            let mut contacts: Vec<(ObjectHandle, hud::Contact)> = units
                .iter(world)
                .filter(|(handle, _, _, _, _, vitality)| {
                    **handle != own && !vitality.is_some_and(|vitality| vitality.dead)
                })
                .map(|(handle, object, transform, previous, team, _)| {
                    let vehicle = object.kind == ObjectKind::Vehicle;
                    let friend = own_team.is_some() && team.copied() == own_team;
                    let kind = match (vehicle, friend) {
                        (false, true) => hud::BlipKind::Friend,
                        (false, false) => hud::BlipKind::Enemy,
                        (true, true) => hud::BlipKind::VehicleFriend,
                        (true, false) => hud::BlipKind::VehicleEnemy,
                    };
                    // (a game tick is two of ours)
                    let step =
                        (transform.position - previous.0.position) * ticks_per_game_tick as f32;
                    (
                        *handle,
                        hud::Contact {
                            position: transform.position,
                            speed_squared: step.length_squared(),
                            firing: false,
                            camouflaged: false,
                            kind,
                            size: if vehicle {
                                hud::BlipSize::Large
                            } else {
                                hud::BlipSize::Medium
                            },
                        },
                    )
                })
                .collect();
            // (the same order on every run)
            contacts.sort_by_key(|(handle, _)| *handle);
            let contacts: Vec<hud::Contact> =
                contacts.into_iter().map(|(_, contact)| contact).collect();
            hud::scan(viewer, &contacts, &settings)
        });
    }

    /// One tick of the first-person weapon, posed from the camera: shown
    /// only out of a living player's eyes.
    /// (the original's `first_person_weapons_update`, in part)
    fn update_first_person(&mut self) {
        let Some(player) = self.players.first() else {
            self.first_person.hide();
            return;
        };
        let dead = self.is_dead(player.handle());
        if dead || player.view() != View::FirstPerson {
            self.first_person.hide();
            return;
        }
        let tick = self.world.resource::<Tick>().0;
        let camera = self.world.resource::<Camera>().transform;
        let grenades = player.inventory().grenades.iter().sum();
        let throttle = player.throttle();
        let (yaw, pitch) = player.look();
        let holder = first_person::Holder {
            throttle: [throttle.x, throttle.y],
            airborne: player.mover().airborne,
            look: [yaw.0, pitch.0],
            melee: player.began_melee(),
        };
        self.first_person
            .update(tick, player.weapon(), grenades, &holder, &camera);
        if let Some(sound) = self.first_person.sound() {
            // (heard by the player alone, as an impulse on the weapon)
            self.sounds.push(player::PlayedSound {
                sound: sound.clone(),
                position: None,
                // (all of it, as `first_person_weapon_update` plays it)
                scale: 1.0,
            });
        }
    }

    /// The player's first-person weapon and hands.
    pub fn first_person(&self) -> &first_person::FirstPersonWeapon {
        &self.first_person
    }

    /// What the HUD shows: the player's shield, health, weapon, grenades
    /// and motion sensor; `None` without a player.
    pub fn hud(&self) -> Option<hud::HudState> {
        let player = self.players.first()?;
        let vitality = self.component::<damage::Vitality>(player.handle());
        let (yaw, _) = player.look();
        let inventory = player.inventory();
        Some(hud::HudState {
            unit: player.unit().clone(),
            shield: vitality.as_ref().map_or(1.0, |vitality| vitality.shield),
            body: vitality.as_ref().map_or(1.0, |vitality| vitality.body),
            shield_depleted: vitality
                .as_ref()
                .is_some_and(|vitality| vitality.shield_depleted),
            dead: vitality.as_ref().is_some_and(|vitality| vitality.dead),
            weapon: player.weapon().map(|(tag, state)| hud::WeaponReadout {
                weapon: tag.clone(),
                magazines: state.magazines.iter().map(hud::magazine_count).collect(),
                heat: state.heat,
                age: state.age,
                overheated: state.overheated,
            }),
            grenades: inventory.grenades,
            grenade: inventory.grenade,
            blips: self
                .sensor_settings
                .map(|settings| self.sensor.blips(yaw.0, &settings))
                .unwrap_or_default(),
            prompt: self.prompt.clone(),
            panels: self.vehicle_panels(player),
        })
    }

    /// The panels a driver sees under its own (`hud_render_unit_interface`):
    /// its vehicle's HUD, then the HUD of each rider's seat, at that unit's
    /// shield and health; none in any other seat. (The original takes the
    /// vehicle's children newest first; here the riders are the players, in
    /// their order.)
    fn vehicle_panels(&self, player: &Player) -> Vec<hud::UnitPanel> {
        let Some(seated) = player.seated() else {
            return Vec::new();
        };
        if seated.flags & opence_content::VehicleSeat::DRIVER == 0 {
            return Vec::new();
        }
        let Some((hud, seats)) = self.rides.huds(seated.vehicle) else {
            return Vec::new();
        };
        let panel = |hud: &TagRef, unit: ObjectHandle| {
            let vitality = self.component::<damage::Vitality>(unit);
            hud::UnitPanel {
                hud: hud.clone(),
                shield: vitality.as_ref().map_or(1.0, |vitality| vitality.shield),
                body: vitality.as_ref().map_or(1.0, |vitality| vitality.body),
                shield_depleted: vitality
                    .as_ref()
                    .is_some_and(|vitality| vitality.shield_depleted),
                dead: vitality.as_ref().is_some_and(|vitality| vitality.dead),
            }
        };
        let mut panels: Vec<_> = hud.iter().map(|hud| panel(hud, seated.vehicle)).collect();
        for rider in &self.players {
            let Some(riding) = rider
                .seated()
                .filter(|riding| riding.vehicle == seated.vehicle)
            else {
                continue;
            };
            if let Some(hud) = seats.get(riding.seat).copied().flatten() {
                panels.push(panel(hud, rider.handle()));
            }
        }
        panels
    }

    /// The local player, if there is one.
    pub fn player(&self) -> Option<&Player> {
        self.players.first()
    }

    /// The vehicles in the world, and their seats.
    pub fn rides(&self) -> &seats::Rides {
        &self.rides
    }

    /// Sees from `view`; false without a player.
    pub fn set_player_view(&mut self, view: View) -> bool {
        let Some(player) = self.players.first_mut() else {
            return false;
        };
        player.set_view(view);
        self.place_player_camera();
        true
    }

    /// The local player's camera while it is dead (`dead_camera_update`):
    /// from where it saw as it died, eased out to look at its body as the
    /// body falls; gone once it lives again.
    fn place_dead_camera(&mut self) {
        let Some(player) = self.players.first() else {
            self.dead_camera = None;
            return;
        };
        let handle = player.handle();
        let dead = self.is_dead(handle);
        let body = self.component::<Transform>(handle);
        let (true, Some(body)) = (dead, body) else {
            self.dead_camera = None;
            return;
        };
        let mut camera = self.camera();
        let dead_camera = match &mut self.dead_camera {
            Some(dead_camera) => {
                dead_camera.step();
                *dead_camera
            }
            None => *self.dead_camera.insert(dead_camera::DeadCamera::new(
                &camera.transform,
                &mut self.local_random,
            )),
        };
        let nodes = self.component::<NodeMatrices>(handle);
        let root = nodes.as_ref().and_then(|nodes| nodes.current.first());
        let focus = player.bounding_centre(root, &body);
        camera.transform = dead_camera.place(focus, |focus, back| player.camera_reach(focus, back));
        self.world.insert_resource(camera);
    }

    /// The camera at the player's view now, as if it had always been there.
    fn place_player_camera(&mut self) {
        let Some(player) = self.players.first() else {
            return;
        };
        let pose = player.pose();
        let mut camera = self.camera();
        camera.transform = pose.camera;
        self.world.insert_resource(camera);
        let poses = self.players.iter().map(Player::pose).collect();
        self.world.insert_resource(PlayerPoses(poses));
        let drives = self
            .players
            .iter()
            .map(|player| player.unit_drive(&self.rides))
            .collect();
        self.world
            .insert_resource(unit_animator::UnitDrives(drives));
    }

    /// Runs every tick of a recording.
    pub fn replay(&mut self, record: &InputRecord) {
        for &input in &record.ticks {
            self.step(input);
        }
    }

    /// The ticks run so far.
    pub fn tick(&self) -> Tick {
        self.world
            .get_resource::<Tick>()
            .copied()
            .unwrap_or_default()
    }

    /// The camera.
    pub fn camera(&self) -> Camera {
        self.world
            .get_resource::<Camera>()
            .copied()
            .unwrap_or_default()
    }

    /// The camera turned `yaw` and `pitch` further, as the next tick will
    /// turn it: the latest mouse look, shown before the tick that takes
    /// it. Its position is the last tick's.
    pub fn camera_turned(&self, yaw: Radians, pitch: Radians) -> Camera {
        let mut camera = self.camera();
        let (turned_yaw, turned_pitch) = match self.players.first() {
            Some(player) => {
                let (from_yaw, from_pitch) = player.look();
                facing::turn(from_yaw.0, from_pitch.0, yaw.0, pitch.0)
            }
            None => {
                let free = self.world.get_resource::<FreeCamera>().copied();
                let (from_yaw, from_pitch) =
                    free.map_or((Radians(0.0), Radians(0.0)), |free| (free.yaw, free.pitch));
                let max = FreeCamera::MAX_PITCH.0;
                (
                    libm::remainderf(from_yaw.0 + yaw.0, std::f32::consts::TAU),
                    (from_pitch.0 + pitch.0).clamp(-max, max),
                )
            }
        };
        let turned = FreeCamera {
            yaw: Radians(turned_yaw),
            pitch: Radians(turned_pitch),
            ..FreeCamera::default()
        };
        if turned.yaw.0.is_finite() && turned.pitch.0.is_finite() {
            camera.transform.rotation = turned.rotation();
        }
        camera
    }

    /// Puts the free camera at `free` (a level's start), its pitch kept
    /// within the limit, and the camera with it.
    pub fn place_free_camera(&mut self, free: FreeCamera) {
        let max = FreeCamera::MAX_PITCH.0;
        let free = FreeCamera {
            pitch: Radians(free.pitch.0.clamp(-max, max)),
            ..free
        };
        self.world.insert_resource(free);
        let mut camera = self.camera();
        camera.transform.position = free.position;
        camera.transform.rotation = free.rotation();
        self.world.insert_resource(camera);
    }

    /// Makes an object.
    pub fn spawn(&mut self, spawn: ObjectSpawn) -> Result<ObjectHandle, TableFull> {
        objects::spawn(&mut self.world, spawn)
    }

    /// The objects made since this was last called, oldest first: the
    /// app gives each its model and animation.
    pub fn take_spawned(&mut self) -> Vec<ObjectHandle> {
        self.world
            .get_resource_mut::<objects::Spawned>()
            .map(|mut spawned| std::mem::take(&mut spawned.0))
            .unwrap_or_default()
    }

    /// Makes every object the game places as `level` starts in its
    /// structure BSP `structure_bsp` ([`objects::level_placements`]).
    pub fn spawn_scenario(
        &mut self,
        level: &Level,
        structure_bsp: usize,
    ) -> Result<Vec<ObjectHandle>, TableFull> {
        objects::spawn_scenario(&mut self.world, level, structure_bsp)
    }

    /// Moves the objects to `level`'s structure BSP `structure_bsp`, as the
    /// game does as it switches to it ([`objects::switch_structure_bsp`]):
    /// the objects made, each with its placement's index.
    pub fn place_structure_bsp(
        &mut self,
        level: &Level,
        structure_bsp: usize,
    ) -> Result<Vec<(usize, ObjectHandle)>, TableFull> {
        objects::switch_structure_bsp(&mut self.world, level, structure_bsp)
    }

    /// Plays `player` on an object, posing its nodes now; false if `handle`
    /// is not in the world.
    pub fn animate(&mut self, handle: ObjectHandle, player: AnimationPlayer) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let Ok(mut object) = self.world.get_entity_mut(entity) else {
            return false;
        };
        let transform = object.get::<Transform>().copied().unwrap_or_default();
        let current = player.matrices(&transform);
        // (an animation set by hand plays as it is, without the state machine)
        object.remove::<unit_animator::UnitAnimator>();
        object.insert((
            player,
            NodeMatrices {
                previous: current.clone(),
                current,
            },
        ));
        true
    }

    /// Plays `player` on a biped as [`Simulation::animate`] does, and has
    /// its state machine choose its base animation from then on (standing,
    /// moving, in the air, landing); false if `handle` is not in the world.
    pub fn animate_unit(&mut self, handle: ObjectHandle, player: AnimationPlayer) -> bool {
        if !self.animate(handle, player) {
            return false;
        }
        let entity = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle));
        if let Some(mut object) = entity.and_then(|entity| self.world.get_entity_mut(entity).ok()) {
            object.insert(unit_animator::UnitAnimator::default());
        }
        true
    }

    /// Poses a device by its position and power, and sets what it exports
    /// into its functions; false if `handle` is not in the world.
    pub fn pose_device(
        &mut self,
        handle: ObjectHandle,
        pose: device_animation::DevicePose,
    ) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let Ok(mut object) = self.world.get_entity_mut(entity) else {
            return false;
        };
        object.insert(pose);
        true
    }

    /// Starts the AI's `impulse` on a unit standing on its own feet, blending
    /// into it, if it may play one now and its graph has it for the
    /// weapon it holds (`unit_start_animation_impulse`); the impulse's
    /// frames, after which the unit goes back to what it did.
    pub fn start_animation_impulse(
        &mut self,
        handle: ObjectHandle,
        impulse: ai_impulse::AnimationImpulse,
    ) -> Option<u16> {
        let entity = self.world.get_resource::<ObjectTable>()?.entity(handle)?;
        let dead = self
            .world
            .get::<damage::Vitality>(entity)
            .is_some_and(|vitality| vitality.dead);
        let player = self.world.get::<AnimationPlayer>(entity)?;
        let state = if dead {
            unit_states::UnitState::Dying
        } else if player.resumes() {
            unit_states::UnitState::AiImpulse
        } else {
            unit_states::UnitState::Idle
        };
        if !ai_impulse::can_play(state, None, impulse) {
            return None;
        }
        let graph = std::sync::Arc::clone(&player.graph);
        let first = self.impulse_animation(handle, impulse)?;
        let random = self
            .world
            .get_resource_mut::<random::RandomSeed>()?
            .next_real();
        let animation =
            animation::choose_permutation(&graph.animations, Some(first), random).unwrap_or(first);
        let frames = graph.animations.get(usize::from(animation))?.frame_count;
        self.world.get_mut::<AnimationPlayer>(entity)?.play(
            animation,
            impulse.interpolation_frames(),
            true,
        );
        Some(frames)
    }

    /// Whether unit `handle`'s graph has an animation for `impulse` in the
    /// class it now plays.
    pub fn has_animation_impulse(
        &self,
        handle: ObjectHandle,
        impulse: ai_impulse::AnimationImpulse,
    ) -> bool {
        self.impulse_animation(handle, impulse).is_some()
    }

    /// The first animation of `impulse` in the class unit `handle` plays.
    fn impulse_animation(
        &self,
        handle: ObjectHandle,
        impulse: ai_impulse::AnimationImpulse,
    ) -> Option<u16> {
        let entity = self.world.get_resource::<ObjectTable>()?.entity(handle)?;
        let player = self.world.get::<AnimationPlayer>(entity)?;
        let class = ai_impulse::class_of(&player.graph, player.state.animation)?;
        ai_impulse::impulse_animation(&player.graph, class, impulse)
    }

    /// What each animated object's animation did in the last tick that is
    /// heard: the sounds it reached and the feet it put down, by object.
    pub fn animation_events(&mut self) -> Vec<animation::AnimationEvent> {
        let mut events: Vec<_> = self
            .queries
            .events
            .get(&mut self.world)
            .iter(&self.world)
            .filter(|(.., player)| player.sound().is_some() || player.foot().is_some())
            .map(
                |(handle, object, transform, player)| animation::AnimationEvent {
                    object: *handle,
                    tag: object.tag.clone(),
                    position: transform.position,
                    sound: player.sound().cloned(),
                    foot: player.foot(),
                    step: MaterialEffects::RUN,
                },
            )
            .collect();
        // (a player's feet walk crouched, and make the jump and its landing
        // with both at once, as `biped_make_footstep` makes them: a landing
        // only one fast enough to land, as `biped_update_landing` runs, and
        // as it lands rather than two ticks into its recovery)
        for player in &self.players {
            let handle = player.handle();
            if player.crouching() {
                for event in events.iter_mut().filter(|event| event.object == handle) {
                    event.step = MaterialEffects::WALK;
                }
            }
            let step = if player.jumped() {
                MaterialEffects::JUMP
            } else if player.landed().is_some() && player.mover().landing.is_some() {
                MaterialEffects::JUMP_LAND
            } else {
                continue;
            };
            let (Some(tag), Some(transform)) = (
                self.entity(handle)
                    .and_then(|entity| entity.get::<objects::Object>())
                    .map(|object| object.tag.clone()),
                self.transform(handle),
            ) else {
                continue;
            };
            for foot in [animation::Foot::Left, animation::Foot::Right] {
                events.push(animation::AnimationEvent {
                    object: handle,
                    tag: tag.clone(),
                    position: transform.position,
                    sound: None,
                    foot: Some(foot),
                    step,
                });
            }
        }
        events.sort_by_key(|event| event.object);
        events
    }

    /// Each object as the renderer draws it, in the order of their ids.
    pub fn visit_drawn(&mut self, mut visit: impl FnMut(Drawn<'_>)) {
        let mut objects: Vec<_> = self
            .queries
            .drawn
            .get(&mut self.world)
            .iter(&self.world)
            .collect();
        objects.sort_by_key(|(id, ..)| **id);
        // (the player's own body is not seen from its eyes)
        let hidden = self
            .players
            .first()
            .filter(|player| player.view() == View::FirstPerson)
            .map(|player| ObjectId(player.handle().index));
        for (id, transform, previous, nodes, appearance) in objects {
            if Some(*id) == hidden {
                continue;
            }
            visit(Drawn {
                id: *id,
                previous: previous.map_or(transform, |p| &p.0),
                current: transform,
                nodes,
                appearance,
            });
        }
    }

    /// Every object, in the order of their ids, with what its lights need:
    /// the player's own body too, whose lights (its flashlight) light the
    /// view though the body is not drawn.
    pub fn visit_lit(&mut self, mut visit: impl FnMut(Lit<'_>)) {
        let mut objects: Vec<_> = self
            .queries
            .lit
            .get(&mut self.world)
            .iter(&self.world)
            .collect();
        objects.sort_by_key(|(id, ..)| **id);
        let viewer = self
            .players
            .first()
            .filter(|player| player.view() == View::FirstPerson)
            .map(|player| ObjectId(player.handle().index));
        for (id, object, transform, previous, nodes, functions, appearance) in objects {
            visit(Lit {
                id: *id,
                tag: &object.tag,
                previous: previous.map_or(transform, |p| &p.0),
                current: transform,
                nodes,
                functions,
                appearance,
                viewer: Some(*id) == viewer,
            });
        }
    }

    /// Sets how an object is drawn; false if `handle` is not in the world.
    pub fn set_appearance(
        &mut self,
        handle: ObjectHandle,
        appearance: objects::Appearance,
    ) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let Ok(mut object) = self.world.get_entity_mut(entity) else {
            return false;
        };
        object.insert(appearance);
        true
    }

    /// Gives an object a component, or replaces the one it has (a script's
    /// teleport, its scale); false if `handle` is not in the world.
    pub fn insert<C: Component>(&mut self, handle: ObjectHandle, component: C) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let Ok(mut object) = self.world.get_entity_mut(entity) else {
            return false;
        };
        object.insert(component);
        true
    }

    /// Puts an object at `transform` at once, as if it had always been
    /// there (a script's teleport: the renderer does not blend it across
    /// from where it was); false if `handle` is not in the world.
    pub fn place(&mut self, handle: ObjectHandle, transform: Transform) -> bool {
        self.insert(handle, transform) && self.insert(handle, PreviousTransform(transform))
    }

    /// Removes an object; false if `handle` is not in the world.
    pub fn remove(&mut self, handle: ObjectHandle) -> bool {
        objects::remove(&mut self.world, handle)
    }

    /// The hooks each kind's objects are made with.
    pub fn kinds_mut(&mut self) -> Mut<'_, ObjectKinds> {
        self.world.resource_mut::<ObjectKinds>()
    }

    /// Every object, by slot.
    pub fn objects(&mut self) -> Vec<ObjectHandle> {
        let mut handles: Vec<ObjectHandle> = self
            .queries
            .handles
            .get(&mut self.world)
            .iter(&self.world)
            .copied()
            .collect();
        handles.sort();
        handles
    }

    /// The entity `handle` is, if it is still in the world.
    fn entity(&self, handle: ObjectHandle) -> Option<EntityRef<'_>> {
        let entity = self.world.get_resource::<ObjectTable>()?.entity(handle)?;
        self.world.get_entity(entity).ok()
    }

    /// One of an object's components, if `handle` is still in the world.
    pub fn component<C: Component + Clone>(&self, handle: ObjectHandle) -> Option<C> {
        self.entity(handle)?.get::<C>().cloned()
    }

    /// What `read` makes of one of an object's components, borrowed rather
    /// than cloned, if `handle` is still in the world and has it.
    pub fn read_component<C: Component, R>(
        &self,
        handle: ObjectHandle,
        read: impl FnOnce(&C) -> R,
    ) -> Option<R> {
        self.entity(handle)?.get::<C>().map(read)
    }

    /// Whether `handle` is in the world and is dead.
    fn is_dead(&self, handle: ObjectHandle) -> bool {
        self.read_component(handle, |vitality: &damage::Vitality| vitality.dead)
            .unwrap_or(false)
    }

    /// Kills an object outright, as the scripts' `unit_kill` does: a
    /// vehicle Halo gives no vitality (each of the stock game's) is
    /// destroyed only this way. Whether `handle` is in the world.
    pub fn kill(&mut self, handle: ObjectHandle) -> bool {
        let Some(entity) = self
            .world
            .get_resource::<ObjectTable>()
            .and_then(|table| table.entity(handle))
        else {
            return false;
        };
        let vitality = match self.world.get::<damage::Vitality>(entity) {
            Some(vitality) => damage::Vitality {
                body: 0.0,
                shield: 0.0,
                dead: true,
                ..vitality.clone()
            },
            None => damage::Vitality {
                body: 0.0,
                shield: 0.0,
                stun_ticks: 0,
                shield_depleted: false,
                dead: true,
                region_damage: Vec::new(),
                regions_destroyed: Vec::new(),
            },
        };
        self.world.entity_mut(entity).insert(vitality);
        true
    }

    /// What an object is (borrowed: a frame asks this of many objects, and
    /// a copy would copy its tag's path).
    pub fn object(&self, handle: ObjectHandle) -> Option<&Object> {
        self.entity(handle)?.get::<Object>()
    }

    /// Where an object is.
    pub fn transform(&self, handle: ObjectHandle) -> Option<Transform> {
        self.component(handle)
    }

    /// A hash of the world's state: the tick, the camera, and every
    /// object in the order of their ids: its transform, kind, tag, name,
    /// permutation, team and animation, the random seed, and the player's
    /// movement and look.
    pub fn world_hash(&mut self) -> u64 {
        let mut hasher = WorldHasher::default();
        hasher.u64(self.tick().0);
        self.camera().transform.hash(&mut hasher);
        hasher.u64(u64::from(
            self.world
                .get_resource::<random::RandomSeed>()
                .map_or(0, |seed| seed.0),
        ));
        let mut objects: Vec<_> = self
            .world
            .query::<(
                &ObjectId,
                &Transform,
                &Object,
                Option<&ObjectName>,
                Option<&Permutation>,
                Option<&Team>,
                Option<&AnimationPlayer>,
                Option<&armory::WeaponItem>,
                Option<&damage::Vitality>,
            )>()
            .iter(&self.world)
            .collect();
        objects.sort_by_key(|(id, ..)| **id);
        for (id, transform, object, name, permutation, team, player, item, vitality) in objects {
            hasher.u64(u64::from(id.0));
            transform.hash(&mut hasher);
            hasher.u64(kind_index(object.kind) as u64);
            hasher.bytes(&object.tag.group.0);
            hasher.u64(object.tag.path.len() as u64);
            hasher.bytes(object.tag.path.as_bytes());
            // (each absent part hashed as such, so two never run together)
            match name {
                Some(name) => {
                    hasher.u64(1 + name.0.len() as u64);
                    hasher.bytes(name.0.as_bytes());
                }
                None => hasher.u64(0),
            }
            match permutation {
                Some(Permutation(permutation)) => {
                    hasher.u64(1);
                    for color in permutation.change_colors {
                        hasher.u64(u64::from(color));
                    }
                    hasher.bytes(&permutation.region_permutations);
                }
                None => hasher.u64(0),
            }
            hasher.u64(team.map_or(u64::MAX, |team| u64::from(team.0)));
            match player {
                Some(player) => {
                    hasher.u64(1 + u64::from(player.state.animation));
                    hasher.u64(u64::from(player.state.frame));
                }
                None => hasher.u64(0),
            }
            // (only weapons a player has touched have rounds of their own,
            // and nothing is hashed for the others, as before there were)
            if let Some(item) = item {
                hasher.u64(1);
                item.0.hash(&mut hasher);
            }
            if let Some(vitality) = vitality {
                hasher.u64(2);
                vitality.hash(&mut hasher);
            }
        }
        // (nothing for no player, so that a world without one hashes as
        // it did before there were players)
        if !self.players.is_empty() {
            for player in &self.players {
                player.hash(&mut hasher);
            }
            self.rides.hash(&mut hasher);
        }
        hasher.finish()
    }
}

/// A player's biped, standing at `transform`, on `team`.
fn biped(
    world: &mut World,
    tag: TagRef,
    transform: Transform,
    team: Option<u16>,
) -> Option<ObjectHandle> {
    objects::spawn(
        world,
        ObjectSpawn {
            kind: ObjectKind::Biped,
            tag,
            transform,
            name: None,
            permutation: None,
            team,
        },
    )
    .ok()
}

/// Where an object was at the tick before: the renderer interpolates from
/// it to its [`Transform`].
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PreviousTransform(pub Transform);

fn remember_transforms(mut objects: Query<(&Transform, &mut PreviousTransform)>) {
    for (transform, mut previous) in &mut objects {
        previous.0 = *transform;
    }
}

/// The objects put somewhere this tick rather than moved there (a
/// teleport), which the renderer must not blend from where they were.
#[derive(Resource, Clone, Debug, Default)]
struct Snaps(Vec<ObjectHandle>);

fn snap_transforms(
    mut snaps: ResMut<Snaps>,
    table: Res<ObjectTable>,
    mut objects: Query<(&Transform, &mut PreviousTransform)>,
) {
    for handle in snaps.0.drain(..) {
        if let Some((transform, mut previous)) = table
            .entity(handle)
            .and_then(|entity| objects.get_mut(entity).ok())
        {
            previous.0 = *transform;
        }
    }
}

fn look_free_camera(input: Res<CurrentInput>, mut camera: ResMut<FreeCamera>) {
    let max = FreeCamera::MAX_PITCH.0;
    camera.yaw = Radians(libm::remainderf(
        camera.yaw.0 + input.0.yaw.0,
        std::f32::consts::TAU,
    ));
    camera.pitch = Radians((camera.pitch.0 + input.0.pitch.0).clamp(-max, max));
}

fn move_free_camera(input: Res<CurrentInput>, mut camera: ResMut<FreeCamera>) {
    let input = input.0;
    let axis = |value: i8| f32::from(value) / 127.0;
    let speed = if input.buttons.contains(Buttons::SPRINT) {
        FreeCamera::SPRINT_SPEED
    } else {
        FreeCamera::SPEED
    };
    let rotation = camera.rotation();
    let direction = rotation * Vec3::X * axis(input.forward)
        - rotation * Vec3::Y * axis(input.right)
        + Vec3::Z * axis(input.up);
    // (no faster on a diagonal)
    let direction = if direction.length_squared() > 1.0 {
        direction.normalize()
    } else {
        direction
    };
    camera.position += direction * (speed.0 * clock::TICK.0);
}

fn place_camera(
    free: Res<FreeCamera>,
    players: Option<Res<PlayerPoses>>,
    mut camera: ResMut<Camera>,
) {
    if let Some(player) = players.as_ref().and_then(|players| players.0.first()) {
        camera.transform = player.camera;
    } else {
        camera.transform.position = free.position;
        camera.transform.rotation = free.rotation();
    }
}

fn advance_tick(mut tick: ResMut<Tick>) {
    tick.0 += 1;
}

/// One tick of `player`'s unit's flashlight: switched when `pressed`.
fn step_flashlight(world: &mut World, player: &Player, pressed: bool) {
    let Some(entity) = world
        .get_resource::<ObjectTable>()
        .and_then(|table| table.entity(player.handle()))
    else {
        return;
    };
    let Ok(mut unit) = world.get_entity_mut(entity) else {
        return;
    };
    let dead = unit
        .get::<damage::Vitality>()
        .is_some_and(|vitality| vitality.dead);
    let mut light = unit
        .get::<functions::IntegratedLight>()
        .copied()
        .unwrap_or_default();
    light.step(pressed, player.seated().is_some(), dead);
    unit.insert(light);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inputs that turn, move and sprint, from a fixed pseudo-random
    /// sequence.
    fn recorded_inputs(ticks: usize) -> InputRecord {
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let ticks = (0..ticks)
            .map(|_| {
                let r = next();
                TickInput {
                    buttons: if r & 1 == 0 {
                        Buttons::SPRINT
                    } else {
                        Buttons::default()
                    },
                    forward: (r >> 8) as i8 / 2,
                    right: (r >> 16) as i8 / 2,
                    up: (r >> 24) as i8 / 4,
                    yaw: Radians(((r >> 32) as u16 as f32 / 65535.0 - 0.5) * 0.1),
                    pitch: Radians(((r >> 48) as u16 as f32 / 65535.0 - 0.5) * 0.05),
                }
            })
            .collect();
        InputRecord { ticks }
    }

    fn replayed_hash(record: &InputRecord) -> u64 {
        let mut simulation = Simulation::empty();
        for at in [3, 1, 2] {
            let spawned = simulation.spawn(objects::ObjectSpawn {
                kind: opence_content::ObjectKind::Scenery,
                tag: opence_content::TagRef {
                    group: opence_content::Group(*b"scen"),
                    path: format!("scenery\\rock{at}"),
                },
                transform: Transform {
                    position: Vec3::splat(at as f32),
                    ..Default::default()
                },
                name: None,
                permutation: None,
                team: Some(at),
            });
            assert!(spawned.is_ok());
        }
        simulation.replay(record);
        simulation.world_hash()
    }

    #[test]
    fn an_empty_world_counts_its_ticks() {
        let mut simulation = Simulation::empty();
        assert_eq!(simulation.tick(), Tick(0));
        simulation.step(TickInput::default());
        simulation.step(TickInput::default());
        assert_eq!(simulation.tick(), Tick(2));
    }

    #[test]
    fn a_replay_gives_the_same_hash_on_every_run() {
        let record = recorded_inputs(3600);
        let first = replayed_hash(&record);
        for _ in 0..3 {
            assert_eq!(replayed_hash(&record), first);
        }
    }

    #[test]
    fn a_replay_read_from_its_file_gives_the_same_hash() {
        let record = recorded_inputs(600);
        let read = InputRecord::from_bytes(&record.to_bytes()).unwrap_or_default();
        assert_eq!(replayed_hash(&read), replayed_hash(&record));
    }

    #[test]
    fn the_replays_hash_is_pinned() {
        // A change to the simulation's maths or its order changes this;
        // when that is meant, write the new value here. A change nobody
        // meant (another platform's maths, a fused multiply-add, systems
        // out of order) is what this catches.
        assert_eq!(replayed_hash(&recorded_inputs(3600)), PINNED_HASH);
    }
    const PINNED_HASH: u64 = 0x0101_d70d_de3d_1b7f;

    #[test]
    fn one_different_input_changes_the_hash() {
        let record = recorded_inputs(600);
        let mut changed = record.clone();
        changed.ticks[300].yaw = Radians(changed.ticks[300].yaw.0 + 0.001);
        assert_ne!(replayed_hash(&changed), replayed_hash(&record));
    }

    #[test]
    fn the_free_camera_moves_at_its_speed() {
        let mut simulation = Simulation::empty();
        for _ in 0..60 {
            simulation.step(TickInput {
                forward: 127,
                ..Default::default()
            });
        }
        let position = simulation.camera().transform.position;
        assert!(
            (position.x - FreeCamera::SPEED.0).abs() < 1e-4,
            "{position}"
        );
        assert!(position.y.abs() < 1e-6 && position.z.abs() < 1e-6);
    }

    #[test]
    fn the_free_camera_turns_and_stops_short_of_straight_up() {
        let mut simulation = Simulation::empty();
        for _ in 0..100 {
            simulation.step(TickInput {
                yaw: Radians(0.1),
                pitch: Radians(0.1),
                ..Default::default()
            });
        }
        let forward = simulation.camera().transform.rotation * Vec3::X;
        assert!(forward.z > 0.999 && forward.z < 1.0, "{forward}");
    }

    fn rock(scale: f32) -> objects::ObjectSpawn {
        objects::ObjectSpawn {
            kind: opence_content::ObjectKind::Scenery,
            tag: opence_content::TagRef {
                group: opence_content::Group(*b"scen"),
                path: "scenery\\rock".into(),
            },
            transform: Transform {
                scale,
                ..Default::default()
            },
            name: None,
            permutation: None,
            team: None,
        }
    }

    #[test]
    fn a_spawned_or_placed_object_is_drawn_from_where_it_is_put(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut simulation = Simulation::empty();
        let mut spawn = rock(1.0);
        spawn.transform.position = Vec3::new(4.0, 5.0, 6.0);
        let handle = simulation.spawn(spawn)?;
        let drawn = |simulation: &Simulation| {
            simulation
                .transform(handle)
                .zip(simulation.component::<PreviousTransform>(handle))
                .map(|(now, PreviousTransform(then))| (then.position, now.position))
        };
        // (spawned: never blended in from the origin)
        let at = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(drawn(&simulation), Some((at, at)));
        // (a script's teleport, between ticks and after one)
        let far = Transform {
            position: Vec3::new(-90.0, 30.0, 2.0),
            ..Transform::default()
        };
        assert!(simulation.place(handle, far));
        assert_eq!(drawn(&simulation), Some((far.position, far.position)));
        simulation.step(TickInput::default());
        assert_eq!(drawn(&simulation), Some((far.position, far.position)));
        // (a moved one is still blended: only what is put is snapped)
        assert!(simulation.insert(
            handle,
            Transform {
                position: at,
                ..far
            }
        ));
        assert_eq!(drawn(&simulation), Some((far.position, at)));
        Ok(())
    }

    #[test]
    fn the_hash_takes_objects_by_id_not_by_spawn_order() -> Result<(), Box<dyn std::error::Error>> {
        // (a's slot 0 is taken again after its first object goes, by an
        // entity made after slot 1's)
        let mut a = Simulation::empty();
        let first = a.spawn(rock(1.0))?;
        a.spawn(rock(2.0))?;
        assert!(a.remove(first));
        a.spawn(rock(3.0))?;
        let mut b = Simulation::empty();
        b.spawn(rock(3.0))?;
        b.spawn(rock(2.0))?;
        assert_eq!(a.world_hash(), b.world_hash());
        let mut c = Simulation::empty();
        c.spawn(rock(2.0))?;
        c.spawn(rock(3.0))?;
        assert_ne!(a.world_hash(), c.world_hash());
        Ok(())
    }
}
