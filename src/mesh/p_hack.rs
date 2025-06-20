use crate::mesh::Mesh;
use macroquad::color::{Color, PURPLE, YELLOW};
use nalgebra::Point3;

#[derive(Debug)]
pub struct PHackMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl PHackMesh {
    pub fn new() -> Self {
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
                (0, 4, 1, YELLOW),
                (4, 0, 2, YELLOW),
                (3, 6, 5, YELLOW),
                (6, 3, 7, YELLOW),
                (9, 2, 8, YELLOW),
                (2, 9, 10, YELLOW),
                (11, 12, 15, YELLOW),
                (15, 13, 11, YELLOW),
                (14, 16, 17, YELLOW),
                (17, 18, 14, YELLOW),
                (20, 19, 13, YELLOW),
                (13, 21, 20, YELLOW),
                (13, 2, 0, PURPLE),
                (0, 11, 13, PURPLE),
                (20, 9, 8, PURPLE),
                (8, 19, 20, PURPLE),
                (18, 7, 4, PURPLE),
                (4, 15, 18, PURPLE),
                (1, 3, 12, PURPLE),
                (14, 12, 3, PURPLE),
                (5, 6, 16, PURPLE),
                (17, 16, 6, PURPLE),
                (2, 10, 13, PURPLE),
                (21, 13, 10, PURPLE),
                (0, 1, 12, PURPLE),
                (12, 11, 0, PURPLE),
                (8, 2, 13, PURPLE),
                (13, 19, 8, PURPLE),
                (3, 5, 16, PURPLE),
                (16, 14, 3, PURPLE),
                (10, 9, 21, PURPLE),
                (20, 21, 9, PURPLE),
                (4, 2, 15, PURPLE),
                (13, 15, 2, PURPLE),
                (6, 7, 17, PURPLE),
                (18, 17, 7, PURPLE),
            ],
        }
    }
}

impl Mesh for PHackMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
