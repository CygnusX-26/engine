use image::DynamicImage;
use nalgebra::{Dynamic, Point2, Point3, Vector3};

use crate::mesh::{Color, Material, Mesh, TextureCoord, Triangle, Vertex, SKYBLUE};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::hash::Hash;
use std::str::SplitWhitespace;

#[derive(Clone)]
pub struct GenericMesh {
    verts: Vec<Vertex>,
    tris: Vec<Triangle>,
    texture_coords: Vec<TextureCoord>,
}

impl GenericMesh {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let mut verts: Vec<Point3<f32>> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut tris: Vec<Triangle> = vec![];
        let mut texture_coords: Vec<TextureCoord> = vec![];
        let mut mtl_map: HashMap<String, Material> = HashMap::new();
        let mut cur_mtl = Default::default();

        for (lineno, line) in read_to_string(file_name)?.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
                Some("mtllib") => {
                    let filename = components
                        .next()
                        .ok_or(format!("Missing mtl filename at line: {}", lineno + 1))?;
                    GenericMesh::parse_mtl(filename, &mut mtl_map)
                        .map_err(|e| format!("Failed to parse MTL: at line: {}", lineno + 1))?;
                }
                Some("usemtl") => {
                    let mtl_name = components
                        .next()
                        .ok_or(format!("Missing material name at line: {}", lineno + 1))?;
                    cur_mtl = mtl_map
                        .get(mtl_name)
                        .ok_or(format!("Invalid material name at line: {}", lineno + 1))?
                        .clone();
                }
                Some("vt") => {
                    let u: f32 = components
                        .next()
                        .ok_or(format!(
                            "Missing first texture component at line: {}",
                            lineno + 1
                        ))?
                        .parse()?;
                    let v: f32 = match components.next() {
                        Some(v) => v.parse()?,
                        None => 0.0,
                    };
                    let w: f32 = match components.next() {
                        Some(w) => w.parse()?,
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
                            .parse()?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing second vertex component at line: {}",
                                lineno + 1
                            ))?
                            .parse()?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing third vertex component at line: {}",
                                lineno + 1
                            ))?
                            .parse()?,
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
                            .parse()?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing second normal component at line: {}",
                                lineno + 1
                            ))?
                            .parse()?,
                        components
                            .next()
                            .ok_or(format!(
                                "Missing third normal component at line: {}",
                                lineno + 1
                            ))?
                            .parse()?,
                    ));
                }
                Some("f") => {
                    let mut poly_verts: Vec<(usize, usize)> = vec![];

                    for _ in 0..2 {
                        let mut face_iter = components
                            .next()
                            .ok_or(format!("Missing vertex at line: {}", lineno + 1))?
                            .split("/");
                        poly_verts.push((
                            face_iter
                                .next()
                                .ok_or(format!("Missing vertex value at line: {}", lineno + 1))?
                                .parse::<usize>()?
                                - 1,
                            match face_iter.next() {
                                Some(vt) => {
                                    if vt.is_empty() {
                                        0
                                    } else {
                                        vt.parse::<usize>()? - 1
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
                                .parse::<usize>()?
                                - 1,
                            match face_iter.next() {
                                Some(vt) => {
                                    if vt.is_empty() {
                                        0
                                    } else {
                                        vt.parse::<usize>()? - 1
                                    }
                                }
                                None => 0,
                            },
                        ));
                    }

                    for tri in clip_ears(&mut poly_verts, &cur_mtl) {
                        tris.push(tri);
                    }
                }
                _ => continue,
            }
        }

        let max_len = verts.len().max(normals.len()).max(texture_coords.len());

        verts.resize(max_len, Point3::origin());
        normals.resize(max_len, Vector3::zeros());

        let mut vertices: Vec<Vertex> = verts
            .into_iter()
            .zip(normals)
            .map(|(v, n)| -> Vertex {
                Vertex {
                    position: v,
                    normal: n,
                }
            })
            .collect();

        for triangle in &tris {
            let i0 = triangle.v[0];
            let i1 = triangle.v[1];
            let i2 = triangle.v[2];
            let v0 = vertices[i0].position;
            let v1 = vertices[i1].position;
            let v2 = vertices[i2].position;
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = edge1.cross(&edge2).normalize();

            vertices[i0].normal += face_normal;
            vertices[i1].normal += face_normal;
            vertices[i2].normal += face_normal;
        }
        for vertex in &mut vertices {
            vertex.normal = vertex.normal.normalize();
        }

        Ok(Self {
            verts: vertices,
            tris,
            texture_coords,
        })
    }

    fn parse_mtl(
        file_name: &str,
        mtl_map: &mut HashMap<String, Material>,
    ) -> Result<(), Box<dyn Error>> {
        let mut cur_mtl_name = "";
        let mut cur_mtl = Default::default();
        let binding = read_to_string(file_name)?;
        for (lineno, line) in binding.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
                Some("newmtl") => {
                    if !cur_mtl_name.is_empty() {
                        mtl_map.insert(String::from(cur_mtl_name), cur_mtl);
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
                        .parse::<f32>()?;
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
                    cur_mtl.map_ka = Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                Some("map_Kd") => {
                    cur_mtl.map_kd = Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                Some("map_Ks") => {
                    cur_mtl.map_ks = Some(open_image_from_line(&mut components, lineno, file_name)?);
                }
                _ => {
                    continue;
                }
            }
        }
        if !cur_mtl_name.is_empty() {
            mtl_map.insert(String::from(cur_mtl_name), cur_mtl);
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
            .parse::<f32>()?,
        g: components
            .next()
            .ok_or(format!(
                "Missing g component at line: {} in file {}",
                lineno + 1,
                file_name
            ))?
            .parse::<f32>()?,
        b: components
            .next()
            .ok_or(format!(
                "Missing b component at line: {} in file {}",
                lineno + 1,
                file_name
            ))?
            .parse::<f32>()?,
        a: 1.0,
    })
}

fn clip_ears(poly_verts: &mut Vec<(usize, usize)>, cur_mtl: &Material) -> Vec<Triangle> {
    let mut tris: Vec<Triangle> = vec![];
    while poly_verts.len() > 2 {
        tris.push(Triangle {
            v: [poly_verts[1].0, poly_verts[0].0, poly_verts[2].0], // wont work with reversed winding order FIXME later
            mtl: cur_mtl.clone(),
            t: [poly_verts[1].1, poly_verts[0].1, poly_verts[2].1],
        });
        poly_verts.remove(1);
    }
    tris
}
