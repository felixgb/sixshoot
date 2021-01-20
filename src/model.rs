use glm::{Mat4x4, Vec3};
use super::collide::AABB;
use super::glm_utils;
use super::buffer;
use super::vertex;
use super::texture;

pub struct Model {
    pub translation: Mat4x4,
    aabb: AABB,
    num_verts: usize,
    _position_vbo: buffer::ArrayBuffer,
    vao: u32,
    texture_id: gl::types::GLuint,
}

impl Model {
    pub fn new(faces: &[f32], pos: Vec3, texture_id: gl::types::GLuint) -> Model {
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
        let aabb = AABB::new(&faces);

        Model { 
            translation: glm::translation(&pos),
            aabb,
            num_verts: (faces.len() / 3),
            _position_vbo: vbo,
            vao,
            texture_id
        }
    }

    pub fn collides_with(&self, pos: Vec3) -> bool {
        let h1 = glm_utils::translate_pos(&self.translation, &self.aabb.left_top_front);
        let h2 = glm_utils::translate_pos(&self.translation, &self.aabb.right_bottom_back);
        let aabb = AABB {
            left_top_front: h1,
            right_bottom_back: h2,
        };
        aabb.is_in_aabb(pos)
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }

        vertex::draw_arrays(self.num_verts);
    }

    // pub fn floor_model(x_dim: usize, z_dim: usize, h: f32) -> Model {
    //     let x = x_dim as f32;
    //     let z = z_dim as f32;
    //     let floor_verts: Vec<f32> = vec![
    //         0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    //         x, 0.0, 0.0, 0.0, 1.0, 0.0,
    //         0.0, 0.0, z, 0.0, 1.0, 0.0,

    //         x, 0.0, z, 0.0, 1.0, 0.0,
    //         x, 0.0, 0.0, 0.0, 1.0, 0.0,
    //         0.0, 0.0, z, 0.0, 1.0, 0.0,
    //     ];
    //     Model::new(&floor_verts, glm::vec3(0.0, h, 0.0))
    // }

    pub fn cube_texture() -> gl::types::GLuint {
        texture::prepare_textures("assets/container.jpg")
    }

    pub fn test_cube_model(pos: Vec3, texture_location: gl::types::GLuint) -> Model {
        let cube_verts: Vec<f32> = vec![
            -2.0, -2.0, -2.0,  0.0,  0.0, -1.0, 0.0, 0.0,
            2.0, -2.0, -2.0,  0.0,  0.0, -1.0, 1.0, 0.0,
            2.0,  2.0, -2.0,  0.0,  0.0, -1.0, 1.0, 1.0,
            2.0,  2.0, -2.0,  0.0,  0.0, -1.0, 1.0, 1.0,
            -2.0,  2.0, -2.0,  0.0,  0.0, -1.0, 0.0, 1.0,
            -2.0, -2.0, -2.0,  0.0,  0.0, -1.0, 0.0, 0.0,

            -2.0, -2.0,  2.0,  0.0,  0.0, 1.0, 0.0, 0.0,
            2.0, -2.0,  2.0,  0.0,  0.0, 1.0, 1.0, 0.0,
            2.0,  2.0,  2.0,  0.0,  0.0, 1.0, 1.0, 1.0,
            2.0,  2.0,  2.0,  0.0,  0.0, 1.0, 1.0, 1.0,
            -2.0,  2.0,  2.0,  0.0,  0.0, 1.0, 0.0, 1.0,
            -2.0, -2.0,  2.0,  0.0,  0.0, 1.0, 0.0, 0.0,

            -2.0,  2.0,  2.0, -1.0,  0.0,  0.0, 1.0, 0.0,
            -2.0,  2.0, -2.0, -1.0,  0.0,  0.0, 1.0, 1.0,
            -2.0, -2.0, -2.0, -1.0,  0.0,  0.0, 0.0, 1.0,
            -2.0, -2.0, -2.0, -1.0,  0.0,  0.0, 0.0, 1.0,
            -2.0, -2.0,  2.0, -1.0,  0.0,  0.0, 0.0, 0.0,
            -2.0,  2.0,  2.0, -1.0,  0.0,  0.0, 1.0, 0.0,

            2.0,  2.0,  2.0,  1.0,  0.0,  0.0, 1.0, 0.0,
            2.0,  2.0, -2.0,  1.0,  0.0,  0.0, 1.0, 1.0,
            2.0, -2.0, -2.0,  1.0,  0.0,  0.0, 0.0, 1.0,
            2.0, -2.0, -2.0,  1.0,  0.0,  0.0, 0.0, 1.0,
            2.0, -2.0,  2.0,  1.0,  0.0,  0.0, 0.0, 0.0,
            2.0,  2.0,  2.0,  1.0,  0.0,  0.0, 1.0, 0.0,

            -2.0, -2.0, -2.0,  0.0, -1.0,  0.0, 0.0, 1.0,
            2.0, -2.0, -2.0,  0.0, -1.0,  0.0, 1.0, 1.0,
            2.0, -2.0,  2.0,  0.0, -1.0,  0.0, 1.0, 0.0,
            2.0, -2.0,  2.0,  0.0, -1.0,  0.0, 1.0, 0.0,
            -2.0, -2.0,  2.0,  0.0, -1.0,  0.0, 0.0, 0.0,
            -2.0, -2.0, -2.0,  0.0, -1.0,  0.0, 0.0, 1.0,

            -2.0,  2.0, -2.0,  0.0,  1.0,  0.0, 0.0, 1.0,
            2.0,  2.0, -2.0,  0.0,  1.0,  0.0, 1.0, 1.0,
            2.0,  2.0,  2.0,  0.0,  1.0,  0.0, 1.0, 0.0,
            2.0,  2.0,  2.0,  0.0,  1.0,  0.0, 1.0, 0.0,
            -2.0,  2.0,  2.0,  0.0,  1.0,  0.0, 0.0, 0.0,
            -2.0,  2.0, -2.0,  0.0,  1.0,  0.0, 0.0, 1.0
        ];
        Model::new(&cube_verts, pos, texture_location)
    }
}

