use gl::types::*;
use nalgebra::Vector3;

use crate::shader::get_shader_location;

pub trait Material {
    fn link_shader(&self, program: u32);
    fn toggle_map(&mut self, t: TextureType);
}

pub enum TextureType {
    DIFFUSE,
    NORMAL,
    ARM,
}

pub struct Texture {
    pub tex: Option<u32>,
    pub enabled: bool,
    pub scale: f32,
}

impl Texture {
    pub fn new() -> Texture {
        return Texture {
            tex: None,
            enabled: false,
            scale: 1.0,
        };
    }
}

pub struct Physical {
    pub albedo: Vector3<f32>,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub arm_texture: Texture,
}

impl Physical {
    pub fn new(a: Vector3<f32>, m: f32, r: f32, ao: f32) -> Physical {
        Physical {
            albedo: a,
            metallic: m,
            roughness: r,
            ao: ao,
            diffuse_texture: Texture::new(),
            normal_texture: Texture::new(),
            arm_texture: Texture::new(),
        }
    }

    pub fn default() -> Physical {
        Physical {
            albedo: Vector3::new(0.5, 0.5, 0.5),
            metallic: 0.5,
            roughness: 0.5,
            ao: 0.5,
            diffuse_texture: Texture::new(),
            normal_texture: Texture::new(),
            arm_texture: Texture::new(),
        }
    }
}

impl Material for Physical {
    fn link_shader(&self, program: u32) {
        // link the uniforms with the shader
        let albedo_loc = get_shader_location(program, "material.albedo");
        let metallic_loc = get_shader_location(program, "material.metallic");
        let roughness_loc = get_shader_location(program, "material.roughness");
        let ao_loc = get_shader_location(program, "material.ao");
        let diffuse_scaling_loc = get_shader_location(program, "material.diffuse_texture.scale");
        let normal_scaling_loc = get_shader_location(program, "material.normal_texture.scale");

        unsafe {
            gl::Uniform3f(albedo_loc, self.albedo.x, self.albedo.y, self.albedo.z);
            gl::Uniform1f(metallic_loc, self.metallic);
            gl::Uniform1f(roughness_loc, self.roughness);
            gl::Uniform1f(ao_loc, self.ao);
            gl::Uniform1f(diffuse_scaling_loc, self.diffuse_texture.scale);
            gl::Uniform1f(normal_scaling_loc, self.normal_texture.scale);

            gl::Uniform1i(
                get_shader_location(program, "material.diffuse_texture.enabled"),
                self.diffuse_texture.enabled as i32,
            );

            if let Some(diffuse) = self.diffuse_texture.tex {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, diffuse);
                let texture_loc = get_shader_location(program, "material.diffuse_texture.tex");
                gl::Uniform1i(texture_loc, 0);
            }

            gl::Uniform1i(
                get_shader_location(program, "material.normal_texture.enabled"),
                self.normal_texture.enabled as i32,
            );

            if let Some(normal) = self.normal_texture.tex {
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, normal);
                let texture_loc = get_shader_location(program, "material.normal_texture.tex");
                gl::Uniform1i(texture_loc, 1);
            }

            gl::Uniform1i(
                get_shader_location(program, "material.arm_texture.enabled"),
                self.arm_texture.enabled as i32,
            );

            if let Some(arm) = self.arm_texture.tex {
                gl::ActiveTexture(gl::TEXTURE2);
                gl::BindTexture(gl::TEXTURE_2D, arm);
                let texture_loc = get_shader_location(program, "material.arm_texture.tex");
                gl::Uniform1i(texture_loc, 2);
            }
        }
    }

    fn toggle_map(&mut self, t: TextureType) {
        // need to add a bool to the struct itself so we can easily toggle
        match t {
            TextureType::DIFFUSE => {
                self.diffuse_texture.enabled = !self.diffuse_texture.enabled;
            }
            TextureType::NORMAL => {
                self.normal_texture.enabled = !self.normal_texture.enabled;
            }
            TextureType::ARM => {
                self.arm_texture.enabled = !self.arm_texture.enabled;
            }
        }
    }
}
