use nalgebra::Vector3;
use ogl33::{glUniform1f, glUniform3f};

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

    pub fn link_shader(&self, program: u32) {
        let light_position_loc = get_shader_location(program, "light.position");
        let light_color_loc = get_shader_location(program, "light.color");
        let light_strength_loc = get_shader_location(program, "light.strength");

        unsafe {
            glUniform3f(
                light_position_loc,
                self.position.x,
                self.position.y,
                self.position.z,
            );
            glUniform3f(light_color_loc, self.color.x, self.color.y, self.color.z);
            glUniform1f(light_strength_loc, self.strength);
        }
    }
}
