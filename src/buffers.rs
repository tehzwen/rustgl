use crate::vertex::Vertex;
use nalgebra::Vector3;
use ogl33::*;
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
            glGenVertexArrays(1, &mut self.vao);
            assert_ne!(self.vao, 0);
            glBindVertexArray(self.vao);

            // Generate and bind the position buffer
            glGenBuffers(1, &mut self.vbo_positions);
            assert_ne!(self.vbo_positions, 0);
            glBindBuffer(GL_ARRAY_BUFFER, self.vbo_positions);

            glBufferData(
                GL_ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vector3<f32>>()) as isize,
                vertices.as_slice().as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            // Position attribute
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            glEnableVertexAttribArray(0);

            // Generate and bind the normal buffer
            glGenBuffers(1, &mut self.vbo_normals);
            assert_ne!(self.vbo_normals, 0);
            glBindBuffer(GL_ARRAY_BUFFER, self.vbo_normals);
            glBufferData(
                GL_ARRAY_BUFFER,
                (normals.len() * std::mem::size_of::<Vector3<f32>>()) as isize,
                normals.as_slice().as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            // Normal attribute
            glVertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            glEnableVertexAttribArray(1);

            self.size = vertices
                .len()
                .try_into()
                .expect("failed to cast vertices size to i32");

            // Unbind the VAO
            glBindVertexArray(0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glBindVertexArray(0);
        }
    }
}
