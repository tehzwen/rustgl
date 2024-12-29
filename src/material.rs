use nalgebra::Vector3;
use ogl33::{glUniform1f, glUniform3f};

use crate::shader::get_shader_location;

pub trait Material {
    fn link_shader(&self, program: u32);
}

pub struct Physical {
    pub albedo: Vector3<f32>,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,
}

impl Physical {
    pub fn new(a: Vector3<f32>, m: f32, r: f32, ao: f32) -> Physical {
        Physical {
            albedo: a,
            metallic: m,
            roughness: r,
            ao: ao,
        }
    }

    pub fn default() -> Physical {
        Physical {
            albedo: Vector3::new(0.5, 0.5, 0.5),
            metallic: 0.5,
            roughness: 0.5,
            ao: 0.5,
        }
    }
}

impl Material for Physical {
    fn link_shader(&self, program: u32) {
        // link the uniforms with the shader
        let albedo_loc = get_shader_location(program, "albedo");
        let metallic_loc = get_shader_location(program, "metallic");
        let roughness_loc = get_shader_location(program, "roughness");
        let ao_loc = get_shader_location(program, "ao");

        unsafe {
            glUniform3f(albedo_loc, self.albedo.x, self.albedo.y, self.albedo.z);
            glUniform1f(metallic_loc, self.metallic);
            glUniform1f(roughness_loc, self.roughness);
            glUniform1f(ao_loc, self.ao);
        }
    }
}
