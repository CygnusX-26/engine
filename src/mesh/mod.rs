pub mod cube;
pub mod cone;
pub mod cylinder;
pub mod letter_n;
pub mod p_hack;

use nalgebra::Point3;
use macroquad::color::Color;

pub trait Mesh {
    fn verts(&self) -> &[Point3<f32>];
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)>;
}