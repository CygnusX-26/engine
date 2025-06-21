pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod letter_n;
pub mod p_hack;
pub mod wall;

use macroquad::color::Color;
use nalgebra::Point3;

#[derive(Debug)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub color: Color,
}

pub trait Mesh {
    fn verts(&self) -> &[Point3<f32>];
    fn tris(&self) -> &Vec<Triangle>;
}
