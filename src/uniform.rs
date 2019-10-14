use gl;
use glm::{Mat4x4, Vec3};
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum UniformError {
    UniformNotFound(String),
}

type Result<T> = std::result::Result<T, UniformError>;

pub struct Uniform {
    id: gl::types::GLint,
}

impl Uniform {

    pub fn get_uniform_location(
        program_id: u32,
        uniform_name: &str
    ) -> Result<Uniform> {
        let transform_c_str = &CString::new(uniform_name).unwrap();
        let location = unsafe {
            gl::GetUniformLocation(
                program_id,
                transform_c_str.as_ptr()
            )
        };

        match location {
            -1 => Err(UniformError::UniformNotFound(uniform_name.to_string())),
            id => Ok(Uniform{ id })
        }
    }

    pub fn set_uniform_matrix4fv(&self, value: &Mat4x4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.id,
                1,
                gl::FALSE,
                value.as_slice().as_ptr() as *const f32
            );
        }
    }

    pub fn set_uniform_vec3(&self, value: &Vec3) {
        unsafe {
            gl::Uniform3fv(
                self.id,
                1,
                value.as_slice().as_ptr() as *const f32
            );
        }
    }
}
