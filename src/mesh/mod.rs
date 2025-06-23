// pub mod cone;
pub mod cube;
// pub mod cylinder;
// pub mod letter_n;
pub mod p_hack;

use nalgebra::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

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
    pub a: u8,
}
pub trait Mesh: Sync {
    fn verts(&self) -> &[Vertex];
    fn tris(&self) -> &[Triangle];
}

pub const RED: Color = Color {
    r: 224,
    g: 4,
    b: 4,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 32,
    g: 32,
    b: 230,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 10,
    g: 196,
    b: 44,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 255,
    g: 248,
    b: 54,
    a: 255,
};
pub const PURPLE: Color = Color {
    r: 171,
    g: 54,
    b: 255,
    a: 255,
};
pub const PINK: Color = Color {
    r: 196,
    g: 10,
    b: 178,
    a: 255,
};
pub const GOLD: Color = Color {
    r: 196,
    g: 159,
    b: 10,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 196,
    g: 106,
    b: 10,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 75,
    g: 75,
    b: 75,
    a: 255,
};
pub const SKYBLUE: Color = Color {
    r: 104,
    g: 131,
    b: 237,
    a: 255,
};
pub const DARKBLUE: Color = Color {
    r: 10,
    g: 27,
    b: 94,
    a: 255,
};
pub const BEIGE: Color = Color {
    r: 245,
    g: 245,
    b: 220,
    a: 255,
};
pub const LIME: Color = Color {
    r: 191,
    g: 255,
    b: 0,
    a: 255,
};
