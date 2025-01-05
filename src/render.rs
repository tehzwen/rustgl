extern crate nalgebra as na;

use na::{Matrix4, UnitQuaternion, Vector3};
use nalgebra::{OVector, Point, Unit, Vector, Vector2};

use crate::{
    buffers::RenderBuffers,
    collision::{self, BoundingBox},
    material::Material,
    vertex::Vertex,
};

pub struct Model {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,

    centroid: Vector3<f32>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: UnitQuaternion::identity(),
            centroid: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn rotate(&mut self, axis: Unit<Vector3<f32>>, angle: f32) {
        // Create a quaternion representing the rotation
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);

        // Combine the current rotation with the new one
        self.rotation = rotation * self.rotation;
    }

    pub fn scale(&mut self, target: Vector3<f32>) {
        self.scale = target;
    }

    pub fn translate(&mut self, target: Vector3<f32>) {
        self.position = target;
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let mut model_matrix = Matrix4::identity();

        // negate the centroid so we can move back correctly
        // let neg_centroid = Vector3::new(-self.centroid.x, -self.centroid.y, -self.centroid.z);

        // Apply scale, rotation, and translation
        // model_matrix *= Matrix4::new_translation(&self.centroid);
        model_matrix *= Matrix4::new_nonuniform_scaling(&self.scale);
        model_matrix *= Matrix4::new_translation(&self.position);
        model_matrix *= self.rotation.to_homogeneous();

        // model_matrix *= Matrix4::new_translation(&neg_centroid);

        return model_matrix;
    }

    fn calculate_centroid(&mut self, vertices: &[Vertex]) {
        let mut center = Vector3::new(0.0, 0.0, 0.0);

        for vertex in vertices {
            center.x += vertex[0];
            center.y += vertex[1];
            center.z += vertex[2];
        }

        self.centroid = center.scale(1.0 / (vertices.len() as f32));
    }
}

pub struct Object {
    pub model: Model,
    pub buffers: RenderBuffers,
    pub material: Box<dyn Material>,
    pub vertices: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub uvs: Vec<Vector2<f32>>,
    pub bounding_box: collision::BoundingBox,
}

impl Object {
    pub fn new<'a>(
        m: Model,
        b: RenderBuffers,
        v: Vec<Vector3<f32>>,
        n: Vec<Vector3<f32>>,
        uv: Vec<Vector2<f32>>,
        material: Box<dyn Material>,
    ) -> Object {
        Object {
            model: m,
            buffers: b,
            vertices: v,
            normals: n,
            uvs: uv,
            material,
            bounding_box: BoundingBox::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn init(&mut self) {
        // initialize our buffers
        self.buffers.init(&self.vertices, &self.normals, &self.uvs);

        // calculate centroid
        // self.model.calculate_centroid(self.vertices);
        // etc etc

        self.bounding_box = collision::get_bounding_box(&self.vertices);
        println!("{:?}", self.bounding_box);
    }
}
