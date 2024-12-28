use crate::vertex::Vertex;
use ogl33::*;

pub struct RenderBuffers {
    pub vao: u32,
    pub vbo: u32,
    pub size: i32,
}

impl RenderBuffers {
    pub fn new() -> RenderBuffers {
        RenderBuffers { vao: 0, vbo: 0, size: 0 }
    }

    pub fn init(&mut self, vertices: &[Vertex]) {
        unsafe {
            glGenVertexArrays(1, &mut self.vao);
            assert_ne!(self.vao, 0);
            glBindVertexArray(self.vao);

            glGenBuffers(1, &mut self.vbo);
            assert_ne!(self.vbo, 0);
            glBindBuffer(GL_ARRAY_BUFFER, self.vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (vertices.len() * size_of::<Vertex>()) as isize,
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );

            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            glEnableVertexAttribArray(0);
            self.size = vertices.len().try_into().expect("failed to cast vertices size to i32");
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
