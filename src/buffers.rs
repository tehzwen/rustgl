use crate::vertex::Vertex;
use nalgebra::Vector3;
use gl::types::*;


pub struct RenderBuffers {
    pub vao: u32,
    pub vbo_positions: u32,
    pub vbo_normals: u32, // Separate buffer for normals
    pub size: i32,
}

impl RenderBuffers {
    pub fn new() -> RenderBuffers {
        RenderBuffers {
            vao: 0,
            vbo_positions: 0,
            vbo_normals: 0,
            size: 0,
        }
    }

    pub fn init(&mut self, vertices: &Vec<Vector3<f32>>, normals: &Vec<Vector3<f32>>) {
        unsafe {
            // Generate and bind the VAO
            gl::GenVertexArrays(1, &mut self.vao);
            assert_ne!(self.vao, 0);
            gl::BindVertexArray(self.vao);

            // Generate and bind the position buffer
            gl::GenBuffers(1, &mut self.vbo_positions);
            assert_ne!(self.vbo_positions, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_positions);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vector3<f32>>()) as isize,
                vertices.as_slice().as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Generate and bind the normal buffer
            gl::GenBuffers(1, &mut self.vbo_normals);
            assert_ne!(self.vbo_normals, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_normals);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (normals.len() * std::mem::size_of::<Vector3<f32>>()) as isize,
                normals.as_slice().as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Normal attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);

            self.size = vertices
                .len()
                .try_into()
                .expect("failed to cast vertices size to i32");

            // Unbind the VAO
            gl::BindVertexArray(0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
