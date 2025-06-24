use nalgebra::{Point3, Vector3};

use crate::mesh::{Mesh, Triangle, Vertex, SKYBLUE};
use std::error::Error;
use std::fs::read_to_string;

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

        for (lineno, line) in read_to_string(file_name)?.lines().enumerate() {
            let mut components = line.split_whitespace();
            match components.next() {
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

                    for tri in clip_ears(&mut poly_verts) {
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
                }
            })
            .collect();

        for triangle in &tris {
            let i0 = triangle.v1 as usize;
            let i1 = triangle.v2 as usize;
            let i2 = triangle.v3 as usize;

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
}

impl Mesh for GenericMesh {
    fn tris(&self) -> &[Triangle] {
        &self.tris
    }

    fn verts(&self) -> &[Vertex] {
        &self.verts
    }
}

fn clip_ears(poly_verts: &mut Vec<usize>) -> Vec<Triangle> {
    let mut tris: Vec<Triangle> = vec![];
    while poly_verts.len() > 2 {
        tris.push(Triangle {
            v1: poly_verts[1],
            v2: poly_verts[0],
            v3: poly_verts[2],
            color: SKYBLUE,
        });
        poly_verts.remove(1);
    }
    tris
}
