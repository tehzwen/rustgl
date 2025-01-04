use crate::shader::get_shader_location;
use nalgebra::{Const, Matrix4, Point, Point3, Vector3};
use gl::types::*;

pub struct Camera {
    pub position: Point<f32, 3>,
    target: Point<f32, 3>,
    up: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Point<f32, 3>, target: Point<f32, 3>, up: Vector3<f32>) -> Camera {
        Camera {
            position,
            target,
            up,
        }
    }

    pub fn default() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.5),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn link_shader(&self, program: u32) {
        // link the uniforms with the shader
        let position_loc = get_shader_location(program, "camera_position");
        let view_loc = get_shader_location(program, "view");

        unsafe {
            gl::Uniform3f(
                position_loc,
                self.position.x,
                self.position.y,
                self.position.z,
            );
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, self.view_matrix().as_ptr());
        }
    }

    pub fn look_at_target(&mut self, t: Point<f32, 3>) {
        self.target = t;
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        return Matrix4::look_at_rh(&self.position, &self.target, &self.up);
    }
}
