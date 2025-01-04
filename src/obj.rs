use core::num;
use nalgebra::{Vector2, Vector3};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct ObjData {
    pub vertices: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub tex_coords: Vec<Vector2<f32>>,
    pub indices: Vec<u32>,
}

impl ObjData {
    pub fn new() -> Self {
        ObjData {
            vertices: Vec::new(),
            normals: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
        }
    }
}

pub fn parse_obj(file_path: &str) -> Result<ObjData, io::Error> {
    let loaded_file = tobj::load_obj(file_path, &tobj::GPU_LOAD_OPTIONS);
    assert!(loaded_file.is_ok());

    let (models, _materials) = loaded_file.expect("Failed to load OBJ file");

    let mut vertices: Vec<Vector3<f32>> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();
    let mut uvs: Vec<Vector2<f32>> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        println!("model[{}].name = '{}'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        for idx in &mesh.indices {
            let vertex_idx = *idx as usize;

            // Push the vertex positions
            vertices.push(Vector3::new(
                mesh.positions[3 * vertex_idx],
                mesh.positions[3 * vertex_idx + 1],
                mesh.positions[3 * vertex_idx + 2],
            ));

            // Push the normals, if available
            if !mesh.normals.is_empty() {
                normals.push(Vector3::new(
                    mesh.normals[3 * vertex_idx],
                    mesh.normals[3 * vertex_idx + 1],
                    mesh.normals[3 * vertex_idx + 2],
                ));
            }

            // Push texture coordinates, if available
            if !mesh.texcoords.is_empty() {
                uvs.push(Vector2::new(
                    mesh.texcoords[2 * vertex_idx],
                    mesh.texcoords[2 * vertex_idx + 1],
                ));
            }

            // Store indices for rendering
            indices.push(*idx);
        }
    }

    Ok(ObjData {
        vertices,
        normals,
        tex_coords: uvs,
        indices,
    })
}
