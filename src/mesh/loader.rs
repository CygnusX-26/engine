use nalgebra::{Point3, Vector3};

use crate::mesh::{Mesh, Triangle, Vertex, SKYBLUE};
use std::error::Error;
use std::fs::read_to_string;

pub struct LoadedMesh {
    verts: Vec<Vertex>,
    tris: Vec<Triangle>
}

impl LoadedMesh {
    pub fn from_file(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let mut loaded_mesh: Self = Self {verts: vec![], tris: vec![]};
        let mut verts: Vec<Point3<f32>> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut tris: Vec<Triangle> = vec![];

        for line in read_to_string(file_name)?.lines() {
            if line.len() < 2 {
                continue;
            }
            let line_type = &line[0..2];
            match line_type {
                "v " => {
                    let field: Vec<&str> = line.split(" ").collect();
                    verts.push(Point3::new(field[1].parse()?, field[2].parse()?, field[3].parse()?));
                }
                "vn" => {
                    let field: Vec<&str> = line.split(" ").collect();
                    normals.push(Vector3::new(field[1].parse()?, field[2].parse()?, field[3].parse()?));
                }
                "f " => {
                    
                    let field: Vec<&str> = line.split(" ").collect();
                    let t1: Vec<&str> = field[1].split("/").collect();
                    let t2: Vec<&str> = field[2].split("/").collect();
                    let t3: Vec<&str> = field[3].split("/").collect();
                    
                    tris.push(Triangle {
                        v1: t1.get(0)
                            .ok_or("Missing vertex index")?
                            .parse::<usize>()? - 1,
                        v2: t3.get(0)
                            .ok_or("Missing vertex index")?
                            .parse::<usize>()? - 1,
                        v3: t2.get(0)
                            .ok_or("Missing vertex index")?
                            .parse::<usize>()? - 1,
                    color: SKYBLUE});
                }
                _ => continue
            }
        }
        let mut vertices: Vec<Vertex> = verts.into_iter().map(|v| -> Vertex 
            {Vertex {position: v, normal: Vector3::zeros()}}).collect();

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

impl Mesh for LoadedMesh {
    fn tris(&self) -> &[Triangle] {
        &self.tris
    }

    fn verts(&self) -> &[Vertex] {
        &self.verts
    }
}