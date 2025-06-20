pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod letter_n;
pub mod p_hack;

use macroquad::color::Color;
use nalgebra::Point3;

pub trait Mesh {
    fn verts(&self) -> &[Point3<f32>];
    fn tris(&self) -> &Vec<(usize, usize, usize, Color)>;
}
