
use macroquad::color::{Color, BLUE, GREEN, ORANGE, RED};
use nalgebra::{Point3};
use crate::mesh::Mesh;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct CylinderMesh {
    verts: Vec<Point3<f32>>,
    tris: Vec<(usize, usize, usize, Color)>,
}

impl CylinderMesh {
    pub fn new(height: f32, radius: f32) -> Self {
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

        let mut tris: Vec<(usize, usize, usize, Color)> = Vec::new();
        for i in 0..12 {
            let next = (i + 1) % 12;
            tris.push((i, next, i + 12, RED));
            tris.push((next + 12, i + 12, next, ORANGE));
            tris.push((next, i, 24, BLUE));
            tris.push((i + 12, next + 12, 25, GREEN));
        }
        Self {
            verts,
            tris
        }
    }
}

impl Mesh for CylinderMesh {
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)> {
        &self.tris
    }

    fn verts(&self) -> &Vec<Point3<f32>> {
        &self.verts
    }
}