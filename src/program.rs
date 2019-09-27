use gl;
use std;
use std::ffi::{CString, CStr};

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, vert: &Shader, frag: &Shader) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        unsafe {
            gl.AttachShader(program_id, vert.id);
            gl.AttachShader(program_id, frag.id);
            gl.LinkProgram(program_id);

            let mut success = 1;
            gl.GetShaderiv(program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let msg = gl_err(gl, program_id);
                return Err(msg);
            }
            gl.DetachShader(program_id, vert.id);
            gl.DetachShader(program_id, frag.id);
        }

        Ok(Program { gl: gl.clone(), id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        gl: &gl::Gl,
        source: &str,
        shader_type: gl::types::GLenum
    ) -> Result<Shader, String> {
        let cstr_src = &CString::new(source).unwrap();
        shader_from_source(gl, cstr_src, shader_type)
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id)
        }
    }
}

fn gl_err(gl: &gl::Gl, id: gl::types::GLuint) -> String {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = create_whitespace_cstring_with_len(len as usize);

    unsafe {
        gl.GetShaderInfoLog(
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
    gl: &gl::Gl,
    source: &CStr,
    shader_type: gl::types::GLuint
) -> Result<Shader, String> {
    let id = unsafe { gl.CreateShader(shader_type) };

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let msg: String = gl_err(gl, id);
        Err(msg)
    } else {
        Ok(Shader { gl: gl.clone(), id })
    }
}

