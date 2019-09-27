extern crate gl;
extern crate sdl2;

pub mod program;

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

    let _gl_context = window.gl_create_context().unwrap();
    let gl = 
        gl::Gl::load_with(|s| video_subsystem
                      .gl_get_proc_address(s) as *const std::os::raw::c_void
                     );

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut events = sdl.event_pump().unwrap();

    let vert_shader = program::Shader::from_vert_source(&gl, include_str!("vert.shdr")).unwrap();
    let frag_shader = program::Shader::from_frag_source(&gl, include_str!("frag.shdr")).unwrap();
    let program = program::Program::from_shaders(&gl, &vert_shader, &frag_shader).unwrap();

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
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
            );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl.EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    'main: loop {
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
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
