extern crate gl;
extern crate sdl2;

pub mod render;
pub mod resources;
pub mod vertex;

use sdl2::keyboard::Keycode;

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

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
            );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        vertex::vertex_attrib_pointers();

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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
