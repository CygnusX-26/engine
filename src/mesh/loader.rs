use image::DynamicImage;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::info;
use nalgebra::{Point2, Point3, Vector3};

use crate::mesh::{Color, Material, Mesh, Normal, SKYBLUE, TextureCoord, Triangle, Vertex};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs::{File, read_to_string};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::SplitWhitespace;
use std::sync::Arc;

#[derive(Clone)]
pub struct GenericMesh {
    verts: Vec<Vertex>,
    tris: Vec<Triangle>,
    texture_coords: Vec<TextureCoord>,
    normals: Vec<Normal>,
}

impl GenericMesh {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let mut verts: Vec<Point3<f32>> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut tris: Vec<Triangle> = vec![];
        let mut texture_coords: Vec<TextureCoord> = vec![];

        let mut mtl_map: HashMap<String, Arc<Material>> = HashMap::new();
        mtl_map.insert(
            String::from("\x04\x06__default__\x05"),
            Arc::new(Default::default()),
        );
        let mut cur_mtl = "\x04\x06__default_\x05";

        let file = File::open(file_name).map_err(|e| format!("Couldn't open file: {file_name}"))?;
        let reader = BufReader::new(file);
        let total_lines = reader.lines().count();

        let pb = ProgressBar::new(total_lines as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{msg:.magenta} {spinner:.green} [{bar:.cyan/blue}] {pos}/{len} ({elapsed})",
            )
            .unwrap()
            .tick_strings(&[
                " ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▄", "▃", "▂",
            ])
            .progress_chars("#>-"),
        );
        pb.set_message(file_name.to_owned());

        for (lineno, line) in read_to_string(file_name)?.lines().enumerate() {
            pb.set_position(lineno as u64);
            let mut components = line.split_whitespace();
            match components.next() {
                Some("mtllib") => {
                    let filename = components
                        .next()
                        .ok_or(format!("Missing mtl filename at line: {}", lineno + 1))?;
                    GenericMesh::parse_mtl(filename, &mut mtl_map)?
                }
                Some("usemtl") => {
                    cur_mtl = components
                        .next()
                        .ok_or(format!("Missing material name at line: {}", lineno + 1))?;
                }
                Some("vt") => {
                    let u: f32 = components
                        .next()
                        .ok_or(format!(
                            "Missing first texture component at line: {}",
                            lineno + 1
                        ))?
                        .parse()
                        .map_err(|e| format!("Invalid f32 for u at line: {}", lineno + 1))?;
                    let v: f32 = match components.next() {
                        Some(v) => v
                            .parse()
                            .map_err(|e| format!("Invalid f32 for v at line: {}", lineno + 1))?,
                        None => 0.0,
                    };
                    let w: f32 = match components.next() {
                        Some(w) => w
                            .parse()
                            .map_err(|e| format!("Invalid f32 for w at line: {}", lineno + 1))?,
                        None => 0.0,
                    };
                    texture_coords.push(TextureCoord { u, v, w });
                }
                Some("v") => {
                    verts.push(Point3::new(
                        components
                            .next()
                            .ok_or(format!(
                                "Missing first vertex component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid first digit for vertex at line: {}", lineno + 1)
                            })?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing second vertex component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid second digit for vertex at line: {}", lineno + 1)
                            })?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing third vertex component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid third digit for vertex at line: {}", lineno + 1)
                            })?,
                    ));
                }
                Some("vn") => {
                    normals.push(Vector3::new(
                        components
                            .next()
                            .ok_or(format!(
                                "Missing first normal component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid digit for normal at line: {}", lineno + 1)
                            })?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing second normal component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid digit for normal at line: {}", lineno + 1)
                            })?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing third normal component at line: {}",
                                lineno + 1
                            ))?
                            .parse()
                            .map_err(|e| {
                                format!("Invalid digit for normal at line: {}", lineno + 1)
                            })?,
                    ));
                }
                Some("f") => {
                    let mut poly_verts: Vec<(usize, usize, usize)> = vec![];

                    for _ in 0..2 {
                        let mut face_iter = components
                            .next()
                            .ok_or(format!("Missing vertex at line: {}", lineno + 1))?
                            .split("/");
                        poly_verts.push((
                            face_iter
                                .next()
                                .ok_or(format!("Missing vertex value at line: {}", lineno + 1))?
                                .parse::<usize>()
                                .map_err(|e| {
                                    format!("Invalid vertex index digit at line: {}", lineno + 1)
                                })?
                                - 1,
                            match face_iter.next() {
                                Some(vt) => {
                                    if vt.is_empty() {
                                        0
                                    } else {
                                        vt.parse::<usize>().map_err(|e| {
                                            format!(
                                                "Invalid texture index digit at line: {}",
                                                lineno + 1
                                            )
                                        })? - 1
                                    }
                                }
                                None => 0,
                            },
                            match face_iter.next() {
                                Some(vn) => {
                                    if vn.is_empty() {
                                        0
                                    } else {
                                        vn.parse::<usize>().map_err(|e| {
                                            format!(
                                                "Invalid normal index digit at line: {}",
                                                lineno + 1
                                            )
                                        })?
                                    }
                                }
                                None => 0,
                            },
                        ));
                    }

                    // We ensure that every polygon has at LEAST three verticies.
                    // Now collect the rest :)
                    for comp in components {
                        let mut face_iter = comp.split("/");
                        poly_verts.push((
                            face_iter
                                .next()
                                .ok_or(format!("Missing vertex value at line: {}", lineno + 1))?
                                .parse::<usize>()
                                .map_err(|e| {
                                    format!("Invalid vertex index digit at line: {}", lineno + 1)
                                })?
                                - 1,
                            match face_iter.next() {
                                Some(vt) => {
                                    if vt.is_empty() {
                                        0
                                    } else {
                                        vt.parse::<usize>().map_err(|e| {
                                            format!(
                                                "Invalid texture index digit at line: {}",
                                                lineno + 1
                                            )
                                        })? - 1
                                    }
                                }
                                None => 0,
                            },
                            match face_iter.next() {
                                Some(vn) => {
                                    if vn.is_empty() {
                                        0
                                    } else {
                                        vn.parse::<usize>().map_err(|e| {
                                            format!(
                                                "Invalid normal index digit at line: {}",
                                                lineno + 1
                                            )
                                        })?
                                    }
                                }
                                None => 0,
                            },
                        ));
                    }

                    let material = mtl_map
                        .get(cur_mtl)
                        .ok_or(format!(
                            "Couldn't find material at line: {} in file: {}",
                            lineno + 1,
                            file_name
                        ))?
                        .clone();

                    for tri in clip_ears(&mut poly_verts, material) {
                        tris.push(tri);
                    }
                }
                _ => continue,
            }
        }

        // IF they did not specify normals, we average over faces.
        if normals.is_empty() {
            normals.resize(verts.len(), Vector3::zeros());
            for triangle in &tris {
                let i0 = triangle.verts[0];
                let i1 = triangle.verts[1];
                let i2 = triangle.verts[2];
                let v0 = verts[i0];
                let v1 = verts[i1];
                let v2 = verts[i2];
                let edge1 = v1 - v0;
                let edge2 = v2 - v0;
                let face_normal = edge1.cross(&edge2).normalize();

                normals[i0] += face_normal;
                normals[i1] += face_normal;
                normals[i2] += face_normal;
            }
        }
        normals = normals.iter().map(|norm| norm.normalize()).collect();
        pb.finish();
        Ok(Self {
            verts,
            tris,
            texture_coords,
            normals,
        })
    }

    fn parse_mtl(
        file_name: &str,
        mtl_map: &mut HashMap<String, Arc<Material>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut cur_mtl_name = "";
        let mut cur_mtl: Material = Default::default();
        let binding = read_to_string(file_name)?;
        for (lineno, line) in binding.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
                Some("newmtl") => {
                    if !cur_mtl_name.is_empty() {
                        mtl_map.insert(String::from(cur_mtl_name), Arc::new(cur_mtl));
                        cur_mtl = Default::default();
                    }
                    cur_mtl_name = components.next().ok_or(format!(
                        "Missing mtl name at line: {} in file {}",
                        lineno + 1,
                        file_name
                    ))?;
                }
                Some("Ka") => {
                    cur_mtl.ka = color_from_line(&mut components, lineno + 1, file_name)?;
                }
                Some("Kd") => {
                    cur_mtl.kd = color_from_line(&mut components, lineno + 1, file_name)?;
                }
                Some("Ks") => {
                    cur_mtl.ks = color_from_line(&mut components, lineno + 1, file_name)?;
                }
                Some("d") | Some("Tr") => {
                    cur_mtl.transparency = components
                        .next()
                        .ok_or(format!(
                            "Missing transparency at line: {} in file {}",
                            lineno + 1,
                            file_name
                        ))?
                        .parse::<f32>()
                        .map_err(|e| format!("Invalid float at line: {}", lineno + 1))?;
                }
                Some("Tf") => {
                    // Not supported TODO later
                    continue;
                }
                Some("Ni") => {
                    // Not supported TODO later
                    continue;
                }
                Some("map_Ka") => {
                    cur_mtl.map_ka =
                        Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                Some("map_Kd") => {
                    cur_mtl.map_kd =
                        Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                Some("map_Ks") => {
                    cur_mtl.map_ks =
                        Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                _ => {
                    continue;
                }
            }
        }
        if !cur_mtl_name.is_empty() {
            mtl_map.insert(String::from(cur_mtl_name), Arc::new(cur_mtl));
        }
        Ok(())
    }
}

impl Mesh for GenericMesh {
    fn tris(&self) -> &[Triangle] {
        &self.tris
    }

    fn verts(&self) -> &[Vertex] {
        &self.verts
    }

    fn texturecoords(&self) -> &[TextureCoord] {
        &self.texture_coords
    }

    fn normals(&self) -> &[Normal] {
        &self.normals
    }
}

fn open_image_from_line(
    components: &mut SplitWhitespace,
    lineno: usize,
    file_name: &str,
) -> Result<DynamicImage, Box<dyn Error>> {
    image::open(components.next().ok_or(format!(
        "Missing image filename at line: {} in file {}",
        lineno + 1,
        file_name
    ))?)
    .map_err(|e| {
        Box::<dyn Error>::from(format!(
            "Failed to open file at line: {} in file {}",
            lineno + 1,
            file_name
        ))
    })
}

fn color_from_line(
    components: &mut SplitWhitespace,
    lineno: usize,
    file_name: &str,
) -> Result<Color, Box<dyn Error>> {
    Ok(Color {
        r: components
            .next()
            .ok_or(format!(
                "Missing r component at line: {} in file {}",
                lineno + 1,
                file_name
            ))?
            .parse::<f32>()
            .map_err(|e| format!("Invalid f32 for red at line: {}", lineno + 1))?,
        g: components
            .next()
            .ok_or(format!(
                "Missing g component at line: {} in file {}",
                lineno + 1,
                file_name
            ))?
            .parse::<f32>()
            .map_err(|e| format!("Invalid f32 for green at line: {}", lineno + 1))?,
        b: components
            .next()
            .ok_or(format!(
                "Missing b component at line: {} in file {}",
                lineno + 1,
                file_name
            ))?
            .parse::<f32>()
            .map_err(|e| format!("Invalid f32 for blue at line: {}", lineno + 1))?,
        a: 1.0,
    })
}

fn clip_ears(poly_verts: &mut Vec<(usize, usize, usize)>, cur_mtl: Arc<Material>) -> Vec<Triangle> {
    let mut tris: Vec<Triangle> = vec![];
    while poly_verts.len() > 2 {
        tris.push(Triangle {
            verts: [poly_verts[1].0, poly_verts[0].0, poly_verts[2].0], // wont work with reversed winding order TODO later
            mtl: cur_mtl.clone(),
            texes: [poly_verts[1].1, poly_verts[0].1, poly_verts[2].1],
            norms: [poly_verts[1].2, poly_verts[0].2, poly_verts[2].2],
        });
        poly_verts.remove(1);
    }
    tris
}
