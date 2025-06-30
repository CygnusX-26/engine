#![allow(unused)]
pub mod loader;
pub mod premade;

use std::{
    ops::{Add, Mul},
    sync::Arc,
};

use image::DynamicImage;
use nalgebra::{Point2, Point3, Vector3};

pub trait Mesh: Sync {
    fn verts(&self) -> &[Vertex];
    fn tris(&self) -> &[Triangle];
    fn texturecoords(&self) -> &[TextureCoord];
    fn normals(&self) -> &[Normal];
}

#[derive(Debug, Copy, Clone)]
pub struct TextureCoord {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

pub type Vertex = Point3<f32>;
pub type Normal = Vector3<f32>;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub verts: [usize; 3], // vertex indicies
    pub texes: [usize; 3], // texture indicies
    pub norms: [usize; 3],
    pub mtl: Arc<Material>,
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
    pub map_ka: Option<DynamicImage>,
    pub map_kd: Option<DynamicImage>,
    pub map_ks: Option<DynamicImage>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ka: DIM,
            kd: DIM,
            ks: DIM,
            transparency: 0.0,
            tf: BLACK,
            ni: 0.0,
            map_ka: None,
            map_kd: None,
            map_ks: None,
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

pub const DIM: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0
};
