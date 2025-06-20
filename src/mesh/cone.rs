use macroquad::color::{Color, RED, GREEN, YELLOW, BLUE, PURPLE, ORANGE, BEIGE, LIME, PINK, GOLD, GRAY, DARKBLUE, SKYBLUE};
use nalgebra::{Point3};
use crate::mesh::Mesh;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct ConeMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl ConeMesh {
    pub fn new(height: f32, radius: f32) -> Self {

        let offset = -height / 2.0;

        let mut verts: Vec<Point3<f32>> = (0..12).map(
            |x| -> Point3<f32> {
                let angle = 2.0 * PI * (x as f32) / 12.0;
                let x = radius * angle.cos();
                let z = radius * angle.sin();
                Point3::new(x, offset, z)
            }
        ).collect();

        verts.push(Point3::new(0.0, height + offset, 0.0));
        verts.push(Point3::new(0.0, offset, 0.0));

        Self {
            verts,
            tris: vec![
                (0, 1, 12, RED),
                (1, 2, 12, ORANGE),
                (2, 3, 12, YELLOW),
                (3, 4, 12, GREEN),
                (4, 5, 12, BLUE),
                (5, 6, 12, PURPLE),
                (6, 7, 12, PINK),
                (7, 8, 12, LIME),
                (8, 9, 12, GOLD),
                (9, 10, 12, BEIGE),
                (10, 11, 12, GRAY),
                (11, 0, 12, DARKBLUE),
                (0, 13, 1, SKYBLUE),
                (1, 13, 2, RED),
                (2, 13, 3, SKYBLUE),
                (3, 13, 4, RED),
                (4, 13, 5, SKYBLUE),
                (5, 13, 6, RED),
                (6, 13, 7, SKYBLUE),
                (7, 13, 8, RED),
                (8, 13, 9, SKYBLUE),
                (9, 13, 10, RED),
                (10, 13, 11, SKYBLUE),
                (11, 13, 0, RED)
            ]
        }
    }
}

impl Mesh for ConeMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &[Point3<f32>] {
        &self.verts
    }
}