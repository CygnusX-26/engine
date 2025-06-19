use macroquad::color::{Color, RED, GREEN, BLUE, PURPLE};
use nalgebra::{Point3};
use crate::mesh::Mesh;

#[derive(Debug)]
pub struct LetterNMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl LetterNMesh {
    pub fn new() -> Self {


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
                (0, 2, 1, RED),
                (2, 3, 1, RED),
                (8, 6, 7, RED),
                (6, 5, 7, RED),
                (8, 9, 4, RED),
                (4, 9, 3, RED),
                (10, 11, 12, BLUE),
                (12, 11, 13, BLUE),
                (18, 17, 16, BLUE),
                (16, 17, 15, BLUE),
                (18, 14, 19, BLUE),
                (14, 13, 19, BLUE),
                (0, 11, 10, GREEN),
                (0, 1, 11, GREEN),
                (6, 15, 5, GREEN),
                (6, 16, 15, GREEN),
                (2, 14, 4, PURPLE),
                (2, 12, 14, PURPLE)
                // TODO later
            ]
        }
    }
}

impl Mesh for LetterNMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &Vec<Point3<f32>> {
        &self.verts
    }
}