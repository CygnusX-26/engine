// pub mod cone;
// pub mod cube;
// pub mod cylinder;
// pub mod letter_n;
pub mod p_hack;

use nalgebra::Point3;

#[derive(Debug)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub color: Color,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub trait Mesh {
    fn verts(&self) -> &[Point3<f32>];
    fn tris(&self) -> &Vec<Triangle>;
}
