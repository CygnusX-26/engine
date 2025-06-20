use crate::mesh::Mesh;
use macroquad::color::{
    BEIGE, BLUE, Color, DARKBLUE, GOLD, GRAY, GREEN, LIME, ORANGE, PINK, PURPLE, RED, YELLOW,
};
use nalgebra::Point3;

#[derive(Debug)]
pub struct CubeMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl CubeMesh {
    pub fn new() -> Self {
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
                (0, 1, 2, RED),
                (0, 2, 3, GREEN),
                (4, 6, 5, BLUE),
                (4, 7, 6, ORANGE),
                (0, 3, 7, PINK),
                (0, 7, 4, DARKBLUE),
                (1, 5, 6, LIME),
                (1, 6, 2, PURPLE),
                (0, 4, 5, BEIGE),
                (0, 5, 1, GOLD),
                (3, 2, 6, GRAY),
                (3, 6, 7, YELLOW),
            ],
        }
    }
}

impl Mesh for CubeMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
