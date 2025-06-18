mod mesh;

use nalgebra::{Matrix4, Point3, Vector3, Perspective3, Point2, Point4, Rotation3};
use macroquad::prelude::*;
use mesh::cube::cube_mesh::CubeMesh;
use mesh::Mesh as MyMesh;

use crate::mesh::cone::cone_mesh::ConeMesh;

pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Point3<f32>,
    pub up: Vector3<f32>
}

pub struct Light {
    pub direction: Point3<f32>
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

    let _light = Light { direction: Point3::new(0.0, 0.0, 1.0)};

    let view_mat: Matrix4<f32> = camera.generate_view_mat();
    let proj_mat: Matrix4<f32> = *Perspective3::new(1.0, 1.0, 0.1, 200.0).as_matrix();

    let mut models: Vec<(Box<dyn MyMesh>, f32, f32)> = Vec::new();
    models.push((Box::new(ConeMesh::new(2.0, 1.5)), 200.0, 0.0));
    models.push((Box::new(CubeMesh::new()), 300.0, 0.0));

    const SCALE: f32 = 100.0;
    let mut radians: f32 = 0.0;

    loop {
        clear_background(WHITE);

        for mesh in models.iter() {
            let model = &mesh.0;
            let mut screen_verts: Vec<Point2<f32>> = Vec::new();
            let model_mat = Rotation3::from_axis_angle(&Vector3::x_axis(), radians).to_homogeneous()
                * Rotation3::from_axis_angle(&Vector3::z_axis(), radians * 1.5).to_homogeneous();

            
            for i in 0..model.verts().len() {
                let vertex = model.verts()[i];
                let persproj = proj_mat * view_mat * model_mat * Point4::new(vertex.x, vertex.y, vertex.z, 1.0);
                screen_verts.push(Point2::new(persproj.x / persproj.z, persproj.y / persproj.z))
            }

            let shift: f32 = mesh.1;
            for tri in model.tris() {
                let t1 = Vec2 { x: SCALE * screen_verts[tri.0].x + shift, y: SCALE * screen_verts[tri.0].y + shift};
                let t2 = Vec2 { x: SCALE * screen_verts[tri.1].x + shift, y: SCALE * screen_verts[tri.1].y + shift};
                let t3 = Vec2 { x: SCALE * screen_verts[tri.2].x + shift, y: SCALE * screen_verts[tri.2].y + shift};
                if is_front_facing(t1, t2, t3) {
                    draw_triangle( t1, t2, t3, tri.3 );
                }
            }
        }
        
        radians += 0.01;
        next_frame().await;
    }
}