//! The game's seeded random numbers (`random_math.c`): a linear
//! congruential generator whose seed is part of the game's state, so that
//! every machine that steps the same game draws the same numbers. Its
//! random directions come from a table of a geosphere's vertices
//! (`geometry.c`'s `geosphere_new`, sixteen segments to an octahedron's
//! edge), built once, in the game's order.

use std::sync::OnceLock;

use bevy_ecs::prelude::*;
use glam::Vec3;

const RANDOM_A: u32 = 1_664_525;
const RANDOM_C: u32 = 1_013_904_223;

/// The seed a game starts with, unless its options give another.
pub const DEFAULT_SEED: u32 = 0xDEAD_BEEF;

/// A random seed: the game's (which the game's state draws from).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RandomSeed(pub u32);

impl Default for RandomSeed {
    fn default() -> Self {
        Self(DEFAULT_SEED)
    }
}

impl RandomSeed {
    /// The next number, from 0 to 65535 (`seed_random`).
    pub fn next_u16(&mut self) -> u16 {
        self.0 = self.0.wrapping_mul(RANDOM_A).wrapping_add(RANDOM_C);
        (self.0 >> 16) as u16
    }

    /// The next number, from 0 to 1 (`real_seed_random`).
    pub fn next_real(&mut self) -> f32 {
        f32::from(self.next_u16()) / 65535.0
    }

    /// The next number from `lower` up to but not including `upper`
    /// (`seed_random_range`).
    pub fn next_range(&mut self, lower: i16, upper: i16) -> i16 {
        let span = (i32::from(upper) - i32::from(lower)) as u32;
        let offset = span.wrapping_mul(u32::from(self.next_u16())) >> 16;
        (i32::from(lower) + offset as i32) as i16
    }

    /// The next number from `lower` to `upper` (`real_seed_random_range`).
    pub fn next_real_range(&mut self, lower: f32, upper: f32) -> f32 {
        let random = self.next_real();
        lower + (upper - lower) * random
    }

    /// One of the table's directions at random (`seed_random_direction3d`).
    pub fn next_direction(&mut self) -> Vec3 {
        let table = direction_table();
        let size = i16::try_from(table.len()).unwrap_or(i16::MAX);
        let index = usize::try_from(self.next_range(0, size)).unwrap_or(0);
        Vec3::from_array(table.get(index).copied().unwrap_or([0.0, 0.0, 1.0]))
    }

    /// `axis` turned by an angle from `inner` to `outer` radians at random,
    /// about the way across it to one of the table's directions at random;
    /// `axis` itself when that direction is along it
    /// (`seed_random_vector_in_cone3d`).
    pub fn vector_in_cone(&mut self, axis: Vec3, inner: f32, outer: f32) -> Vec3 {
        let random = self.next_direction();
        let (a, b) = (axis, random);
        let mut across = Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        );
        if normalize(&mut across) > 0.0 {
            let angle = self.next_real_range(inner, outer);
            let sine = libm::sin(f64::from(angle)) as f32;
            let cosine = libm::cos(f64::from(angle)) as f32;
            rotate_about_axis(axis, across, sine, cosine)
        } else {
            axis
        }
    }
}

/// The geosphere's segments to an octahedron's edge
/// (`RANDOM_DIRECTION_TABLE_GEOSPHERE_SEGMENT_COUNT`).
const SEGMENTS: usize = 16;

/// The octahedron the geosphere subdivides: its vertices and faces.
const OCTAHEDRON_VERTICES: [[f32; 3]; 6] = [
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, 0.0, -1.0],
];
const OCTAHEDRON_FACES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [0, 2, 3],
    [0, 3, 4],
    [0, 4, 1],
    [5, 1, 4],
    [5, 4, 3],
    [5, 3, 2],
    [5, 2, 1],
];

/// The random directions: the geosphere's vertices, in the game's order.
pub fn direction_table() -> &'static [[f32; 3]] {
    static TABLE: OnceLock<Vec<[f32; 3]>> = OnceLock::new();
    TABLE.get_or_init(|| Geosphere::new(SEGMENTS).vertices)
}

/// `v` made a unit vector, its length returned; left alone, and 0
/// returned, when it is shorter than the game's epsilon (`normalize3d`,
/// its square root in double).
pub(crate) fn normalize(v: &mut Vec3) -> f32 {
    let magnitude = libm::sqrt(f64::from(v.x * v.x + v.y * v.y + v.z * v.z)) as f32;
    // a NaN length is scaled by, as the game's `!(epsilon > length)` does
    if magnitude.abs() < 0.0001 {
        return 0.0;
    }
    let scale = 1.0 / magnitude;
    *v = Vec3::new(scale * v.x, scale * v.y, scale * v.z);
    magnitude
}

/// `v` turned about the unit `n` by the angle of `sine` and `cosine`
/// (`rotate_vector_about_axis`).
pub(crate) fn rotate_about_axis(v: Vec3, n: Vec3, sine: f32, cosine: f32) -> Vec3 {
    let along = (n.x * v.x + n.y * v.y + n.z * v.z) * (1.0 - cosine);
    let cross = Vec3::new(
        n.z * v.y - v.z * n.y,
        n.x * v.z - v.x * n.z,
        n.y * v.x - v.y * n.x,
    );
    Vec3::new(
        cosine * v.x + along * n.x - sine * cross.x,
        cosine * v.y + along * n.y - sine * cross.y,
        cosine * v.z + along * n.z - sine * cross.z,
    )
}

/// An octahedron's faces subdivided into `segments` rows and pushed out
/// onto the unit sphere, its vertices numbered as `geosphere_new` numbers
/// them: the octahedron's, then each edge's as a face first reaches it,
/// then each face's own.
struct Geosphere {
    segments: usize,
    vertices: Vec<[f32; 3]>,
    next: usize,
    /// The first vertex inside each edge, by its lower and higher vertex.
    edges: [[Option<usize>; 8]; 8],
}

impl Geosphere {
    fn new(segments: usize) -> Self {
        let count = 8 * (segments - 2) * (segments - 1) / 2 + 12 * (segments - 1) + 6;
        let mut sphere = Self {
            segments,
            vertices: vec![[0.0; 3]; count],
            next: 0,
            edges: [[None; 8]; 8],
        };
        for vertex in OCTAHEDRON_VERTICES {
            sphere.vertices[sphere.next] = vertex;
            sphere.next += 1;
        }
        for [v1, v2, v3] in OCTAHEDRON_FACES {
            sphere.subdivide(v1, v2, v3);
        }
        sphere
    }

    /// A face's vertices, in the order its triangle strips reach them
    /// (`subdivide_triangle`).
    fn subdivide(&mut self, v1: usize, v2: usize, v3: usize) {
        let mut face = vec![None; (self.segments + 1) * (self.segments + 1)];
        for row in 1..=self.segments {
            for column in 1..=row {
                self.face_vertex(&mut face, [v1, v2, v3], row - 1, column - 1);
                self.face_vertex(&mut face, [v1, v2, v3], row, column - 1);
                self.face_vertex(&mut face, [v1, v2, v3], row, column);
                if column < row {
                    self.face_vertex(&mut face, [v1, v2, v3], row - 1, column);
                }
            }
        }
    }

    /// The vertex at a face's row and column (`get_face_vertex`).
    fn face_vertex(
        &mut self,
        face: &mut [Option<usize>],
        [v1, v2, v3]: [usize; 3],
        row: usize,
        column: usize,
    ) -> usize {
        let index = row * (self.segments + 1) + column;
        if let Some(vertex) = face[index] {
            return vertex;
        }
        let vertex = if column == 0 {
            self.edge_vertex(v1, v3, row)
        } else if row == self.segments {
            self.edge_vertex(v3, v2, column)
        } else if column == row {
            self.edge_vertex(v1, v2, row)
        } else {
            let new = self.next;
            self.next += 1;
            let first = self.edge_vertex(v1, v3, row);
            let second = self.edge_vertex(v1, v2, row);
            self.calculate(new, first, second, row, column);
            new
        };
        face[index] = Some(vertex);
        vertex
    }

    /// The vertex `subdivision` of the way along the edge from `v1` to `v2`,
    /// the whole edge's made when it is first reached (`get_edge_vertex`).
    fn edge_vertex(&mut self, v1: usize, v2: usize, subdivision: usize) -> usize {
        if subdivision == 0 {
            return v1;
        }
        if subdivision == self.segments {
            return v2;
        }
        let (low, high) = (v1.min(v2), v1.max(v2));
        let first = match self.edges[low][high] {
            Some(first) => first,
            None => {
                let first = self.next;
                self.edges[low][high] = Some(first);
                for step in 1..self.segments {
                    let new = self.next;
                    self.next += 1;
                    self.calculate(new, low, high, self.segments, step);
                }
                first
            }
        };
        let offset = if v1 > v2 {
            self.segments - subdivision
        } else {
            subdivision
        };
        first + offset - 1
    }

    /// The vertex `index` of `count` of the way from one parent to the
    /// other, on the sphere (`calculate_vertex`).
    fn calculate(&mut self, new: usize, first: usize, second: usize, count: usize, index: usize) {
        let t = index as f32 / count as f32;
        let one_minus_t = 1.0 - t;
        let (a, b) = (self.vertices[first], self.vertices[second]);
        let mut vertex = Vec3::new(
            one_minus_t * a[0] + t * b[0],
            one_minus_t * a[1] + t * b[1],
            one_minus_t * a[2] + t * b[2],
        );
        normalize(&mut vertex);
        self.vertices[new] = vertex.to_array();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_numbers_are_the_games() {
        let mut seed = RandomSeed::default();
        // 0xDEADBEEF * 1664525 + 1013904223, kept to 32 bits
        let first = 0xDEAD_BEEFu32
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        assert_eq!(seed.next_u16(), (first >> 16) as u16);
        assert_eq!(seed.0, first);
        let mut again = RandomSeed::default();
        assert_eq!(again.next_real(), f32::from((first >> 16) as u16) / 65535.0);
        for _ in 0..1000 {
            let value = seed.next_range(3, 9);
            assert!((3..9).contains(&value));
            assert!((0.0..=1.0).contains(&seed.next_real()));
        }
    }

    #[test]
    fn the_directions_are_a_geospheres() {
        let table = direction_table();
        assert_eq!(table.len(), 1026);
        assert_eq!(table[0], [0.0, 0.0, 1.0]);
        for direction in table {
            let length = Vec3::from_array(*direction).length();
            assert!((length - 1.0).abs() < 1e-5, "{direction:?}");
        }
    }

    #[test]
    fn a_cone_of_no_angle_is_its_axis() {
        let mut seed = RandomSeed::default();
        for _ in 0..100 {
            let turned = seed.vector_in_cone(Vec3::X, 0.0, 0.0);
            assert!((turned - Vec3::X).length() < 1e-6, "{turned}");
        }
    }
}
