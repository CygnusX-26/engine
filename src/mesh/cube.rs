use crate::mesh::{Mesh, Triangle};
use nalgebra::Point3;

#[derive(Debug)]
pub struct CubeMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl CubeMesh {
    pub fn _new() -> Self {
        Self {
            verts: vec![
                Point3::new(1.0, -1.0, 1.0),
                Point3::new(1.0, -1.0, -1.0),
                Point3::new(-1.0, -1.0, -1.0),
                Point3::new(-1.0, -1.0, 1.0),
                Point3::new(1.0, 1.0, 1.0),
                Point3::new(1.0, 1.0, -1.0),
                Point3::new(-1.0, 1.0, -1.0),
                Point3::new(-1.0, 1.0, 1.0),
            ],
            tris: vec![
                Triangle {
                    v1: 0,
                    v2: 1,
                    v3: 2,
                    color: RED,
                },
                Triangle {
                    v1: 0,
                    v2: 2,
                    v3: 3,
                    color: GREEN,
                },
                Triangle {
                    v1: 4,
                    v2: 6,
                    v3: 5,
                    color: BLUE,
                },
                Triangle {
                    v1: 4,
                    v2: 7,
                    v3: 6,
                    color: ORANGE,
                },
                Triangle {
                    v1: 0,
                    v2: 3,
                    v3: 7,
                    color: PINK,
                },
                Triangle {
                    v1: 0,
                    v2: 7,
                    v3: 4,
                    color: DARKBLUE,
                },
                Triangle {
                    v1: 1,
                    v2: 5,
                    v3: 6,
                    color: LIME,
                },
                Triangle {
                    v1: 1,
                    v2: 6,
                    v3: 2,
                    color: PURPLE,
                },
                Triangle {
                    v1: 0,
                    v2: 4,
                    v3: 5,
                    color: BEIGE,
                },
                Triangle {
                    v1: 0,
                    v2: 5,
                    v3: 1,
                    color: GOLD,
                },
                Triangle {
                    v1: 3,
                    v2: 2,
                    v3: 6,
                    color: GRAY,
                },
                Triangle {
                    v1: 3,
                    v2: 6,
                    v3: 7,
                    color: YELLOW,
                },
            ],
        }
    }
}

impl Mesh for CubeMesh {
    fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
