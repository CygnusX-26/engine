mod mesh;

use mesh::Mesh as MyMesh;
use mesh::Triangle;
use mesh::Color;
use mesh::p_hack::PHackMesh;
use nalgebra::{Matrix4, Perspective3, Point2, Point3, Point4, Vector3, Vector4};
use ordered_float::OrderedFloat;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use log::error;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

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

pub struct World {
    pub camera: Camera,
    pub light: Light,
    pub models: Vec<Object>,
    pub proj_mat: Matrix4<f32>
}

impl Camera {
    pub fn generate_view_mat(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }
}

impl World {
    pub fn new(camera: Camera, light: Light, proj_mat: Matrix4<f32>, models: Vec<Object>) -> Self {
        World {
            camera,
            light,
            models,
            proj_mat
        }
    }

    pub fn draw(&mut self, view_mat: Matrix4<f32>, frame: &mut [u8]) {
        frame.fill(255);
        let mut sorted_models: Vec<(&Object, Matrix4<f32>)> = self.models
            .iter()
            .map(|model| -> (&Object, Matrix4<f32>) {
                (
                    model,
                    Matrix4::new(
                        1.0,
                        0.0,
                        0.0,
                        model.offset_x,
                        0.0,
                        1.0,
                        0.0,
                        model.offset_y,
                        0.0,
                        0.0,
                        1.0,
                        model.offset_z,
                        0.0,
                        0.0,
                        0.0,
                        1.0,
                    ),
                )
            })
            .collect();

        sorted_models.sort_by_key(|(_, model_mat)| -> OrderedFloat<f32> {
            object_depth(&self.camera, model_mat)
        });

        // Iterate over meshes in sorted zbuffer order
        for (mesh, model_mat) in &sorted_models {
            let model = &mesh.mesh;
            let mut screen_verts: Vec<Point2<f32>> = Vec::new();
            let mut zbuffer: Vec<Vector4<f32>> = Vec::new();
            let mut transformed_verts: Vec<Vector4<f32>> = Vec::new();

            let proj = self.proj_mat * view_mat * model_mat;

            for vertex in model.verts().iter().copied() {
                let persproj = proj * Point4::new(vertex.x, vertex.y, vertex.z, 1.0);
                let ndc_x = persproj.x / persproj.w;
                let ndc_y = persproj.y / persproj.w;
                let ndc_z = persproj.z / persproj.w;

                if !(0.0..=1.0).contains(&ndc_z) {
                    screen_verts.push(Point2::new(f32::NAN, f32::NAN));
                } else {
                    let screen_x = (ndc_x + 1.0) * 0.5 * WIDTH as f32;
                    let screen_y = (1.0 - ndc_y) * 0.5 * HEIGHT as f32;
                    screen_verts.push(Point2::new(screen_x, screen_y));
                }
                zbuffer.push(view_mat * model_mat * Vector4::from(vertex));
                transformed_verts.push(model_mat * Vector4::from(vertex));
            }

            //Z order each triangle in each mesh
            let mut z_ordered_tris: Vec<(&Triangle, f32)> = model
                .tris()
                .iter()
                .map(|tri| -> (&Triangle, f32) {
                    let z = (zbuffer[tri.v1].z + zbuffer[tri.v2].z + zbuffer[tri.v3].z) / 3.0;
                    (tri, z)
                })
                .collect();
            z_ordered_tris.sort_by_key(|tri| -> OrderedFloat<f32> { OrderedFloat(tri.1) });

            // Draw the triangles
            for tri in z_ordered_tris {
                let s1 = screen_verts[tri.0.v1];
                let s2 = screen_verts[tri.0.v2];
                let s3 = screen_verts[tri.0.v3];
                if !s1.x.is_finite() || !s2.x.is_finite() || !s3.x.is_finite() {
                    continue;
                }

                let v1 = transformed_verts[tri.0.v1];
                let v2 = transformed_verts[tri.0.v2];
                let v3 = transformed_verts[tri.0.v3];

                let v1 = Vector3::new(v1.x, v1.y, v1.z);
                let v2 = Vector3::new(v2.x, v2.y, v2.z);
                let v3 = Vector3::new(v3.x, v3.y, v3.z);

                let vec1 = (v2 - v1).normalize();
                let vec2 = (v3 - v1).normalize();
                let norm = vec1.cross(&vec2);

                let brightness = norm.dot(&(self.light.direction.normalize())).clamp(0.0, 1.0)
                    * self.light.intensity
                    + self.light.ambient;
                let color = Color {
                    r: ((tri.0.color.r as f32) * brightness) as u8,
                    g: ((tri.0.color.g as f32) * brightness) as u8,
                    b: ((tri.0.color.b as f32) * brightness) as u8,
                    a: tri.0.color.a,
                };

                if is_front_facing(s1, s2, s3) {
                    self.draw_triangle(s1, s2, s3, color, frame);
                }
            }
        }
    }

    fn draw_triangle(&self, t1: Point2<f32>, t2: Point2<f32>, t3: Point2<f32>, color: Color, frame: &mut [u8]) {
        let to_i32 = |p: Point2<f32>| (p.x as i32, p.y as i32);
        let (x1, y1) = to_i32(t1);
        let (x2, y2) = to_i32(t2);
        let (x3, y3) = to_i32(t3);
        let min_x = x1.min(x2).min(x3).max(0);
        let max_x = x1.max(x2).max(x3).min(WIDTH as i32 - 1);
        let min_y = y1.min(y2).min(y3).max(0);
        let max_y = y1.max(y2).max(y3).min(HEIGHT as i32 - 1);

        let edge = |(ax, ay): (i32, i32), (bx, by): (i32, i32), (px, py): (i32, i32)| -> i32 {
            (px - ax) * (by - ay) - (py - ay) * (bx - ax)
        };
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = (x, y);
                let w0 = -edge((x2, y2), (x3, y3), p);
                let w1 = -edge((x3, y3), (x1, y1), p);
                let w2 = -edge((x1, y1), (x2, y2), p);

                if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                    let index = (y as u32 * WIDTH + x as u32) * 4;
                    if index as usize + 4 <= frame.len() {
                        frame[index as usize..index as usize + 4]
                            .copy_from_slice(&[color.r, color.g, color.b, 255]);
                    }
                }
            }
        }
    }
}

/// True if the triangle faces the cam. False, we dont need to draw it.
fn is_front_facing(p1: Point2<f32>, p2: Point2<f32>, p3: Point2<f32>) -> bool {
    let cross = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    cross > 0.0
}

/// Handle key press turning and etc... TODO add mouse movement
fn handle_keys(input: &WinitInputHelper, camera: &mut Camera, move_speed: f32, turn_speed: f32) -> Matrix4<f32> {
    if input.key_held(KeyCode::KeyA) {
        camera.yaw += turn_speed;
        let radius = (camera.position - camera.target).norm();
        camera.target.x = camera.position.x + radius * camera.yaw.sin();
        camera.target.z = camera.position.z + radius * camera.yaw.cos();
    } else if input.key_held(KeyCode::KeyD) {
        camera.yaw -= turn_speed;
        let radius = (camera.position - camera.target).norm();
        camera.target.x = camera.position.x + radius * camera.yaw.sin();
        camera.target.z = camera.position.z + radius * camera.yaw.cos();
    } else if input.key_held(KeyCode::KeyW) {
        let direction: Vector3<f32> = (camera.position - camera.target).normalize();
        camera.position -= direction * move_speed;
        camera.target -= direction * move_speed;
    } else if input.key_held(KeyCode::KeyS) {
        let direction: Vector3<f32> = (camera.position - camera.target).normalize();
        camera.position += direction * move_speed;
        camera.target += direction * move_speed;
    }
    camera.generate_view_mat()
}

fn object_depth(camera: &Camera, model_mat: &Matrix4<f32>) -> OrderedFloat<f32> {
    let view_mat = camera.generate_view_mat();
    let view_model = view_mat * model_mat;
    let object_pos = view_model.transform_point(&Point3::origin());
    OrderedFloat(object_pos.z)
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new(
        Camera {
            position: Point3::new(0.0, 0.0, -5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
        },
        Light {
            direction: Vector3::new(1.0, 0.0, 1.0),
            intensity: 1.0,
            ambient: 0.2,
        },
        Perspective3::new((WIDTH as f32) / (HEIGHT as f32), 1.0, 0.1, 200.0).to_homogeneous(),
        vec![
            Object {
                mesh: Box::new(PHackMesh::new()),
                offset_x: 0.0,
                offset_y: 0.0,
                offset_z: 0.0,
            },
            Object {
                mesh: Box::new(PHackMesh::new()),
                offset_x: 3.0,
                offset_y: 0.0,
                offset_z: 3.0,
            },
        ]
    );

    let res = event_loop.run(|event, elwt| {
        let mut view_mat: Matrix4<f32> = world.camera.generate_view_mat();
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(view_mat, pixels.frame_mut());
            if let Err(err) = pixels.render() {
                error!("failed: {err}");
                elwt.exit();
                return;
            }
        }
        if input.update(&event) {
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("failed: {err}");
                    elwt.exit();
                    return;
                }
            }

            view_mat = handle_keys(&input, &mut world.camera, 0.1, 0.02);
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}
