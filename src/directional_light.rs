use nalgebra::Vector3;

use crate::shader::get_shader_location;

pub struct DirectionalLight {
    pub color: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl DirectionalLight {
    pub fn new() -> DirectionalLight {
        DirectionalLight {
            direction: Vector3::new(0.0, 0.0, 1.0),
            color: Vector3::new(0.5, 0.5, 0.5),
        }
    }

    pub fn link_shader(&self, program: u32) {
        let light_dir_loc = get_shader_location(program, "dirLight.direction");
        let light_color_loc = get_shader_location(program, &format!("dirLight.color"));

        unsafe {
            gl::Uniform3f(
                light_dir_loc,
                self.direction.x,
                self.direction.y,
                self.direction.z,
            );
            gl::Uniform3f(light_color_loc, self.color.x, self.color.y, self.color.z);
        }
    }
}
