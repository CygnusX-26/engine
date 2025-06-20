use crate::mesh::Mesh;
use macroquad::color::{BLUE, Color, GOLD, GREEN, PINK, PURPLE, RED};
use nalgebra::Point3;

#[derive(Debug)]
pub struct LetterNMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
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
                (0, 2, 1, RED),
                (1, 2, 3, RED),
                (8, 6, 7, RED),
                (7, 6, 5, RED),
                (8, 9, 4, RED),
                (3, 4, 9, RED),
                (10, 11, 12, BLUE),
                (11, 13, 12, BLUE),
                (18, 17, 16, BLUE),
                (15, 16, 17, BLUE),
                (18, 14, 19, BLUE),
                (13, 19, 14, BLUE),
                (0, 11, 10, GREEN),
                (11, 0, 1, GREEN),
                (6, 15, 5, GREEN),
                (15, 6, 16, GREEN),
                (4, 2, 14, PURPLE),
                (2, 12, 14, PURPLE),
                (7, 17, 9, PURPLE),
                (9, 17, 19, PURPLE),
                (4, 14, 8, PINK),
                (8, 14, 18, PINK),
                (3, 9, 13, PINK),
                (19, 13, 9, PINK),
                (1, 3, 13, GOLD),
                (1, 13, 11, GOLD),
                (7, 5, 15, GOLD),
                (7, 15, 17, GOLD),
                (0, 12, 2, GOLD),
                (0, 10, 12, GOLD),
                (8, 16, 6, GOLD),
                (8, 18, 16, GOLD), // TODO later
            ],
        }
    }
}

impl Mesh for LetterNMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
