pub mod program_load;

use gl;
use std;
use std::ffi::{CString, CStr};

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        source: &CStr,
        shader_type: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, shader_type)?;
        Ok(id)
    }
}

fn gl_shader_err(id: gl::types::GLuint) -> String {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = create_whitespace_cstring_with_len(len as usize);

    unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
            );
    }

    return error.to_string_lossy().into_owned();
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn shader_from_source(
    source: &CStr,
    shader_type: gl::types::GLuint
) -> Result<Shader, String> {
    let id = unsafe { gl::CreateShader(shader_type) };

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let msg: String = gl_shader_err(id);
        Err(msg)
    } else {
        Ok(Shader { id })
    }
}

