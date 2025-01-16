// basically like an object, only need one set of buffers, but then more data for each individual particle? (lifespan, world pos etc?)

use crate::{
    shader::{self, Shader},
    vertex::Vertex,
};
use core::num;

use nalgebra::Vector3;
use rand::Rng;

use crate::buffers::RenderBuffers;

pub struct Particle {
    pub lifespan: f32,
    pub velocity: Vector3<f32>,
    pub position: Vector3<f32>,
}

impl Particle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Particle {
            lifespan: rng.gen_range(0.0..5.0),
            position: Vector3::zeros(),
            velocity: Vector3::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
            ),
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
        self.lifespan -= delta_time;
        if self.lifespan <= 0.0 {
            *self = Particle::new();
        }
    }
}

pub struct ParticleGenerator {
    pub vao_buffer: u32,
    pub position_buffer: u32,
    pub num_particles: i32,
    pub particles: Vec<Particle>,
    pub shader_program: u32,
}

impl ParticleGenerator {
    pub fn new(num_particles: i32) -> Self {
        let mut particles = Vec::new();

        for n in 0..num_particles {
            particles.push(Particle::new());
        }

        // what does a quad look like
        let quad_vertices: [f32; 12] = [
            -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0,
        ];
        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        let mut vbo_indices: u32 = 0;
        let mut vbo_instance_data: u32 = 0;
        let mut shader_program: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (quad_vertices.len() * size_of::<f32>()) as isize,
                quad_vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            let stride: i32 = (3 * size_of::<f32>())
                .try_into()
                .expect("Stride calculation overflowed!");

            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut vbo_indices);

            assert_ne!(vbo_indices, 0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_indices);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_slice().as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // instance data
            gl::GenBuffers(1, &mut vbo_instance_data);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_instance_data);
            let instance_data: Vec<Vector3<f32>> = particles.iter().map(|p| p.position).collect();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (instance_data.len() * size_of::<Vector3<f32>>()) as isize,
                instance_data.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            );
            let instance_stride: i32 = size_of::<Vector3<f32>>()
                .try_into()
                .expect("Instance Stride calculation overflowed!");
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                instance_stride,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribDivisor(1, 1);
            shader_program = Shader::new("particle".to_string()).program;
        }

        // setup buffers here for particles
        ParticleGenerator {
            vao_buffer: vao,
            position_buffer: vbo_instance_data,
            num_particles,
            particles,
            shader_program,
        }
    }

    pub fn update(&mut self) {
        for mut particle in &mut self.particles {
            particle.update(0.0005);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer);
            let instance_data: Vec<Vector3<f32>> =
                self.particles.iter().map(|p| p.position).collect();
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (instance_data.len() * size_of::<Vector3<f32>>()) as isize,
                instance_data.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn render(
        &self,
        model_view_matrix: &nalgebra::Matrix4<f32>,
        projection_matrix: &nalgebra::Matrix4<f32>,
        particle_size: f32,
    ) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
            gl::UseProgram(self.shader_program);

            // Set uniform variables for model-view and projection matrices
            let model_view_loc = shader::get_shader_location(self.shader_program, "viewMatrix");
            gl::UniformMatrix4fv(model_view_loc, 1, gl::FALSE, model_view_matrix.as_ptr());

            let projection_loc =
                shader::get_shader_location(self.shader_program, "projectionMatrix");
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_matrix.as_ptr());

            // Bind VAO and draw
            gl::BindVertexArray(self.vao_buffer);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                self.num_particles as i32,
            );
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                panic!("gl::DrawElementsInstanced error: {}", error);
            }
            gl::Disable(gl::BLEND);
        }
    }
}
