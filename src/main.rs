#![feature(clamp)]
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
use std::time::SystemTime;

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
    sdl.mouse().set_relative_mouse_mode(true);

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

    let cubes = vec![
        model::Model::test_cube_model(Point3::new(0.0, 0.0, -3.0)),
        model::Model::test_cube_model(Point3::new(0.0, 0.0, 0.0)),
        model::Model::test_cube_model(Point3::new(0.0, 0.0, 3.0)),
    ];

    let mut events = sdl.event_pump().unwrap();
    let mut iter = 0.0;

    let transform = uniform::Uniform::get_uniform_location(program.id, "transform").unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let projection = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);

    let mut camera_pos: Vector3<f32> = Vector3::new(0.0, 0.0, 10.0);
    let mut camera_front: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let camera_up = Vector3::y();

    let mut delta_time;
    let mut last_frame = 0.0;
    let start = SystemTime::now();

    let mut last_x = 0;
    let mut last_y = 0;

    let mut yaw = 180.0;
    let mut pitch = 0.0;

    let sensitivity = 0.5;
    'main: loop {
        let current_time = SystemTime::now().duration_since(start).unwrap().as_millis() as f32;
        delta_time = current_time - last_frame;
        last_frame = current_time;
        let camera_speed = 0.005 * delta_time;

        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    let x_offset = (x - last_x) as f32 * sensitivity;
                    let y_offset = (last_y - y) as f32 * sensitivity;
                    last_x = x;
                    last_y = y;
                    yaw += x_offset;
                    pitch += y_offset;
                    pitch = pitch.clamp(-89.0, 89.0);

                    let front = Vector3::new(
                        yaw.to_radians().cos() * pitch.to_radians().cos(),
                        pitch.to_radians().sin(),
                        yaw.to_radians().sin() * pitch.to_radians().cos()
                    );
                    camera_front = front.normalize();
                }
                sdl2::event::Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Q => break 'main,
                    Keycode::W => {
                        camera_pos += camera_speed * camera_front;
                    },
                    Keycode::A => {
                        camera_pos -= camera_front.cross(&camera_up).normalize() * camera_speed;
                    },
                    Keycode::S => {
                        camera_pos -= camera_speed * camera_front;
                    },
                    Keycode::D => {
                        camera_pos += camera_front.cross(&camera_up).normalize() * camera_speed;
                    },
                    _ => { },
                },
                _ => { }
            }
        }

        let p = Point3::from(camera_pos);
        let v = Point3::from(camera_pos + camera_front);
        let view = Isometry3::look_at_rh(&p, &v, &camera_up).to_homogeneous();

        for cube in &cubes {
            let rot = Rotation3::from_euler_angles(0.0, iter, 0.0);

            let model = cube.isometry.to_homogeneous() * rot.to_homogeneous();
            let model_view_projection = projection.into_inner() * view * model;

            transform.set_uniform_matrix4fv(&model_view_projection);

            cube.draw();
        }

        window.gl_swap_window();
        iter = iter + 0.05;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}
