use crate::mesh::{Mesh, Triangle, BLUE, GREEN, ORANGE, RED};
use nalgebra::Point3;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct CylinderMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl CylinderMesh {
    pub fn _new(height: f32, radius: f32) -> Self {
        let offset = -height / 2.0;

        let mut verts: Vec<Point3<f32>> = Vec::new();

        for i in 0..12 {
            let angle = 2.0 * PI * (i as f32) / 12.0;
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            verts.push(Point3::new(x, offset, z));
        }

        for i in 0..12 {
            let angle = 2.0 * PI * (i as f32) / 12.0;
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            verts.push(Point3::new(x, height + offset, z));
        }

        verts.push(Point3::new(0.0, offset, 0.0));
        verts.push(Point3::new(0.0, height + offset, 0.0));

        let mut tris: Vec<Triangle> = Vec::new();
        for i in 0..12 {
            let next = (i + 1) % 12;

            tris.push(Triangle {
                v1: i,
                v2: next,
                v3: i + 12,
                color: RED,
            });

            tris.push(Triangle {
                v1: i + 12,
                v2: next,
                v3: next + 12,
                color: ORANGE,
            });

            tris.push(Triangle {
                v1: next,
                v2: i,
                v3: 24,
                color: BLUE,
            });

            tris.push(Triangle {
                v1: i + 12,
                v2: next + 12,
                v3: 25,
                color: GREEN,
            });
        }

        Self { verts, tris }
    }
}

impl Mesh for CylinderMesh {
    fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
