use crate::mesh::{
    Mesh, Triangle, Vertex, BEIGE, BLUE, DARKBLUE, GOLD, GRAY, GREEN, LIME, ORANGE, PINK, PURPLE,
    RED, YELLOW,
};
use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct CubeMesh {
    verts: Vec<Vertex>,
    tris: Vec<Triangle>,
}

impl CubeMesh {
    pub fn new() -> Self {
        // Initial cube positions
        let mut verts: Vec<Vertex> = vec![
            Vertex {
                position: Point3::new(1.0, -1.0, 1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(1.0, -1.0, -1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(-1.0, -1.0, -1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(-1.0, -1.0, 1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(1.0, 1.0, 1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(1.0, 1.0, -1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(-1.0, 1.0, -1.0),
                normal: Vector3::zeros(),
            },
            Vertex {
                position: Point3::new(-1.0, 1.0, 1.0),
                normal: Vector3::zeros(),
            },
        ];

        let tris: Vec<Triangle> = vec![
            Triangle {
                v1: 0,
                v2: 1,
                v3: 2,
                color: RED,
            },
            Triangle {
                v1: 0,
                v2: 2,
                v3: 3,
                color: GREEN,
            },
            Triangle {
                v1: 4,
                v2: 6,
                v3: 5,
                color: BLUE,
            },
            Triangle {
                v1: 4,
                v2: 7,
                v3: 6,
                color: ORANGE,
            },
            Triangle {
                v1: 0,
                v2: 3,
                v3: 7,
                color: PINK,
            },
            Triangle {
                v1: 0,
                v2: 7,
                v3: 4,
                color: DARKBLUE,
            },
            Triangle {
                v1: 1,
                v2: 5,
                v3: 6,
                color: LIME,
            },
            Triangle {
                v1: 1,
                v2: 6,
                v3: 2,
                color: PURPLE,
            },
            Triangle {
                v1: 0,
                v2: 4,
                v3: 5,
                color: BEIGE,
            },
            Triangle {
                v1: 0,
                v2: 5,
                v3: 1,
                color: GOLD,
            },
            Triangle {
                v1: 3,
                v2: 2,
                v3: 6,
                color: GRAY,
            },
            Triangle {
                v1: 3,
                v2: 6,
                v3: 7,
                color: YELLOW,
            },
        ];

        // Compute vertex normals by averaging face normals
        for tri in &tris {
            let i0 = tri.v1 as usize;
            let i1 = tri.v2 as usize;
            let i2 = tri.v3 as usize;

            let v0 = verts[i0].position;
            let v1 = verts[i1].position;
            let v2 = verts[i2].position;

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = edge1.cross(&edge2).normalize();

            verts[i0].normal += face_normal;
            verts[i1].normal += face_normal;
            verts[i2].normal += face_normal;
        }

        // Normalize accumulated vertex normals
        for vert in &mut verts {
            vert.normal = vert.normal.normalize();
        }

        Self { verts, tris }
    }
}

impl Mesh for CubeMesh {
    fn tris(&self) -> &[Triangle] {
        &self.tris
    }

    fn verts(&self) -> &[Vertex] {
        &self.verts
    }
}
