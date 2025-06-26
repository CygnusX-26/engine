use nalgebra::{Point2, Point3, Vector3};

use crate::mesh::{Color, Material, Mesh, Triangle, Vertex, SKYBLUE};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::hash::Hash;
use std::str::SplitWhitespace;

#[derive(Clone)]
pub struct GenericMesh {
    verts: Vec<Vertex>,
    tris: Vec<Triangle>,
}

impl GenericMesh {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let mut loaded_mesh: Self = Self {
            verts: vec![],
            tris: vec![],
        };
        let mut verts: Vec<Point3<f32>> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut tris: Vec<Triangle> = vec![];
        let mut mtl_map: HashMap<String, Material> = HashMap::new();
        let mut cur_mtl = Material::new();

        for (lineno, line) in read_to_string(file_name)?.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
                Some("mtllib") => {
                    let filename = components
                        .next()
                        .ok_or(format!("Missing mtl filename at line: {}", lineno))?;
                    GenericMesh::parse_mtl(filename, &mut mtl_map)
                        .map_err(|e| format!("Failed to parse MTL: at line: {}", lineno))?;
                }
                Some("usemtl") => {
                    let mtl_name = components
                        .next()
                        .ok_or(format!("Missing material name at line: {}", lineno))?;
                    cur_mtl = mtl_map
                        .get(mtl_name)
                        .ok_or(format!("Invalid material name at line: {}", lineno))?
                        .clone();
                }
                Some("s") => {
                    //TODO not implemented
                    continue;
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
                    let mut poly_verts: Vec<usize> = vec![];

                    for _ in 0..2 {
                        poly_verts.push(
                            components
                                .next()
                                .ok_or(format!("Missing vertex at line: {}", lineno + 1))?
                                .split("/")
                                .next()
                                .ok_or(format!("Missing vertex value at line: {}", lineno + 1))?
                                .parse::<usize>()?
                                - 1,
                        );
                    }

                    // We ensure that every polygon has at LEAST three verticies.
                    // Now collect the rest :)
                    for comp in components {
                        poly_verts.push(
                            comp.split("/")
                                .next()
                                .ok_or(format!("Missing vertex value at line: {}", lineno + 1))?
                                .parse::<usize>()?
                                - 1,
                        );
                    }

                    for tri in clip_ears(&mut poly_verts, &cur_mtl) {
                        tris.push(tri);
                    }
                }
                _ => continue,
            }
        }
        let mut vertices: Vec<Vertex> = verts
            .into_iter()
            .map(|v| -> Vertex {
                Vertex {
                    position: v,
                    normal: Vector3::zeros(),
                    texcoord: Point2::new(0.0, 0.0),
                }
            })
            .collect();

        for triangle in &tris {
            let i0 = triangle.v1;
            let i1 = triangle.v2;
            let i2 = triangle.v3;

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
        loaded_mesh.verts = vertices;
        loaded_mesh.tris = tris;

        Ok(loaded_mesh)
    }

    fn parse_mtl(
        file_name: &str,
        mtl_map: &mut HashMap<String, Material>,
    ) -> Result<(), Box<dyn Error>> {
        let mut cur_mtl_name = "";
        let mut cur_mtl = Material::new();
        let binding = read_to_string(file_name)?;
        for (lineno, line) in binding.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
                Some("newmtl") => {
                    if !cur_mtl_name.is_empty() {
                        mtl_map.insert(String::from(cur_mtl_name), cur_mtl);
                        cur_mtl = Material::new();
                    }
                    cur_mtl_name = components.next().ok_or(format!(
                        "Missing mtl name at line: {} in file {}",
                        lineno, file_name
                    ))?;
                }
                Some("Ka") => {
                    cur_mtl.ka = color_from_line(&mut components, lineno, file_name)?;
                }
                Some("Kd") => {
                    cur_mtl.kd = color_from_line(&mut components, lineno, file_name)?;
                }
                Some("Ks") => {
                    cur_mtl.ks = color_from_line(&mut components, lineno, file_name)?;
                }
                Some("d") | Some("Tr") => {
                    cur_mtl.transparency = components
                        .next()
                        .ok_or(format!(
                            "Missing transparency at line: {} in file {}",
                            lineno, file_name
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
                lineno, file_name
            ))?
            .parse::<f32>()?,
        g: components
            .next()
            .ok_or(format!(
                "Missing g component at line: {} in file {}",
                lineno, file_name
            ))?
            .parse::<f32>()?,
        b: components
            .next()
            .ok_or(format!(
                "Missing b component at line: {} in file {}",
                lineno, file_name
            ))?
            .parse::<f32>()?,
        a: 1.0,
    })
}

fn clip_ears(poly_verts: &mut Vec<usize>, cur_mtl: &Material) -> Vec<Triangle> {
    let mut tris: Vec<Triangle> = vec![];
    while poly_verts.len() > 2 {
        tris.push(Triangle {
            v1: poly_verts[1],
            v2: poly_verts[0],
            v3: poly_verts[2], // wont work with reversed winding order FIXME later
            mtl: cur_mtl.clone(),
        });
        poly_verts.remove(1);
    }
    tris
}
