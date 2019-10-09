use nalgebra::*;

use super::vertex;
use super::render::buffer;

// each model has one VAO, and one VBO for position and color

pub struct Model {
    pub isometry: Isometry3<f32>,
    num_verts: usize,
    _position_vbo: buffer::ArrayBuffer,
    vao: u32,
}

impl Model {
    pub fn new(faces: &Vec<f32>, pos: Point3<f32>) -> Model {
        let vbo = buffer::ArrayBuffer::new();

        vbo.bind();
        vbo.static_draw_data(faces);
        vbo.unbind();

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            vbo.bind();
            vertex::vertex_attrib_pointers();
            vbo.unbind();

            gl::BindVertexArray(0);
        }

        Model { 
            isometry: Isometry3::from_parts(
                          Translation3::new(pos.x, pos.y, pos.z),
                          UnitQuaternion::identity()
                      ),
            num_verts: (faces.len() / 3),
            _position_vbo: vbo,
            vao: vao,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }

        vertex::draw_arrays(self.num_verts);
    }

    pub fn test_cube_model(pos: Point3<f32>) -> Model {
        let cube_verts: Vec<f32> = vec![
            -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5,  0.5, -0.5,

            0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,

            -0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5, -0.5,  0.5,

            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5,  0.5,
            -0.5,  0.5,  0.5,

            0.5,  0.5,  0.5,
            0.5,  0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,

            -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
            -0.5, -0.5,  0.5,
            -0.5, -0.5, -0.5,

            -0.5,  0.5, -0.5,
            0.5,  0.5, -0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
        ];
        Model::new(&cube_verts, pos)
    }
}
