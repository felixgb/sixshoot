extern crate gl;
extern crate sdl2;

use std::ffi::{CString, CStr};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("HA HA", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = 
        gl::load_with(|s| video_subsystem
                      .gl_get_proc_address(s) as *const std::os::raw::c_void
                     );

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.6, 0.6, 0.6, 1.0);
    }

    let mut events = sdl.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(
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
