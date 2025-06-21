use crate::mesh::{Mesh, Triangle};
use macroquad::color::{BLUE, GOLD, GREEN, PINK, PURPLE, RED};
use nalgebra::Point3;

#[derive(Debug)]
pub struct LetterNMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl LetterNMesh {
    pub fn _new() -> Self {
        Self {
            verts: vec![
                Point3::new(-2.0, -2.0, -1.0),
                Point3::new(-2.0, 2.0, -1.0),
                Point3::new(-1.0, -2.0, -1.0),
                Point3::new(-1.0, 2.0, -1.0),
                Point3::new(-1.0, 1.0, -1.0),
                Point3::new(2.0, 2.0, -1.0),
                Point3::new(2.0, -2.0, -1.0),
                Point3::new(1.0, 2.0, -1.0),
                Point3::new(1.0, -2.0, -1.0),
                Point3::new(1.0, -1.0, -1.0),
                Point3::new(-2.0, -2.0, 0.0),
                Point3::new(-2.0, 2.0, 0.0),
                Point3::new(-1.0, -2.0, 0.0),
                Point3::new(-1.0, 2.0, 0.0),
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(2.0, 2.0, 0.0),
                Point3::new(2.0, -2.0, 0.0),
                Point3::new(1.0, 2.0, 0.0),
                Point3::new(1.0, -2.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
            ],
            tris: vec![
                Triangle {
                    v1: 0,
                    v2: 2,
                    v3: 1,
                    color: RED,
                },
                Triangle {
                    v1: 1,
                    v2: 2,
                    v3: 3,
                    color: RED,
                },
                Triangle {
                    v1: 8,
                    v2: 6,
                    v3: 7,
                    color: RED,
                },
                Triangle {
                    v1: 7,
                    v2: 6,
                    v3: 5,
                    color: RED,
                },
                Triangle {
                    v1: 8,
                    v2: 9,
                    v3: 4,
                    color: RED,
                },
                Triangle {
                    v1: 3,
                    v2: 4,
                    v3: 9,
                    color: RED,
                },
                Triangle {
                    v1: 10,
                    v2: 11,
                    v3: 12,
                    color: BLUE,
                },
                Triangle {
                    v1: 11,
                    v2: 13,
                    v3: 12,
                    color: BLUE,
                },
                Triangle {
                    v1: 18,
                    v2: 17,
                    v3: 16,
                    color: BLUE,
                },
                Triangle {
                    v1: 15,
                    v2: 16,
                    v3: 17,
                    color: BLUE,
                },
                Triangle {
                    v1: 18,
                    v2: 14,
                    v3: 19,
                    color: BLUE,
                },
                Triangle {
                    v1: 13,
                    v2: 19,
                    v3: 14,
                    color: BLUE,
                },
                Triangle {
                    v1: 0,
                    v2: 11,
                    v3: 10,
                    color: GREEN,
                },
                Triangle {
                    v1: 11,
                    v2: 0,
                    v3: 1,
                    color: GREEN,
                },
                Triangle {
                    v1: 6,
                    v2: 15,
                    v3: 5,
                    color: GREEN,
                },
                Triangle {
                    v1: 15,
                    v2: 6,
                    v3: 16,
                    color: GREEN,
                },
                Triangle {
                    v1: 4,
                    v2: 2,
                    v3: 14,
                    color: PURPLE,
                },
                Triangle {
                    v1: 2,
                    v2: 12,
                    v3: 14,
                    color: PURPLE,
                },
                Triangle {
                    v1: 7,
                    v2: 17,
                    v3: 9,
                    color: PURPLE,
                },
                Triangle {
                    v1: 9,
                    v2: 17,
                    v3: 19,
                    color: PURPLE,
                },
                Triangle {
                    v1: 4,
                    v2: 14,
                    v3: 8,
                    color: PINK,
                },
                Triangle {
                    v1: 8,
                    v2: 14,
                    v3: 18,
                    color: PINK,
                },
                Triangle {
                    v1: 3,
                    v2: 9,
                    v3: 13,
                    color: PINK,
                },
                Triangle {
                    v1: 19,
                    v2: 13,
                    v3: 9,
                    color: PINK,
                },
                Triangle {
                    v1: 1,
                    v2: 3,
                    v3: 13,
                    color: GOLD,
                },
                Triangle {
                    v1: 1,
                    v2: 13,
                    v3: 11,
                    color: GOLD,
                },
                Triangle {
                    v1: 7,
                    v2: 5,
                    v3: 15,
                    color: GOLD,
                },
                Triangle {
                    v1: 7,
                    v2: 15,
                    v3: 17,
                    color: GOLD,
                },
                Triangle {
                    v1: 0,
                    v2: 12,
                    v3: 2,
                    color: GOLD,
                },
                Triangle {
                    v1: 0,
                    v2: 10,
                    v3: 12,
                    color: GOLD,
                },
                Triangle {
                    v1: 8,
                    v2: 16,
                    v3: 6,
                    color: GOLD,
                },
                Triangle {
                    v1: 8,
                    v2: 18,
                    v3: 16,
                    color: GOLD,
                },
            ],
        }
    }
}

impl Mesh for LetterNMesh {
    fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
