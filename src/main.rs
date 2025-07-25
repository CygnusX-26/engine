mod mesh;

use image::DynamicImage;
use image::GenericImageView;
use image::Pixel;
use mesh::Material;
use mesh::Mesh;
use mesh::loader::GenericMesh;

use clap::Parser;
use log::{error, info};
use nalgebra::{Matrix4, Perspective3, Point2, Point3, Point4, Vector3};
use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::mesh::Color;
use crate::mesh::Normal;
use crate::mesh::TextureCoord;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

struct Object {
    mesh: Box<dyn Mesh>,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, help = "flip all normals")]
    normals: bool,
}

struct TriParam {
    screen_verts: [Point2<f32>; 3],
    normals: [Vector3<f32>; 3],
    z_values: [f32; 3],
    texture_coords: Option<[TextureCoord; 3]>,
    w_values: [f32; 3],
}

struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
}

struct Light {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub ambient: f32,
}

struct World {
    pub camera: Camera,
    pub light: Light,
    pub models: Vec<Object>,
    pub proj_mat: Matrix4<f32>,
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
            proj_mat,
        }
    }

    pub fn draw(&mut self, view_mat: Matrix4<f32>, frame: &mut [u8], flip_normals: bool) {
        frame.fill(255);
        let light_dir_world = (self.light.target - self.light.position).normalize();
        let light_dir_view = (view_mat.fixed_view::<3, 3>(0, 0) * light_dir_world).normalize();
        let model_with_mats: Vec<(&Object, Matrix4<f32>)> = self
            .models
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

        let mut screen_verts: Vec<Point2<f32>> = vec![];
        let mut zvalues: Vec<f32> = vec![];
        let mut wvalues: Vec<f32> = vec![];
        let mut zbuffer: Vec<AtomicU32> = (0..WIDTH * HEIGHT)
            .map(|_| AtomicU32::new(f32::to_bits(1.0)))
            .collect();

        // Iterate over meshes in sorted zbuffer order
        for (model, model_mat) in &model_with_mats {
            let mesh = &model.mesh;
            let normal_mat = (view_mat * model_mat)
                .fixed_view::<3, 3>(0, 0)
                .try_inverse()
                .unwrap()
                .transpose();
            let proj = self.proj_mat * view_mat * model_mat;

            for vertex in mesh.verts().iter().copied() {
                let persproj = proj * Point4::new(vertex.x, vertex.y, vertex.z, 1.0);
                let ndc_x = persproj.x / persproj.w;
                let ndc_y = persproj.y / persproj.w;
                let ndc_z = persproj.z / persproj.w;
                let ndc_w = 1.0 / persproj.w;

                if !(0.0..=1.0).contains(&ndc_z) {
                    screen_verts.push(Point2::new(f32::NAN, f32::NAN));
                } else {
                    let screen_x = (ndc_x + 1.0) * 0.5 * WIDTH as f32;
                    let screen_y = (1.0 - ndc_y) * 0.5 * HEIGHT as f32;
                    screen_verts.push(Point2::new(screen_x, screen_y));
                }
                wvalues.push(ndc_w);
                zvalues.push(ndc_z);
            }

            let transformed_norms: Vec<Normal> = model
                .mesh
                .normals()
                .iter()
                .map(|norm| normal_mat * norm)
                .collect();

            // Draw the triangles
            for tri in mesh.tris() {
                let vert1_index = tri.verts[0];
                let vert2_index = tri.verts[1];
                let vert3_index = tri.verts[2];
                let s1 = screen_verts[vert1_index];
                let s2 = screen_verts[vert2_index];
                let s3 = screen_verts[vert3_index];
                if !should_be_rendered(s1, s2, s3) || !is_front_facing(s1, s2, s3) {
                    continue;
                }
                let texture_coords = mesh.texturecoords();

                let n1_idx = tri.norms[0];
                let n2_idx = tri.norms[1];
                let n3_idx = tri.norms[2];
                let mut n1;
                let mut n2;
                let mut n3;

                if n1_idx > 0 && n2_idx > 0 && n3_idx > 0 {
                    n1 = transformed_norms[n1_idx - 1];
                    n2 = transformed_norms[n2_idx - 1];
                    n3 = transformed_norms[n3_idx - 1];
                } else {
                    n1 = transformed_norms[vert1_index];
                    n2 = transformed_norms[vert2_index];
                    n3 = transformed_norms[vert3_index];
                }

                if flip_normals {
                    n1 = -n1;
                    n2 = -n2;
                    n3 = -n3;
                }

                let z1 = zvalues[vert1_index];
                let z2 = zvalues[vert2_index];
                let z3 = zvalues[vert3_index];

                let w1 = wvalues[vert1_index];
                let w2 = wvalues[vert2_index];
                let w3 = wvalues[vert3_index];

                let t1 = texture_coords.get(tri.texes[0]);
                let t2 = texture_coords.get(tri.texes[1]);
                let t3 = texture_coords.get(tri.texes[2]);

                self.draw_triangle(
                    TriParam {
                        screen_verts: [s1, s2, s3],
                        normals: [n1, n2, n3],
                        z_values: [z1, z2, z3],
                        texture_coords: match (t1, t2, t3) {
                            (Some(tc1), Some(tc2), Some(tc3)) => Some([*tc1, *tc2, *tc3]),
                            _ => None,
                        },
                        w_values: [w1, w2, w3],
                    },
                    &tri.mtl,
                    frame,
                    &mut zbuffer,
                    light_dir_view,
                );
            }
            screen_verts.clear();
            zvalues.clear();
        }
    }

    fn draw_triangle(
        &self,
        tri: TriParam,
        mtl: &Material,
        frame: &mut [u8],
        zbuffer: &mut [AtomicU32],
        light_dir_view: Vector3<f32>,
    ) {
        let (x1, y1) = (tri.screen_verts[0].x, tri.screen_verts[0].y);
        let (x2, y2) = (tri.screen_verts[1].x, tri.screen_verts[1].y);
        let (x3, y3) = (tri.screen_verts[2].x, tri.screen_verts[2].y);
        let min_x = (x1.min(x2).min(x3).max(0.0)) as usize;
        let max_x = (x1.max(x2).max(x3).min(WIDTH as f32 - 1.0) + 1.0) as usize;
        let min_y = (y1.min(y2).min(y3).max(0.0)) as usize;
        let max_y = (y1.max(y2).max(y3).min(HEIGHT as f32 - 1.0) + 1.0) as usize;

        if min_x > max_x || min_y > max_y {
            return;
        }

        let z1 = tri.z_values[0];
        let z2 = tri.z_values[1];
        let z3 = tri.z_values[2];
        let perspective_warp_1 = tri.w_values[0];
        let perspective_warp_2 = tri.w_values[1];
        let perspective_warp_3 = tri.w_values[2];
        let n1 = tri.normals[0];
        let n2 = tri.normals[1];
        let n3 = tri.normals[2];
        let ambient = self.light.ambient;

        let edge = |(ax, ay): (f32, f32), (bx, by): (f32, f32), (px, py): (f32, f32)| -> f32 {
            (py - ay) * (bx - ax) - (px - ax) * (by - ay)
        };

        let row_stride = WIDTH * 4;

        frame
            .par_chunks_exact_mut(row_stride)
            .skip(min_y)
            .take(max_y - min_y)
            .enumerate()
            .for_each(|(row_idx, row)| {
                let y = row_idx + min_y;
                for x in min_x..=max_x {
                    let p = (x as f32, y as f32);
                    let mut w1 = edge((x2, y2), (x3, y3), p);
                    let mut w2 = edge((x3, y3), (x1, y1), p);
                    let mut w3 = edge((x1, y1), (x2, y2), p);
                    if w1 < 0.0 || w2 < 0.0 || w3 < 0.0 {
                        continue;
                    }

                    let idx = x * 4;
                    if idx + 4 > row.len() {
                        continue;
                    }

                    let z_index = y * WIDTH + x;
                    if z_index >= WIDTH * HEIGHT {
                        continue;
                    }

                    let area = w1 + w2 + w3;

                    w1 /= area;
                    w2 /= area;
                    w3 /= area;

                    let current_z = &zbuffer[z_index];
                    let current_z_bits = current_z.load(Ordering::Relaxed);
                    let interpolated_z = w1 * z1 + w2 * z2 + w3 * z3;
                    if interpolated_z > f32::from_bits(current_z_bits)
                        || current_z
                            .compare_exchange(
                                current_z_bits,
                                f32::to_bits(interpolated_z),
                                Ordering::Relaxed,
                                Ordering::Relaxed,
                            )
                            .is_err()
                    {
                        continue;
                    }

                    let mut ka = mtl.ka;
                    let mut kd = mtl.kd;
                    let mut ks = mtl.ks;

                    let one_over_z =
                        w1 * perspective_warp_1 + w2 * perspective_warp_2 + w3 * perspective_warp_3;

                    if let Some([uv1, uv2, uv3]) = tri.texture_coords {
                        let u_over_z = w1 * uv1.u * perspective_warp_1
                            + w2 * uv2.u * perspective_warp_2
                            + w3 * uv3.u * perspective_warp_3;
                        let v_over_z = w1 * uv1.v * perspective_warp_1
                            + w2 * uv2.v * perspective_warp_2
                            + w3 * uv3.v * perspective_warp_3;

                        let interpolated_u = (u_over_z / one_over_z).clamp(0.0, 1.0);
                        let interpolated_v = 1.0 - (v_over_z / one_over_z).clamp(0.0, 1.0);

                        if let Some(ref tex) = mtl.map_ka {
                            ka = sample_texture(interpolated_u, interpolated_v, tex);
                        }
                        if let Some(ref tex) = mtl.map_kd {
                            kd = sample_texture(interpolated_u, interpolated_v, tex);
                        }
                        if let Some(ref tex) = mtl.map_ks {
                            ks = sample_texture(interpolated_u, interpolated_v, tex);
                        }
                    }
                    let nx_over_z = w1 * n1.x * perspective_warp_1
                        + w2 * n2.x * perspective_warp_2
                        + w3 * n3.x * perspective_warp_3;
                    let ny_over_z = w1 * n1.y * perspective_warp_1
                        + w2 * n2.y * perspective_warp_2
                        + w3 * n3.y * perspective_warp_3;
                    let nz_over_z = w1 * n1.z * perspective_warp_1
                        + w2 * n2.z * perspective_warp_2
                        + w3 * n3.z * perspective_warp_3;

                    let interpolated_normal = Vector3::new(
                        nx_over_z / one_over_z,
                        ny_over_z / one_over_z,
                        nz_over_z / one_over_z,
                    )
                    .normalize();

                    let diffuse = light_dir_view.dot(&interpolated_normal).clamp(0.1, 1.0);
                    let specular = 0.0; //no fancy lighting for now its too laggy
                    let color = ka * ambient + kd * diffuse + ks * specular;
                    row[idx..idx + 4].copy_from_slice(&[
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        (color.a * 255.0) as u8,
                    ]);
                }
            });
    }
}

/// True if the triangle faces the cam. False, we dont need to draw it.
#[inline(always)]
fn is_front_facing(p1: Point2<f32>, p2: Point2<f32>, p3: Point2<f32>) -> bool {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x) > 0.0
}

#[inline(always)]
fn camera_shift(camera: &mut Camera, delta: Vector3<f32>) {
    camera.position.x += delta.x;
    camera.position.z += delta.z;
    camera.target.x += delta.x;
    camera.target.z += delta.z;
}

/// Handle key press turning and etc...
fn handle_keys(input: &WinitInputHelper, camera: &mut Camera, move_speed: f32) -> Matrix4<f32> {
    if input.key_held(KeyCode::KeyA) {
        let mut delta: Vector3<f32> = (camera.position - camera.target)
            .normalize()
            .cross(&camera.up)
            * move_speed;
        delta -= delta.dot(&camera.up) * camera.up;
        delta = delta.normalize() * move_speed;
        camera_shift(camera, delta);
    } else if input.key_held(KeyCode::KeyD) {
        let mut delta: Vector3<f32> = (camera.position - camera.target)
            .normalize()
            .cross(&camera.up)
            * move_speed;
        delta -= delta.dot(&camera.up) * camera.up;
        delta = delta.normalize() * move_speed;
        camera_shift(camera, -delta);
    } else if input.key_held(KeyCode::KeyW) {
        let mut delta: Vector3<f32> = (camera.position - camera.target).normalize();
        delta -= delta.dot(&camera.up) * camera.up;
        delta = delta.normalize() * move_speed;
        camera_shift(camera, -delta);
    } else if input.key_held(KeyCode::KeyS) {
        let mut delta: Vector3<f32> = (camera.position - camera.target).normalize();
        delta -= delta.dot(&camera.up) * camera.up;
        delta = delta.normalize() * move_speed;
        camera_shift(camera, delta);
    } else if input.key_held(KeyCode::Space) {
        camera.position.y += move_speed;
        camera.target.y += move_speed;
    } else if input.key_held(KeyCode::ShiftLeft) {
        camera.position.y -= move_speed;
        camera.target.y -= move_speed;
    }
    camera.generate_view_mat()
}

// TODO fix triangles diappearing
#[inline(always)]
fn should_be_rendered(s1: Point2<f32>, s2: Point2<f32>, s3: Point2<f32>) -> bool {
    s1.x.is_finite() && s2.x.is_finite() && s3.x.is_finite()
}

#[inline(always)]
fn sample_texture(interp_u: f32, interp_v: f32, tex: &DynamicImage) -> Color {
    let u = (interp_u * (tex.width() - 1) as f32).round() as u32;
    let v = (interp_v * (tex.height() - 1) as f32).round() as u32;
    let pixel = tex
        .get_pixel(u.min(tex.width() - 1), v.min(tex.height() - 1))
        .to_rgb();
    Color {
        r: pixel[0] as f32 / 255.0,
        g: pixel[1] as f32 / 255.0,
        b: pixel[2] as f32 / 255.0,
        a: 1.0,
    }
}

/// TODO: Add better documentation and clean up code
fn main() -> Result<(), Error> {
    env_logger::init();
    let cli = Args::parse();
    let filename = cli.filename;
    info!("Loading mesh for {filename}");
    let mesh = GenericMesh::from_file(&filename).unwrap_or_else(|e| {
        error!("{e:?}");
        std::process::exit(1);
    });
    info!("Done loading mesh for {filename}");
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

    window
        .set_cursor_grab(winit::window::CursorGrabMode::Locked)
        .unwrap();
    window.set_cursor_visible(false);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let mut world = World::new(
        Camera {
            position: Point3::new(0.0, 0.0, -10.0),
            target: Point3::new(0.0, 0.0, -9.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
        },
        Light {
            position: Point3::new(0.0, 1.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            ambient: 0.1,
        },
        Perspective3::new(
            (WIDTH as f32) / (HEIGHT as f32),
            (2.0 * std::f32::consts::PI) / 5.0,
            0.1,
            2000.0,
        )
        .to_homogeneous(),
        vec![Object {
            mesh: Box::new(mesh),
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
        }],
    );

    let res = event_loop.run(|event, elwt| {
        let view_mat: Matrix4<f32> = world.camera.generate_view_mat();
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(view_mat, pixels.frame_mut(), cli.normals);
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

            let (dx, dy) = input.mouse_diff();
            let sensitivity = 0.003;
            world.camera.yaw -= dx * sensitivity;
            world.camera.pitch -= dy * sensitivity;

            let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
            world.camera.pitch = world.camera.pitch.clamp(-max_pitch, max_pitch);

            let radius = (world.camera.position - world.camera.target).norm();
            let yaw = world.camera.yaw;
            let pitch = world.camera.pitch;

            world.camera.target.x = world.camera.position.x + radius * pitch.cos() * yaw.sin();
            world.camera.target.y = world.camera.position.y + radius * pitch.sin();
            world.camera.target.z = world.camera.position.z + radius * pitch.cos() * yaw.cos();
            handle_keys(&input, &mut world.camera, 1.0);
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}
