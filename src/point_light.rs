use nalgebra::Vector3;
use gl::types::*;

use crate::shader::get_shader_location;

pub struct PointLight {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub strength: f32,
}
impl PointLight {
    pub fn new() -> PointLight {
        PointLight {
            position: Vector3::new(0.0, 0.0, 0.0),
            color: Vector3::new(0.5, 0.5, 0.5),
            strength: 1.0,
        }
    }

    pub fn link_shader(&self, program: u32, index: i32) {
        let light_position_loc = get_shader_location(program, &format!("pointLights[{index}].position"));
        let light_color_loc = get_shader_location(program, &format!("pointLights[{index}].color"));
        let light_strength_loc = get_shader_location(program, &format!("pointLights[{index}].strength"));

        unsafe {
            gl::Uniform3f(
                light_position_loc,
                self.position.x,
                self.position.y,
                self.position.z,
            );
            gl::Uniform3f(light_color_loc, self.color.x, self.color.y, self.color.z);
            gl::Uniform1f(light_strength_loc, self.strength);
        }
    }
}
