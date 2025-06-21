use crate::mesh::{Color, Mesh, Triangle};
use nalgebra::Point3;

#[derive(Debug)]
pub struct PHackMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl PHackMesh {
    pub fn new() -> Self {
        const YELLOW: Color = Color {
            r: 255,
            g: 248,
            b: 54,
            a: 255,
        };
        const PURPLE: Color = Color {
            r: 171,
            g: 54,
            b: 255,
            a: 255,
        };
        Self {
            verts: vec![
                Point3::new(-0.5, 1.5, -0.5),
                Point3::new(0.5, 1.5, -0.5),
                Point3::new(-0.5, -0.5, -0.5),
                Point3::new(0.5, 0.5, -0.5),
                Point3::new(0.5, -0.5, -0.5),
                Point3::new(1.5, 0.5, -0.5),
                Point3::new(1.5, -1.5, -0.5),
                Point3::new(0.5, -1.5, -0.5),
                Point3::new(-1.5, -0.5, -0.5),
                Point3::new(-1.5, -1.5, -0.5),
                Point3::new(-0.5, -1.5, -0.5),
                Point3::new(-0.5, 1.5, 0.5),
                Point3::new(0.5, 1.5, 0.5),
                Point3::new(-0.5, -0.5, 0.5),
                Point3::new(0.5, 0.5, 0.5),
                Point3::new(0.5, -0.5, 0.5),
                Point3::new(1.5, 0.5, 0.5),
                Point3::new(1.5, -1.5, 0.5),
                Point3::new(0.5, -1.5, 0.5),
                Point3::new(-1.5, -0.5, 0.5),
                Point3::new(-1.5, -1.5, 0.5),
                Point3::new(-0.5, -1.5, 0.5),
            ],
            tris: vec![
                Triangle {
                    v1: 0,
                    v2: 4,
                    v3: 1,
                    color: YELLOW,
                },
                Triangle {
                    v1: 4,
                    v2: 0,
                    v3: 2,
                    color: YELLOW,
                },
                Triangle {
                    v1: 3,
                    v2: 6,
                    v3: 5,
                    color: YELLOW,
                },
                Triangle {
                    v1: 6,
                    v2: 3,
                    v3: 7,
                    color: YELLOW,
                },
                Triangle {
                    v1: 9,
                    v2: 2,
                    v3: 8,
                    color: YELLOW,
                },
                Triangle {
                    v1: 2,
                    v2: 9,
                    v3: 10,
                    color: YELLOW,
                },
                Triangle {
                    v1: 11,
                    v2: 12,
                    v3: 15,
                    color: YELLOW,
                },
                Triangle {
                    v1: 15,
                    v2: 13,
                    v3: 11,
                    color: YELLOW,
                },
                Triangle {
                    v1: 14,
                    v2: 16,
                    v3: 17,
                    color: YELLOW,
                },
                Triangle {
                    v1: 17,
                    v2: 18,
                    v3: 14,
                    color: YELLOW,
                },
                Triangle {
                    v1: 20,
                    v2: 19,
                    v3: 13,
                    color: YELLOW,
                },
                Triangle {
                    v1: 20,
                    v2: 13,
                    v3: 21,
                    color: YELLOW,
                },
                Triangle {
                    v1: 13,
                    v2: 2,
                    v3: 0,
                    color: PURPLE,
                },
                Triangle {
                    v1: 0,
                    v2: 11,
                    v3: 13,
                    color: PURPLE,
                },
                Triangle {
                    v1: 20,
                    v2: 9,
                    v3: 8,
                    color: PURPLE,
                },
                Triangle {
                    v1: 8,
                    v2: 19,
                    v3: 20,
                    color: PURPLE,
                },
                Triangle {
                    v1: 18,
                    v2: 7,
                    v3: 4,
                    color: PURPLE,
                },
                Triangle {
                    v1: 4,
                    v2: 15,
                    v3: 18,
                    color: PURPLE,
                },
                Triangle {
                    v1: 1,
                    v2: 3,
                    v3: 12,
                    color: PURPLE,
                },
                Triangle {
                    v1: 14,
                    v2: 12,
                    v3: 3,
                    color: PURPLE,
                },
                Triangle {
                    v1: 5,
                    v2: 6,
                    v3: 16,
                    color: PURPLE,
                },
                Triangle {
                    v1: 17,
                    v2: 16,
                    v3: 6,
                    color: PURPLE,
                },
                Triangle {
                    v1: 2,
                    v2: 10,
                    v3: 13,
                    color: PURPLE,
                },
                Triangle {
                    v1: 21,
                    v2: 13,
                    v3: 10,
                    color: PURPLE,
                },
                Triangle {
                    v1: 0,
                    v2: 1,
                    v3: 12,
                    color: PURPLE,
                },
                Triangle {
                    v1: 12,
                    v2: 11,
                    v3: 0,
                    color: PURPLE,
                },
                Triangle {
                    v1: 8,
                    v2: 2,
                    v3: 13,
                    color: PURPLE,
                },
                Triangle {
                    v1: 13,
                    v2: 19,
                    v3: 8,
                    color: PURPLE,
                },
                Triangle {
                    v1: 3,
                    v2: 5,
                    v3: 16,
                    color: PURPLE,
                },
                Triangle {
                    v1: 16,
                    v2: 14,
                    v3: 3,
                    color: PURPLE,
                },
                Triangle {
                    v1: 10,
                    v2: 9,
                    v3: 21,
                    color: PURPLE,
                },
                Triangle {
                    v1: 20,
                    v2: 21,
                    v3: 9,
                    color: PURPLE,
                },
                Triangle {
                    v1: 4,
                    v2: 2,
                    v3: 15,
                    color: PURPLE,
                },
                Triangle {
                    v1: 13,
                    v2: 15,
                    v3: 2,
                    color: PURPLE,
                },
                Triangle {
                    v1: 6,
                    v2: 7,
                    v3: 17,
                    color: PURPLE,
                },
                Triangle {
                    v1: 18,
                    v2: 17,
                    v3: 7,
                    color: PURPLE,
                },
            ],
        }
    }
}

impl Mesh for PHackMesh {
    fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
