//! The HUD's bitmap lookups are the game's:
//! tools/engine_hud_sprite_reference.py runs bitmap_group.c's
//! `bitmap_group_get_bitmap_from_sequence` and hud_draw.c's
//! `hud_retrieve_bitmap_and_bounding_rect` and `get_sprite_clip_rect` over
//! three bitmaps, at each sequence from below the first to past the last
//! and frames to past the last, and the engine's lookups are asked the same
//! and compared.

use opence_content::{
    Bitmap, BitmapImage, Group, Pixels, ResourceRange, Sequence, Sprite as TagSprite, TagRef,
};

use super::{sprite, Frame, Sprite, WHOLE};

const REFERENCE: &str = include_str!("../../tests/hud_sprite_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

fn integer(fields: &[&str], at: usize) -> Result<i32, Failure> {
    Ok(fields.get(at).ok_or("a short line")?.parse()?)
}

fn rectangle(fields: &[&str]) -> Result<[f32; 4], Failure> {
    let values = fields
        .iter()
        .map(|field| field.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()?;
    match values[..] {
        [left, right, top, bottom] => Ok([left, right, top, bottom]),
        _ => Err("not a rectangle".into()),
    }
}

fn image() -> BitmapImage {
    BitmapImage {
        width: 16,
        height: 16,
        depth: 1,
        kind: 0,
        format: 0,
        flags: 0,
        registration_point: [0, 0],
        mipmap_count: 0,
        pixels: Pixels::ResourceMap(ResourceRange { offset: 0, size: 0 }),
    }
}

/// The trace's bitmaps, as the engine imports them.
fn bitmaps() -> Result<Vec<Bitmap>, Failure> {
    let mut bitmaps = lines("group")
        .map(|fields| {
            Ok(Bitmap {
                tag: TagRef {
                    group: Group(*b"bitm"),
                    path: String::new(),
                },
                kind: 4,
                format: 0,
                usage: 0,
                flags: 0,
                bump_height: 0.0,
                sequences: Vec::new(),
                images: vec![image(); usize::try_from(integer(&fields, 1)?)?],
            })
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    for fields in lines("sequence") {
        let bitmap = bitmaps
            .get_mut(usize::try_from(integer(&fields, 0)?)?)
            .ok_or("no such group")?;
        bitmap.sequences.push(Sequence {
            name: String::new(),
            first_image: u16::try_from(integer(&fields, 2)?).ok(),
            image_count: u16::try_from(integer(&fields, 3)?)?,
            sprites: Vec::new(),
        });
    }
    for fields in lines("sprite") {
        let sequence = bitmaps
            .get_mut(usize::try_from(integer(&fields, 0)?)?)
            .and_then(|bitmap| {
                usize::try_from(integer(&fields, 1).ok()?)
                    .ok()
                    .and_then(|index| bitmap.sequences.get_mut(index))
            })
            .ok_or("no such sequence")?;
        sequence.sprites.push(TagSprite {
            image: u16::try_from(integer(&fields, 3)?)?,
            bounds: rectangle(fields.get(4..8).ok_or("a short line")?)?,
            registration_point: [0.0, 0.0],
        });
    }
    Ok(bitmaps)
}

/// The trace's answer: an image and its part, or nothing drawn.
fn expected(fields: &[&str]) -> Result<Option<Sprite>, Failure> {
    let Ok(image) = u16::try_from(integer(fields, 3)?) else {
        return Ok(None);
    };
    let clip = match fields.get(4..) {
        Some(["none"]) => WHOLE,
        Some(rest) => rectangle(rest)?,
        None => return Err("a short line".into()),
    };
    Ok(Some(Sprite { image, clip }))
}

fn compare(kind: &str, bitmaps: &[Bitmap]) -> Result<usize, Failure> {
    let mut compared = 0;
    for fields in lines(kind) {
        let bitmap = bitmaps
            .get(usize::try_from(integer(&fields, 0)?)?)
            .ok_or("no such group")?;
        let sequence = i16::try_from(integer(&fields, 1)?)?;
        let frame = usize::try_from(integer(&fields, 2)?)?;
        let asked = if kind == "static" {
            // a static element asks for its sequence's first frame: the
            // trace's other frames ask the same lookup, as the engine's
            // takes them
            if frame != 0 {
                compared += usize::from(compare_frame(bitmap, sequence, frame, &fields)?);
                continue;
            }
            Frame::Element(sequence)
        } else {
            Frame::Sprite(sequence, frame)
        };
        assert_eq!(
            sprite(bitmap, asked),
            expected(&fields)?,
            "{kind} {fields:?}"
        );
        compared += 1;
    }
    Ok(compared)
}

/// A static lookup at a frame past the first, through the lookups a static
/// element is drawn by.
fn compare_frame(
    bitmap: &Bitmap,
    sequence: i16,
    frame: usize,
    fields: &[&str],
) -> Result<bool, Failure> {
    let actual = super::sequence_image(bitmap, sequence, frame).map(|image| Sprite {
        image,
        clip: super::sprite_clip(bitmap, sequence, frame).unwrap_or(WHOLE),
    });
    assert_eq!(actual, expected(fields)?, "static {fields:?}");
    Ok(true)
}

#[test]
fn a_static_elements_bitmap_is_the_games() -> Result<(), Failure> {
    assert_eq!(compare("static", &bitmaps()?)?, 340);
    Ok(())
}

#[test]
fn a_numbers_and_crosshairs_bitmap_is_the_games() -> Result<(), Failure> {
    assert_eq!(compare("hud", &bitmaps()?)?, 3 * 8 * 20);
    Ok(())
}
