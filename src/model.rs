use nalgebra::*;

use super::vertex;
use super::render::buffer;
use super::collide;

// each model has one VAO, and one VBO for position and color

pub struct Model {
    pub isometry: Isometry3<f32>,
    aabb: collide::AABB,
    num_verts: usize,
    _position_vbo: buffer::ArrayBuffer,
    vao: u32,
}

impl Model {
    pub fn new(faces: &[f32], pos: Point3<f32>) -> Model {
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
        let aabb = collide::AABB::new(&faces);

        Model { 
            isometry: Isometry3::from_parts(
                          Translation3::new(pos.x, pos.y, pos.z),
                          UnitQuaternion::identity()
                      ),
            aabb,
            num_verts: (faces.len() / 3),
            _position_vbo: vbo,
            vao,
        }
    }

    pub fn collides_with(&self, pos: Point3<f32>) -> bool {
        let trans = self.isometry;
        let p1 = trans * self.aabb.left_top_front;
        let p2 = trans * self.aabb.right_bottom_back;
        let aabb = collide::AABB { left_top_front: p1, right_bottom_back: p2 };
        aabb.is_in_aabb(pos)
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }

        vertex::draw_arrays(self.num_verts);
    }

    pub fn floor_model(x_dim: usize, z_dim: usize) -> Model {
        let x = x_dim as f32;
        let z = z_dim as f32;
        let floor_verts: Vec<f32> = vec![
            0.0, 0.0, 0.0,
            x, 0.0, 0.0,
            0.0, 0.0, z,

            x, 0.0, z,
            x, 0.0, 0.0,
            0.0, 0.0, z,
        ];
        Model::new(&floor_verts, Point3::new(0.0, 0.0, 0.0))
    }

    pub fn test_cube_model(pos: Point3<f32>) -> Model {
        let cube_verts: Vec<f32> = vec![
            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,

            -1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5, -1.5,  1.5,

            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5, -1.5,
            -1.5, -1.5,  1.5,
            -1.5,  1.5,  1.5,

            1.5,  1.5,  1.5,
            1.5,  1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5,  1.5,  1.5,

            -1.5, -1.5, -1.5,
            1.5, -1.5, -1.5,
            1.5, -1.5,  1.5,
            1.5, -1.5,  1.5,
            -1.5, -1.5,  1.5,
            -1.5, -1.5, -1.5,

            -1.5,  1.5, -1.5,
            1.5,  1.5, -1.5,
            1.5,  1.5,  1.5,
            1.5,  1.5,  1.5,
            -1.5,  1.5,  1.5,
            -1.5,  1.5, -1.5,
        ];
        Model::new(&cube_verts, pos)
    }
}
