mod mesh;

use nalgebra::{Matrix4, Perspective3, Point2, Point3, Point4, Rotation3, Vector3, Vector4};
use macroquad::prelude::*;
use mesh::cube::CubeMesh;
use mesh::cylinder::CylinderMesh;
use mesh::cone::ConeMesh;
use mesh::Mesh as MyMesh;
use ordered_float::OrderedFloat;

use crate::mesh::letter_n::LetterNMesh;

pub struct Object(Box<dyn MyMesh>, f32, f32);

pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Point3<f32>,
    pub up: Vector3<f32>
}

pub struct Light {
    pub direction: Vector3<f32>,
    pub intensity: f32
}

impl Camera {
    pub fn generate_view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.direction, &self.up)
    }
}

fn is_front_facing(p1: Vec2, p2: Vec2, p3: Vec2) -> bool {
    let cross = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    cross < 0.0
}

#[macroquad::main("Renderer")]
async fn main() {
    let camera = Camera {
        position: Point3::new(0.0, 0.0,4.0), 
        direction: Point3::new(0.0, 0.0, 0.0),
        up: Vector3::new(0.0, 1.0, 0.0)
    };

    let light = Light { direction: Vector3::new(0.0, 1.0, -2.0), intensity: 1.0};
    let view_mat: Matrix4<f32> = camera.generate_view_mat();
    let proj_mat: Matrix4<f32> = *Perspective3::new(screen_width()/screen_height(), 1.0, 0.1, 200.0).as_matrix();

    let mut models: Vec<Object> = Vec::new();
    models.push(Object(Box::new(ConeMesh::new(2.0, 1.0)), 100.0, 50.0));
    models.push(Object(Box::new(CubeMesh::new()), 200.0, 50.0));
    models.push(Object(Box::new(ConeMesh::new(2.0, 1.0)), 300.0, 50.0));
    models.push(Object(Box::new(CubeMesh::new()), 400.0, 50.0));
    models.push(Object(Box::new(CubeMesh::new()), 500.0, 50.0));
    models.push(Object(Box::new(CylinderMesh::new(3.0, 1.0)), 600.0, 50.0));
    models.push(Object(Box::new(LetterNMesh::new()), 400.0, 200.0));

    const SCALE: f32 = 50.0;
    let mut radians: f32 = 0.0;

    loop {
        clear_background(WHITE);

        let mut zbuffer: Vec<f32> = Vec::new();

        for mesh in models.iter() {
            let model = &mesh.0;
            let mut screen_verts: Vec<Point2<f32>> = Vec::new();
            let model_mat = Rotation3::from_axis_angle(&Vector3::x_axis(), radians).to_homogeneous()
                * Rotation3::from_axis_angle(&Vector3::z_axis(), radians * 2.0).to_homogeneous();

            let proj = proj_mat * view_mat * model_mat;     

            for i in 0..model.verts().len() {
                let vertex = model.verts()[i];
                let persproj = proj * Point4::new(vertex.x, vertex.y, vertex.z, 1.0);
                zbuffer.push(persproj.z);
                screen_verts.push(Point2::new(persproj.x / persproj.z, persproj.y / persproj.z));
            }

            let mut z_ordered_tris: Vec<(usize, usize, usize, Color, f32)> = model.tris().iter().map(
                |tri| -> (usize, usize, usize, Color, f32) {
                    let z = (model.verts()[tri.0].z +
                        model.verts()[tri.1].z +
                        model.verts()[tri.2].z) / 3.0;
                    (tri.0, tri.1, tri.2, tri.3, z)
                }
            ).collect();
            z_ordered_tris.sort_by_key(
                |tri| -> OrderedFloat<f32> {
                    OrderedFloat(tri.4)
                }
            );

            let shift_x: f32 = mesh.1;
            let shift_y: f32 = mesh.2;
            for tri in z_ordered_tris {
                let s1 = screen_verts[tri.0];
                let s2 = screen_verts[tri.1];
                let s3 = screen_verts[tri.2];

                let v1: Vector4<f32> =  model_mat * Vector4::from(model.verts()[tri.0]);
                let v2: Vector4<f32> =  model_mat * Vector4::from(model.verts()[tri.1]);
                let v3: Vector4<f32> =  model_mat * Vector4::from(model.verts()[tri.2]);

                let v1 = Vector3::new(v1.x, v1.y, v1.z);
                let v2 = Vector3::new(v2.x, v2.y, v2.z);
                let v3 = Vector3::new(v3.x, v3.y, v3.z);

                let vec1 = (v2 - v1).normalize();
                let vec2 = (v3 - v1).normalize();
                let norm = vec1.cross(&vec2);

                let brightness = norm.dot(&(light.direction.normalize())).max(0.0).min(1.0) * light.intensity;
                let color = Color {
                    r: tri.3.r * brightness,
                    g: tri.3.g * brightness,
                    b: tri.3.b * brightness,
                    a: tri.3.a
                };

                let t1 = Vec2 { x: SCALE * s1.x + shift_x, y: SCALE * s1.y + shift_y};
                let t2 = Vec2 { x: SCALE * s2.x + shift_x, y: SCALE * s2.y + shift_y};
                let t3 = Vec2 { x: SCALE * s3.x + shift_x, y: SCALE * s3.y + shift_y};
                if is_front_facing(t1, t2, t3) {
                    draw_triangle( t1, t2, t3, color );
                }
            }
        }
        
        radians += 0.01;
        next_frame().await;
    }
}