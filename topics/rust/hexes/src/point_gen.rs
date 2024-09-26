use cgmath::{Vector3,Point3,InnerSpace};
use std::collections::HashMap;
use std::iter::repeat;

// Precompute some constants
///
/// X multiplication factor.
/// 1.0 / sqrt(2)
///
const X_MULT_FACTOR: f32 = std::f32::consts::FRAC_1_SQRT_2;

///
/// Y multiplication factor.
/// sqrt(3) / sqrt(2) == sqrt(1.5)
///
const Y_MULT_FACTOR: f32 = SQRT_3 * X_MULT_FACTOR;

const SIN_45: f32 = std::f32::consts::FRAC_1_SQRT_2;
const COS_45: f32 = SIN_45;
const SQRT_3: f32 = 1.73205080757;

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct TerrainVertexAttributes {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [u8; 4],
}

#[derive(Copy,Clone,Debug)]
pub struct TerrainVertex {
    pub position: Point3<f32>,
    pub colour: [u8; 4],
}

pub fn middle(p1: &TerrainVertex, p2: &TerrainVertex, p3: &TerrainVertex) -> Point3<f32> {
    Point3 {
        x: (p1.position.x + p2.position.x + p3.position.x) / 3.0,
        y: (p1.position.y + p2.position.y + p3.position.y) / 3.0,
        z: (p1.position.z + p2.position.z + p3.position.z) / 3.0
    }
}

pub fn halfway(p1: &TerrainVertex, p2: &TerrainVertex) -> Point3<f32> {
    Point3 {
        x: (p1.position.x + p2.position.x) / 2.0,
        y: (p1.position.y + p2.position.y) / 2.0,
        z: (p1.position.z + p2.position.z) / 2.0
    }
}

/// Calculate normal of the plane defined by (counter-clockwise?) points a-b-c
pub fn calculate_normal(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Vector3<f32> {
    (b - a).normalize().cross((c - a).normalize()).normalize()
}

/// Given a hexagon at (x, y), return the (x, y) coords for each hexagon that shares an edge with this one.
///
/// +---0---1
/// | / |   |
/// 5---p---2
/// |   | / |
/// 4---3---+
fn surrounding_hexagonal_points(x: isize, y: isize) -> [(isize, isize); 6] {
    [
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
    ]
}

pub fn surrounding_point_values_iter<T>(
    point_map: &HashMap<(isize, isize), T>,
    x: isize,
    y: isize,
    for_each: impl FnMut((&T, &T)),
) {
    let points = surrounding_hexagonal_points(x, y);
    let points = [
        points[0],
        points[1],
        points[2],
        points[3],
        points[4],
        points[5],
        points[0], // TODO: Why the extra?
    ];

    points
        .windows(2)
        .map(|x| (point_map.get(&x[0]), point_map.get(&x[1])))
        .flat_map(|(p1, p2)| p1.and_then(|x| p2.map(|y| (x, y))))
        .for_each(for_each); // TODO: Can we return the iterator? Or unsized -> not practical?
}

/// Given
fn square_width_given_radius(radius: f32) -> usize {
    ((((((4.0 * radius) / SQRT_3) + 1.0).floor() / 2.0).floor() * 2.0) + 1.0) as usize
}

#[derive(Clone)]
pub struct HexTerrainMesh {
    pub vertices: HashMap<(isize, isize), TerrainVertex>,
    half_size: isize,
}

impl HexTerrainMesh {
    pub fn generate(radius: f32, mut gen_vertex: impl FnMut([f32; 2]) -> TerrainVertex) -> Self {
        let width = square_width_given_radius(radius);
        let half_width = (width / 2) as isize;
        let mut map = HashMap::new();
        let mut max = std::f32::NEG_INFINITY;
        for i in -half_width..=half_width {
            let x_o = i as f32;
            for j in -half_width..=half_width {
                let y_o = j as f32;
                let x = X_MULT_FACTOR * (x_o * SIN_45 - y_o * COS_45);
                let y = Y_MULT_FACTOR * (x_o * SIN_45 + y_o * COS_45);
                if x.hypot(y) < radius {
                    let vertex = gen_vertex([x, y]);
                    if vertex.position.y > max {
                        max = vertex.position.y;
                    }
                    map.insert((i, j), vertex);
                }
            }
        }

        Self {
            vertices: map,
            half_size: half_width,
        }
    }

    pub fn make_buffer_data(&self) -> Vec<TerrainVertexAttributes> {
        let mut vertices = Vec::new();

        let mut push_triangle = |p1: &TerrainVertex, 
                                 p2: &TerrainVertex, 
                                 p3: &TerrainVertex, 
                                 colour: [u8; 4]| {
            let mid_point = middle(p1, p2, p3);
            let half_p1_to_p3 = halfway(p1, p3);
            let half_p2_to_p3 = halfway(p2, p3);
            let p = p3.position;

            let plane1_normal = calculate_normal(half_p1_to_p3, mid_point, p);
            let plane2_normal = calculate_normal(mid_point, half_p2_to_p3, p);

            vertices.extend(
                [half_p1_to_p3, mid_point, p, // Plane 1
                 mid_point, half_p2_to_p3, p] // Plane 2
                    .iter()
                    .zip(
                        // TODO: I think needs a `take()` to make plane1 not be used 
                        // for plane2 normals, as `repeat` is infinite.
                        repeat::<[f32; 3]>(plane1_normal.into())
                            .chain(repeat::<[f32; 3]>(plane2_normal.into())))
                    .zip(repeat(colour))
                    .map(|((pos, normal), colour)| TerrainVertexAttributes {
                        position: *pos.as_ref(),
                        normal,
                        colour,
                    })
            )
        };

        for i in -self.half_size..=self.half_size {
            for j in -self.half_size..=self.half_size {
                if let Some(p) = self.vertices.get(&(i, j)) {
                    surrounding_point_values_iter(&self.vertices, i, j, |(a, b)| {
                        push_triangle(a, b, p, p.colour)
                    });
                }
            }
        }

        vertices
    }
}
