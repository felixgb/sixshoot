use nalgebra::Point

use super::render::buffer;
use super::vertex;

pub struct Light {
}

impl Light {
    pub fn new(faces: &:[f32]) -> Light {
        let vbo = buffer::ArrayBuffer::new();
        
        vbo.bind();
        vbo.static_draw_data(faces);
        vbo.unbind();

        let mut vao: gl::types::GLuint = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        vertex::vertex_attrib_pointers();
    }
}
