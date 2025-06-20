use crate::mesh::Mesh;
use macroquad::color::{
    BEIGE, BLUE, Color, DARKBLUE, GOLD, GRAY, GREEN, LIME, ORANGE, PINK, PURPLE, RED, YELLOW,
};
use nalgebra::Point3;

#[derive(Debug)]
pub struct TallWallMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl TallWallMesh {
    pub fn new() -> Self {
        Self {
            verts: vec![
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
                Point3::new(-0.5, 2.0, 0.0),
                Point3::new(0.5, 2.0, 0.0),
            ],
            tris: vec![(0, 1, 2, BLUE), (3, 2, 1, BLUE)],
        }
    }
}

impl Mesh for TallWallMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}
