use crate::mesh::{
    Mesh, Triangle, BEIGE, BLUE, DARKBLUE, GOLD, GRAY, GREEN, LIME, ORANGE, PINK, PURPLE, RED,
    SKYBLUE, YELLOW,
};
use nalgebra::Point3;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct ConeMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl ConeMesh {
    pub fn new(height: f32, radius: f32) -> Self {
        let offset = -height / 2.0;

        let mut verts: Vec<Point3<f32>> = (0..12)
            .map(|x| -> Point3<f32> {
                let angle = 2.0 * PI * (x as f32) / 12.0;
                let x = radius * angle.cos();
                let z = radius * angle.sin();
                Point3::new(x, offset, z)
            })
            .collect();

        verts.push(Point3::new(0.0, height + offset, 0.0));
        verts.push(Point3::new(0.0, offset, 0.0));

        Self {
            verts,
            tris: vec![
                Triangle {
                    v1: 0,
                    v2: 1,
                    v3: 12,
                    color: RED,
                },
                Triangle {
                    v1: 1,
                    v2: 2,
                    v3: 12,
                    color: ORANGE,
                },
                Triangle {
                    v1: 2,
                    v2: 3,
                    v3: 12,
                    color: YELLOW,
                },
                Triangle {
                    v1: 3,
                    v2: 4,
                    v3: 12,
                    color: GREEN,
                },
                Triangle {
                    v1: 4,
                    v2: 5,
                    v3: 12,
                    color: BLUE,
                },
                Triangle {
                    v1: 5,
                    v2: 6,
                    v3: 12,
                    color: PURPLE,
                },
                Triangle {
                    v1: 6,
                    v2: 7,
                    v3: 12,
                    color: PINK,
                },
                Triangle {
                    v1: 7,
                    v2: 8,
                    v3: 12,
                    color: LIME,
                },
                Triangle {
                    v1: 8,
                    v2: 9,
                    v3: 12,
                    color: GOLD,
                },
                Triangle {
                    v1: 9,
                    v2: 10,
                    v3: 12,
                    color: BEIGE,
                },
                Triangle {
                    v1: 10,
                    v2: 11,
                    v3: 12,
                    color: GRAY,
                },
                Triangle {
                    v1: 11,
                    v2: 0,
                    v3: 12,
                    color: DARKBLUE,
                },
                Triangle {
                    v1: 0,
                    v2: 13,
                    v3: 1,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 1,
                    v2: 13,
                    v3: 2,
                    color: RED,
                },
                Triangle {
                    v1: 2,
                    v2: 13,
                    v3: 3,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 3,
                    v2: 13,
                    v3: 4,
                    color: RED,
                },
                Triangle {
                    v1: 4,
                    v2: 13,
                    v3: 5,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 5,
                    v2: 13,
                    v3: 6,
                    color: RED,
                },
                Triangle {
                    v1: 6,
                    v2: 13,
                    v3: 7,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 7,
                    v2: 13,
                    v3: 8,
                    color: RED,
                },
                Triangle {
                    v1: 8,
                    v2: 13,
                    v3: 9,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 9,
                    v2: 13,
                    v3: 10,
                    color: RED,
                },
                Triangle {
                    v1: 10,
                    v2: 13,
                    v3: 11,
                    color: SKYBLUE,
                },
                Triangle {
                    v1: 11,
                    v2: 13,
                    v3: 0,
                    color: RED,
                },
            ],
        }
    }
}

impl Mesh for ConeMesh {
    fn tris(&self) -> &[Triangle] {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
