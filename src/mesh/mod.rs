#![allow(unused)]
pub mod loader;
pub mod premade;

use std::ops::{Add, Mul};

use nalgebra::{Point2, Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Point2<f32>,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub mtl: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Debug)]
pub struct Material {
    pub ka: Color,
    pub kd: Color,
    pub ks: Color,
    pub transparency: f32,
    pub tf: Color,
    pub ni: f32,
}

impl Material {
    pub fn new() -> Self {
        Self {
            ka: SKYBLUE,
            kd: SKYBLUE,
            ks: SKYBLUE,
            transparency: 0.0,
            tf: BLACK,
            ni: 0.0,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a,
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, color: Color) -> Self::Output {
        Self {
            r: (self.r + color.r),
            g: (self.g + color.g),
            b: (self.b + color.b),
            a: self.a,
        }
    }
}

pub trait Mesh: Sync {
    fn verts(&self) -> &[Vertex];
    fn tris(&self) -> &[Triangle];
}

pub const RED: Color = Color {
    r: 0.878,
    g: 0.016,
    b: 0.016,
    a: 1.0,
};
pub const BLUE: Color = Color {
    r: 0.125,
    g: 0.125,
    b: 0.902,
    a: 1.0,
};
pub const GREEN: Color = Color {
    r: 0.039,
    g: 0.769,
    b: 0.173,
    a: 1.0,
};
pub const YELLOW: Color = Color {
    r: 1.0,
    g: 0.973,
    b: 0.212,
    a: 1.0,
};
pub const PURPLE: Color = Color {
    r: 0.671,
    g: 0.212,
    b: 1.0,
    a: 1.0,
};
pub const PINK: Color = Color {
    r: 0.769,
    g: 0.039,
    b: 0.698,
    a: 1.0,
};
pub const GOLD: Color = Color {
    r: 0.769,
    g: 0.624,
    b: 0.039,
    a: 1.0,
};
pub const ORANGE: Color = Color {
    r: 0.769,
    g: 0.416,
    b: 0.039,
    a: 1.0,
};
pub const GRAY: Color = Color {
    r: 0.294,
    g: 0.294,
    b: 0.294,
    a: 1.0,
};
pub const SKYBLUE: Color = Color {
    r: 0.408,
    g: 0.514,
    b: 0.929,
    a: 1.0,
};
pub const DARKBLUE: Color = Color {
    r: 0.039,
    g: 0.106,
    b: 0.369,
    a: 1.0,
};
pub const BEIGE: Color = Color {
    r: 0.961,
    g: 0.961,
    b: 0.863,
    a: 1.0,
};
pub const LIME: Color = Color {
    r: 0.749,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
