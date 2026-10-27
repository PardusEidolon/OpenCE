//! The engine's questions of its units, bipeds and vehicles against the
//! game's: which unit aims for a rider, whether any unit is dangerous,
//! whether a biped is unsuspecting, the markers of a unit's head and body,
//! the frames left of its animation, its equipment deleted, an animation's
//! foot frames, a biped's sight position and its limp body stopped, the
//! vehicles' dot product and whether a vehicle moves near a player, as
//! tools/engine_unit_questions_reference.py records the game's code
//! answering them.

use std::error::Error;

use glam::Vec3;
use opence_sim::unit_definition::seat_label;
use opence_sim::unit_life::UnitEquipment;
use opence_sim::unit_questions::{
    aiming_unit, animation_frames_remaining, foot_frame_index, is_dangerous,
    stop_limp_body_physics, unsuspecting, vehicle_dot_product, vehicle_moving_near_any_player,
    BipedSight, Estimate, Suspicion, BODY_MARKER, HEAD_MARKER, UNIT_NEVER_UNSUSPECTING,
};
use opence_sim::unit_states::UnitState;

const REFERENCE: &str = include_str!("unit_questions_reference.txt");

type Failure = Box<dyn Error>;

/// A real from its bits in hex.
fn real(field: &str) -> Result<f32, Failure> {
    Ok(f32::from_bits(u32::from_str_radix(field, 16)?))
}

/// A real as the trace writes it.
fn hex(value: f32) -> String {
    format!("{:x}", value.to_bits())
}

fn vector(fields: &[&str]) -> Result<Vec3, Failure> {
    match fields {
        [x, y, z, ..] => Ok(Vec3::new(real(x)?, real(y)?, real(z)?)),
        _ => Err("a vector missing".into()),
    }
}

fn point(value: Vec3) -> String {
    format!("{} {} {}", hex(value.x), hex(value.y), hex(value.z))
}

/// The game's index (or -1) as an index.
fn index(field: &str) -> Result<Option<usize>, Failure> {
    let value: i32 = field.parse()?;
    Ok(usize::try_from(value).ok())
}

/// An index as the game writes it, -1 for none.
fn game_index(value: Option<usize>) -> String {
    value.map_or_else(|| "-1".to_owned(), |value| value.to_string())
}

/// A seat's flags and label.
type Seat = (u32, String);

/// One object of the fake world.
struct Object {
    exists: bool,
    kind: i32,
    flags: u32,
    parent: Option<usize>,
    center: Vec3,
    velocity: Vec3,
    animation: usize,
    frame: i16,
    looking: Vec3,
    grenade_throw: i8,
    parent_seat: Option<usize>,
    state: usize,
    animation_flags: u16,
    base_seat: usize,
    biped_flags: u32,
    crouch: f32,
    definition_flags: u32,
    seats: Vec<Seat>,
    biped_definition_flags: u32,
    standing: f32,
    crouching: f32,
}

struct World {
    objects: Vec<Object>,
    /// Each animation's frame count and foot frames.
    animations: Vec<(i16, u8, u8)>,
}

fn field<'a>(fields: &[&'a str], at: usize) -> Result<&'a str, Failure> {
    fields
        .get(at)
        .copied()
        .ok_or_else(|| "a field missing".into())
}

fn object(text: &str) -> Result<Object, Failure> {
    let fields: Vec<&str> = text.split(' ').filter(|field| !field.is_empty()).collect();
    let at = |at| field(&fields, at);
    let seats = (25..29)
        .map(|at| {
            let (flags, label) = field(&fields, at)?.split_once(':').ok_or("no `:`")?;
            Ok((flags.parse()?, label.trim_matches('\'').to_owned()))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    Ok(Object {
        exists: at(0)? == "1",
        kind: at(1)?.parse()?,
        flags: at(2)?.parse()?,
        parent: index(at(3)?)?,
        center: vector(fields.get(4..).ok_or("no centre")?)?,
        velocity: vector(fields.get(7..).ok_or("no velocity")?)?,
        animation: at(10)?.parse()?,
        frame: at(11)?.parse()?,
        looking: vector(fields.get(13..).ok_or("no looking")?)?,
        grenade_throw: at(16)?.parse()?,
        parent_seat: index(at(17)?)?,
        state: at(19)?.parse()?,
        animation_flags: at(20)?.parse()?,
        base_seat: at(21)?.parse()?,
        biped_flags: at(22)?.parse()?,
        crouch: real(at(23)?)?,
        definition_flags: at(24)?.parse()?,
        seats,
        biped_definition_flags: at(29)?.parse()?,
        standing: real(at(30)?)?,
        crouching: real(at(31)?)?,
    })
}

fn world(line: &str) -> Result<World, Failure> {
    let (objects, animations) = line.split_once(" | animations ").ok_or("no animations")?;
    let objects = objects
        .split(" | ")
        .skip(1)
        .map(object)
        .collect::<Result<Vec<_>, Failure>>()?;
    let numbers: Vec<&str> = animations.split(' ').collect();
    let animations = numbers
        .chunks(3)
        .map(|animation| match animation {
            [count, left, right] => Ok((count.parse()?, left.parse()?, right.parse()?)),
            _ => Err("an animation cut short".into()),
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    Ok(World {
        objects,
        animations,
    })
}

impl World {
    fn object(&self, at: usize) -> Result<&Object, Failure> {
        self.objects.get(at).ok_or_else(|| "no such object".into())
    }

    /// The unit `unit` rides and its seat, if it rides one in a seat.
    fn riding(&self, unit: &Object) -> Result<Option<(usize, &Seat)>, Failure> {
        Ok(match (unit.parent, unit.parent_seat) {
            (Some(parent), Some(seat)) => Some((
                parent,
                self.object(parent)?.seats.get(seat).ok_or("no seat")?,
            )),
            _ => None,
        })
    }

    fn seat_label(&self, unit: &Object) -> Result<String, Failure> {
        let riding = self.riding(unit)?;
        let labels: Vec<&str> = riding
            .map(|(_, (_, label))| vec![label.as_str()])
            .unwrap_or_default();
        Ok(
            seat_label(riding.map(|_| (labels.as_slice(), 0)), unit.base_seat)
                .ok_or("no label")?
                .to_owned(),
        )
    }
}

fn case(world: &World, line: &str) -> Result<String, Failure> {
    let (asked, _) = line.rsplit_once(" -> ").ok_or("no `->`")?;
    let fields: Vec<&str> = asked.split(' ').collect();
    let unit = |at: usize| -> Result<Option<(usize, &Object)>, Failure> {
        index(field(&fields, at)?)?.map_or(Ok(None), |at| Ok(Some((at, world.object(at)?))))
    };
    Ok(match fields.as_slice() {
        ["aiming", ..] => game_index(match unit(1)? {
            Some((at, object)) => {
                let riding = world
                    .riding(object)?
                    .map(|(parent, (flags, _))| Ok::<_, Failure>((u32::try_from(parent)?, *flags)))
                    .transpose()?;
                Some(usize::try_from(aiming_unit(u32::try_from(at)?, riding))?)
            }
            None => None,
        }),
        ["dangerous"] => {
            let dangerous = world.objects.iter().any(|object| {
                object.exists
                    && object.kind <= 1
                    && is_dangerous(
                        UnitState::from_index(object.state),
                        object.grenade_throw,
                        object.animation_flags,
                    )
            });
            u8::from(dangerous).to_string()
        }
        ["unsuspecting", _, rest @ ..] => {
            let label;
            let biped = match unit(1)? {
                Some((_, object)) if object.kind == 0 => {
                    label = world.seat_label(object)?;
                    Some(Suspicion {
                        never_unsuspecting: object.definition_flags & UNIT_NEVER_UNSUSPECTING != 0,
                        center: object.center,
                        looking: object.looking,
                        seat_label: &label,
                    })
                }
                _ => None,
            };
            u8::from(unsuspecting(biped.as_ref(), vector(rest)?)).to_string()
        }
        ["head" | "body", which] => {
            let marker = if fields.first() == Some(&"head") {
                HEAD_MARKER
            } else {
                BODY_MARKER
            };
            // the fake world's marker: the unit's index, the name's length, a half
            let at: u16 = which.parse()?;
            let length = u16::try_from(marker.len())?;
            format!(
                "{marker} {}",
                point(Vec3::new(f32::from(at), f32::from(length), 0.5))
            )
        }
        ["frames_remaining", _] => {
            let (_, object) = unit(1)?.ok_or("no unit")?;
            let (count, _, _) = world
                .animations
                .get(object.animation)
                .ok_or("no animation")?;
            format!(
                "{} {}",
                animation_frames_remaining(*count, object.frame),
                object.state
            )
        }
        ["delete_equipment", _, equipment] => {
            let mut held = UnitEquipment(index(equipment)?.map(u32::try_from).transpose()?);
            let deleted = held.delete_current();
            let game = |value: Option<u32>| {
                value.map_or_else(|| "-1".to_owned(), |value| value.to_string())
            };
            format!("{} {}", game(deleted), game(held.current()))
        }
        ["sight", _, mode, rest @ ..] => {
            let (_, object) = unit(1)?.ok_or("no unit")?;
            let mode = Estimate::from_index(mode.parse()?).ok_or("no such estimate")?;
            let estimated = vector(rest)?;
            let facing = vector(rest.get(3..).ok_or("no facing")?)?;
            let offset = vector(rest.get(6..).ok_or("no offset")?)?;
            let sight = BipedSight {
                origin: object.center,
                crouch: object.crouch,
                standing_camera_height: object.standing,
                crouching_camera_height: object.crouching,
            };
            point(sight.position(mode, estimated, facing, offset))
        }
        ["stop_limp", _] => {
            let (_, object) = unit(1)?.ok_or("no unit")?;
            let (mut object_flags, mut biped_flags) = (object.flags, object.biped_flags);
            stop_limp_body_physics(
                object.biped_definition_flags,
                &mut biped_flags,
                &mut object_flags,
            );
            format!("{object_flags} {biped_flags}")
        }
        ["feet", animation] => {
            let at: usize = animation.parse()?;
            let (_, left, right) = world.animations.get(at).ok_or("no animation")?;
            format!("{} {}", foot_frame_index(*left), foot_frame_index(*right))
        }
        ["dot", rest @ ..] => hex(vehicle_dot_product(
            vector(rest)?,
            vector(rest.get(3..).ok_or("no b")?)?,
        )),
        ["moving_near", _, "|", players @ ..] => {
            let mut centers = Vec::new();
            for player in players.chunks(2) {
                if let [has, unit] = player {
                    if *has == "1" {
                        if let Some(unit) = index(unit)? {
                            let unit = world.object(unit)?;
                            if unit.parent.is_none() {
                                centers.push(unit.center);
                            }
                        }
                    }
                }
            }
            let vehicles = world
                .objects
                .iter()
                .filter(|object| object.exists && object.kind == 1)
                .map(|object| (object.center, object.velocity));
            u8::from(vehicle_moving_near_any_player(&centers, vehicles)).to_string()
        }
        _ => return Err(format!("an unknown case: {line}").into()),
    })
}

#[test]
fn the_unit_questions_are_the_games() -> Result<(), Failure> {
    let mut cases = 0;
    let mut current = None;
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        if line.starts_with("world") {
            current = Some(world(line)?);
            continue;
        }
        let world = current.as_ref().ok_or("a case before its world")?;
        let (_, expected) = line.rsplit_once(" -> ").ok_or("no `->`")?;
        assert_eq!(case(world, line)?, expected, "{line}");
        cases += 1;
    }
    assert!(cases > 9000, "only {cases} cases");
    Ok(())
}

#[test]
fn a_gunner_aims_for_the_unit_it_rides() {
    assert_eq!(aiming_unit(4, Some((2, 1 << 3))), 2);
    assert_eq!(aiming_unit(4, Some((2, 1 << 1))), 4);
    assert_eq!(aiming_unit(4, None), 4);
}
