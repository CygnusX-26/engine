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
                        components.next().ok_or(format!("Missing first vertex component at line: {}", lineno + 1))?.parse()?,
                        components.next().ok_or(format!("Missing second vertex component at line: {}", lineno + 1))?.parse()?,
                        components.next().ok_or(format!("Missing third vertex component at line: {}", lineno + 1))?.parse()?,
                    ));
                }
                Some("vn") => {
                    normals.push(Vector3::new(
                        components.next().ok_or(format!("Missing first normal component at line: {}", lineno + 1))?.parse()?,
                        components.next().ok_or(format!("Missing second normal component at line: {}", lineno + 1))?.parse()?,
                        components.next().ok_or(format!("Missing third normal component at line: {}", lineno + 1))?.parse()?,
                    ));
                }
                Some("f") => {
                    let t1 = components
                        .next()
                        .ok_or(format!("Missing triangle vertex at line: {}", lineno + 1))?
                        .split("/")
                        .next()
                        .ok_or(format!("Missing first vertex value at line: {}", lineno + 1))?;
                    let t2 = components
                        .next()
                        .ok_or(format!("Missing triangle vertex at line: {}", lineno + 1))?
                        .split("/")
                        .next()
                        .ok_or(format!("Missing second vertex value at line: {}", lineno + 1))?;
                    let t3 = components
                        .next()
                        .ok_or(format!("Missing triangle vertex at line: {}", lineno + 1))?
                        .split("/")
                        .next()
                        .ok_or(format!("Missing third vertex value at line: {}", lineno + 1))?;

                    tris.push(Triangle {
                        v1: t1.parse::<usize>()? - 1,
                        v2: t3.parse::<usize>()? - 1, // do these objs specify whether the triangles are wound cw or ccw?
                        v3: t2.parse::<usize>()? - 1,
                        color: SKYBLUE,
                    });
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
