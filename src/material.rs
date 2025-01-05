use gl::types::*;
use nalgebra::Vector3;

use crate::shader::get_shader_location;

pub trait Material {
    fn link_shader(&self, program: u32);
    fn diffuse_texture(&self) -> Option<u32>;
    fn normal_texture(&self) -> Option<u32>;
}

pub struct Physical {
    pub albedo: Vector3<f32>,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,
    pub diffuse_texture: Option<u32>,
    pub diffuse_texture_scale: f32,
    pub normal_texture: Option<u32>,
    pub normal_texture_scale: f32,
}

impl Physical {
    pub fn new(a: Vector3<f32>, m: f32, r: f32, ao: f32) -> Physical {
        Physical {
            albedo: a,
            metallic: m,
            roughness: r,
            ao: ao,
            diffuse_texture: None,
            diffuse_texture_scale: 1.0,
            normal_texture: None,
            normal_texture_scale: 1.0,
        }
    }

    pub fn default() -> Physical {
        Physical {
            albedo: Vector3::new(0.5, 0.5, 0.5),
            metallic: 0.5,
            roughness: 0.5,
            ao: 0.5,
            diffuse_texture: None,
            diffuse_texture_scale: 1.0,
            normal_texture: None,
            normal_texture_scale: 1.0,
        }
    }
    pub fn set_diffuse_texture(&mut self, tex: u32) {
        self.diffuse_texture = Some(tex);
    }
}

impl Material for Physical {
    fn link_shader(&self, program: u32) {
        // link the uniforms with the shader
        let albedo_loc = get_shader_location(program, "albedo");
        let metallic_loc = get_shader_location(program, "metallic");
        let roughness_loc = get_shader_location(program, "roughness");
        let ao_loc = get_shader_location(program, "ao");
        let diffuse_scaling_loc = get_shader_location(program, "diffuse_texture_scale");
        let normal_scaling_loc = get_shader_location(program, "normal_texture_scale");

        unsafe {
            gl::Uniform3f(albedo_loc, self.albedo.x, self.albedo.y, self.albedo.z);
            gl::Uniform1f(metallic_loc, self.metallic);
            gl::Uniform1f(roughness_loc, self.roughness);
            gl::Uniform1f(ao_loc, self.ao);
            gl::Uniform1f(diffuse_scaling_loc, self.diffuse_texture_scale);
            gl::Uniform1f(normal_scaling_loc, self.normal_texture_scale);

            if let Some(diffuse) = self.diffuse_texture {
                gl::Uniform1i(get_shader_location(program, "enable_diffuse_texture"), 1);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, diffuse);
                let texture_loc = get_shader_location(program, "diffuse_texture");
                gl::Uniform1i(texture_loc, 0);
            } else {
                gl::Uniform1i(get_shader_location(program, "enable_diffuse_texture"), 0);
            }

            if let Some(normal) = self.normal_texture {
                gl::Uniform1i(get_shader_location(program, "enable_normal_texture"), 1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, normal);
                let texture_loc = get_shader_location(program, "normal_texture");
                gl::Uniform1i(texture_loc, 1);
            } else {
                gl::Uniform1i(get_shader_location(program, "enable_normal_texture"), 0);
            }
        }
    }

    fn diffuse_texture(&self) -> Option<u32> {
        return self.diffuse_texture;
    }

    fn normal_texture(&self) -> Option<u32> {
        return self.normal_texture;
    }
}
