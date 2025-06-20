mod mesh;

use macroquad::prelude::*;
use mesh::Mesh as MyMesh;
use mesh::cone::ConeMesh;
use mesh::cube::CubeMesh;
use mesh::cylinder::CylinderMesh;
use mesh::letter_n::LetterNMesh;
use mesh::p_hack::PHackMesh;
use nalgebra::{Matrix4, Perspective3, Point2, Point3, Point4, Rotation3, Vector3, Vector4};
use ordered_float::OrderedFloat;

pub struct Object {
    mesh: Box<dyn MyMesh>,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
}

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
}

pub struct Light {
    pub direction: Vector3<f32>,
    pub intensity: f32,
    pub ambient: f32,
}

impl Camera {
    pub fn generate_view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }
}

fn is_front_facing(p1: Vec2, p2: Vec2, p3: Vec2) -> bool {
    let cross = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    cross > 0.0
}

fn handle_keys(camera: &mut Camera, move_speed: f32, turn_speed: f32) -> Matrix4<f32> {
    if is_key_down(KeyCode::A) {
        camera.yaw += turn_speed;
        let radius = (camera.position - camera.target).norm();
        camera.target.x = camera.position.x + radius * camera.yaw.sin();
        camera.target.z = camera.position.z + radius * camera.yaw.cos();
    } else if is_key_down(KeyCode::D) {
        camera.yaw -= turn_speed;
        let radius = (camera.position - camera.target).norm();
        camera.target.x = camera.position.x + radius * camera.yaw.sin();
        camera.target.z = camera.position.z + radius * camera.yaw.cos();
    } else if is_key_down(KeyCode::W) {
        let direction: Vector3<f32> = (camera.position - camera.target).normalize();
        camera.position -= direction * move_speed;
        camera.target -= direction * move_speed;
    } else if is_key_down(KeyCode::S) {
        let direction: Vector3<f32> = (camera.position - camera.target).normalize();
        camera.position += direction * move_speed;
        camera.target += direction * move_speed;
    }
    camera.generate_view_mat()
}

#[macroquad::main("Renderer")]
async fn main() {
    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, -5.0),
        target: Point3::new(0.0, 0.0, 0.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        pitch: 0.0,
        yaw: 0.0,
    };

    let light = Light {
        direction: Vector3::new(1.0, 0.0, 1.0),
        intensity: 1.0,
        ambient: 0.2,
    };

    let proj_mat: Matrix4<f32> =
        *Perspective3::new(screen_width() / screen_height(), 1.0, 0.1, 200.0).as_matrix();

    let mut models: Vec<Object> = Vec::new();
    models.push(Object {
        mesh: Box::new(PHackMesh::new()),
        offset_x: 0.0,
        offset_y: 0.0,
        offset_z: 0.0,
    });

    let mut radians: f32 = 0.0;

    loop {
        clear_background(WHITE);

        let view_mat: Matrix4<f32> = handle_keys(&mut camera, 0.1, 0.02);

        for mesh in models.iter() {
            let model = &mesh.mesh;
            let mut screen_verts: Vec<Point2<f32>> = Vec::new();
            let mut zbuffer: Vec<Vector4<f32>> = Vec::new();
            let mut transformed_verts: Vec<Vector4<f32>> = Vec::new();
            let model_mat = Rotation3::from_axis_angle(&Vector3::x_axis(), radians)
                .to_homogeneous()
                * Rotation3::from_axis_angle(&Vector3::z_axis(), radians * 1.5).to_homogeneous();

            let proj = proj_mat * view_mat * model_mat;

            for i in 0..model.verts().len() {
                let vertex = model.verts()[i];
                let persproj = proj * Point4::new(vertex.x, vertex.y, vertex.z, 1.0);
                let ndc_x = persproj.x / persproj.w;
                let ndc_y = persproj.y / persproj.w;
                let ndc_z = persproj.z / persproj.w;

                if ndc_z < 0.0 || ndc_z > 1.0 {
                    screen_verts.push(Point2::new(f32::NAN, f32::NAN));
                } else {
                    let screen_x = (ndc_x + 1.0) * 0.5 * screen_width();
                    let screen_y = (1.0 - ndc_y) * 0.5 * screen_height();
                    screen_verts.push(Point2::new(screen_x, screen_y));
                }
                zbuffer.push(view_mat * model_mat * Vector4::from(vertex));
                transformed_verts.push(model_mat * Vector4::from(vertex));
            }

            let mut z_ordered_tris: Vec<(usize, usize, usize, Color, f32)> = model
                .tris()
                .iter()
                .map(|tri| -> (usize, usize, usize, Color, f32) {
                    let z = (zbuffer[tri.0].z + zbuffer[tri.1].z + zbuffer[tri.2].z) / 3.0;
                    (tri.0, tri.1, tri.2, tri.3, z)
                })
                .collect();
            z_ordered_tris.sort_by_key(|tri| -> OrderedFloat<f32> { OrderedFloat(tri.4) });

            for tri in z_ordered_tris {
                let s1 = screen_verts[tri.0];
                let s2 = screen_verts[tri.1];
                let s3 = screen_verts[tri.2];
                if !s1.x.is_finite() || !s2.x.is_finite() || !s3.x.is_finite() {
                    continue;
                }

                let v1: Vector4<f32> = transformed_verts[tri.0];
                let v2: Vector4<f32> = transformed_verts[tri.1];
                let v3: Vector4<f32> = transformed_verts[tri.2];

                let v1 = Vector3::new(v1.x, v1.y, v1.z);
                let v2 = Vector3::new(v2.x, v2.y, v2.z);
                let v3 = Vector3::new(v3.x, v3.y, v3.z);

                let vec1 = (v2 - v1).normalize();
                let vec2 = (v3 - v1).normalize();
                let norm = vec1.cross(&vec2);

                let brightness = norm.dot(&(light.direction.normalize())).max(0.0).min(1.0)
                    * light.intensity
                    + light.ambient;
                let color = Color {
                    r: tri.3.r * brightness,
                    g: tri.3.g * brightness,
                    b: tri.3.b * brightness,
                    a: tri.3.a,
                };

                let t1 = Vec2 { x: s1.x, y: s1.y };
                let t2 = Vec2 { x: s2.x, y: s2.y };
                let t3 = Vec2 { x: s3.x, y: s3.y };
                if is_front_facing(t1, t2, t3) {
                    draw_triangle(t1, t2, t3, color);
                }
            }
        }

        radians += 0.00;
        next_frame().await;
    }
}
