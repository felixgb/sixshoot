extern crate nalgebra;
extern crate gl;
extern crate sdl2;

pub mod render;
pub mod resources;
pub mod vertex;

use nalgebra::Rotation;
use sdl2::keyboard::Keycode;
use render::buffer;
use std::ffi::{CString, CStr};

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

fn create_window(video_subsystem: &sdl2::VideoSubsystem) -> sdl2::video::Window {
    video_subsystem
        .window("HA HA", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap()
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = create_window(&video_subsystem);

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let _gl_context = window.gl_create_context().unwrap();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
    }

    let vert_shader = render::Shader::from_vert_source(include_str!("vert.shdr")).unwrap();
    let frag_shader = render::Shader::from_frag_source(include_str!("frag.shdr")).unwrap();
    let program = render::Program::from_shaders(&vert_shader, &frag_shader).unwrap();

    program.set_used();

    unsafe {
        let transform = gl::GetUniformLocation(
            program.id,
            c_str!("transform").as_ptr()
        );
        let rot = Rotation::from_euler_angles(2.0, 0.0, 0.0);
        gl::UniformTransformMatrix4fv(program.id, 1, GL_FALSE, rot.as_slice())
    };

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,
        0.0, 0.0, 1.0,
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    let vbo = buffer::ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);

        vbo.bind();
        vertex::vertex_attrib_pointers();
        vbo.unbind();

        gl::BindVertexArray(0);
    }

    let mut events = sdl.event_pump().unwrap();

    'main: loop {
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                6 // number of indices to be rendered
            );
        }
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Q), .. } => break 'main,
                e => {
                    println!("{:?}", e);
                },
            }
        }

        window.gl_swap_window();
    }

}
