extern crate nalgebra;
extern crate gl;
extern crate sdl2;

pub mod render;
pub mod obj;
pub mod vertex;
pub mod uniform;
pub mod model;

use nalgebra::*;
use sdl2::keyboard::Keycode;
use render::buffer;

fn create_window(video_subsystem: &sdl2::VideoSubsystem) -> sdl2::video::Window {
    video_subsystem
        .window("HA HA", 1920, 1080)
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
        gl::Viewport(0, 0, 1920, 1080);
    }

    let vert_shader = render::Shader::from_vert_source(include_str!("vert.shdr")).unwrap();
    let frag_shader = render::Shader::from_frag_source(include_str!("frag.shdr")).unwrap();
    let program = render::Program::from_shaders(&vert_shader, &frag_shader).unwrap();

    program.set_used();

    // let vertices = obj::read_lines().unwrap().compute_faces();

    let cube = model::Model::test_cube_model();

    let vbo = buffer::ArrayBuffer::new();

    vbo.bind();
    vbo.static_draw_data(&cube.faces);
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
    let mut iter = 0.0;

    let transform = uniform::Uniform::get_uniform_location(program.id, "transform").unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let projection = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);

    'main: loop {
        unsafe {
            gl::BindVertexArray(vao);
        }
        vertex::draw_arrays(&cube.faces);

        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Q), .. } => break 'main,
                e => {
                    println!("{:?}", e);
                },
            }
        }

        let rot = Rotation3::from_euler_angles(-1.57, 1.57 + iter, 0.0);
        let model = rot.to_homogeneous() * Isometry3::identity().to_homogeneous();

        let camera_pos = Point3::new(3.0, 0.0, 0.0);
        let camera_dir = Point3::new(1.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&camera_pos, &camera_dir, &Vector3::y()).to_homogeneous();
        let model_view_projection = projection.into_inner() * view * model;

        transform.set_uniform_matrix4fv(&model_view_projection);

        window.gl_swap_window();
        iter = iter + 0.05;
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}
