use crate::mesh::{Mesh, Triangle};
use macroquad::color::BLUE;
use nalgebra::Point3;

#[derive(Debug)]
pub struct WallMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<Triangle>,
}

impl WallMesh {
    pub fn new(layout: [[bool; 8]; 8]) -> Self {
        let verts: Vec<Point3<f32>> = vec![];
        for (i, line) in layout.iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                //TODO
            }
        }
        Self {
            verts: vec![
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
                Point3::new(-0.5, 2.0, 0.0),
                Point3::new(0.5, 2.0, 0.0),
            ],
            tris: vec![
                Triangle {
                    v1: 0,
                    v2: 1,
                    v3: 2,
                    color: BLUE,
                },
                Triangle {
                    v1: 3,
                    v2: 2,
                    v3: 1,
                    color: BLUE,
                },
            ],
        }
    }
}

impl Mesh for WallMesh {
    fn tris(&self) -> &Vec<Triangle> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
