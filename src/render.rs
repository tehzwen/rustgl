extern crate nalgebra as na;

use na::{Matrix4, UnitQuaternion, Vector3};
use nalgebra::{OVector, Unit};

use crate::{buffers::RenderBuffers, vertex::Vertex};

pub struct Model {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>, // Using a quaternion for rotation
}

impl Model {
    pub fn new() -> Model {
        Model {
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.5, 0.1, 1.0),
            rotation: UnitQuaternion::identity(),
        }
    }

    pub fn rotate(&mut self, axis: Unit<Vector3<f32>>, angle: f32) {
        // Create a quaternion representing the rotation
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);

        // Combine the current rotation with the new one
        self.rotation = rotation * self.rotation;
    }

    pub fn translate(&mut self, target: Vector3<f32>) {
        self.position = target;
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let mut model_matrix = Matrix4::identity();

        // Apply scale, rotation, and translation
        model_matrix = model_matrix * Matrix4::new_scaling(self.scale.x);
        model_matrix = model_matrix * Matrix4::new_scaling(self.scale.y);
        model_matrix = model_matrix * Matrix4::new_scaling(self.scale.z);
        model_matrix = model_matrix * self.rotation.to_homogeneous(); // Convert quaternion to a homogeneous rotation matrix
        model_matrix = model_matrix * Matrix4::new_translation(&self.position);

        model_matrix
    }
}

pub struct Object<'a> {
    pub model: Model,
    pub buffers: RenderBuffers,
    pub vertices: &'a [Vertex],
}

impl Object<'_> {
    pub fn new(m: Model, b: RenderBuffers, v: &[Vertex]) -> Object {
        Object {
            model: m,
            buffers: b,
            vertices: v,
        }
    }
}
