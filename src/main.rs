extern crate gl;
extern crate glfw;
extern crate nalgebra;

pub mod render;
pub mod obj;
pub mod vertex;
pub mod uniform;
pub mod model;

use nalgebra::*;
use std::time::SystemTime;
use glfw::*;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(
        WIDTH,
        HEIGHT,
        "hello i am window",
        glfw::WindowMode::Windowed
    ).expect("failed to create window!");

    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.make_current();

    let _gl =
        gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 1920, 1080);
    }

    let vert_shader = render::Shader::from_vert_source(include_str!("vert.shdr")).unwrap();
    let frag_shader = render::Shader::from_frag_source(include_str!("frag.shdr")).unwrap();
    let program = render::Program::from_shaders(&vert_shader, &frag_shader).unwrap();

    program.set_used();

    // let vertices = obj::read_lines().unwrap().compute_faces();

    let cubes = vec![
        model::Model::test_cube_model(Point3::new(-3.0, 0.0, 0.0)),
        model::Model::test_cube_model(Point3::new(0.0, 0.0, 0.0)),
        model::Model::test_cube_model(Point3::new(3.0, 0.0, 0.0)),
    ];

    let mut iter = 0.0;

    let transform = uniform::Uniform::get_uniform_location(program.id, "transform").unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let projection = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 0.1, 1000.0);

    let mut camera_pos: Vector3<f32> = Vector3::new(0.0, 0.0, -10.0);
    let mut camera_front: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    let camera_up = Vector3::y();

    let mut delta_time;
    let mut last_frame = 0.0;
    let start = SystemTime::now();

    let mut last_x = 1920.0 / 2.0;
    let mut last_y = 1080.0 / 2.0;

    let mut yaw = 0.0;
    let mut pitch = 0.0;

    let sensitivity = 0.5;
    let mut forward = false;
    let mut right = false;
    let mut backward = false;
    let mut left = false;
    while !window.should_close() {
        let current_time = SystemTime::now().duration_since(start).unwrap().as_millis() as f32;
        delta_time = current_time - last_frame;
        last_frame = current_time;
        let camera_speed = 0.005 * delta_time;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x_offset = (x - last_x) as f32;
                    let y_offset = (last_y - y) as f32;

                    last_x = x;
                    last_y = y;

                    yaw += x_offset * sensitivity;
                    pitch += y_offset * sensitivity;
                    if pitch > 89.0 {
                        pitch = 89.0;
                    }
                    if pitch < -89.0 {
                        pitch = -89.0;
                    }

                    let front = Vector3::new(
                        yaw.to_radians().cos() * pitch.to_radians().cos(),
                        pitch.to_radians().sin(),
                        yaw.to_radians().sin() * pitch.to_radians().cos()
                    );
                    camera_front = front.normalize();
                }
                glfw::WindowEvent::Key(key, _, action, _) => match key {
                    Key::Q => window.set_should_close(true),
                    Key::W => {
                        match action {
                            Action::Press => forward = true,
                            Action::Release => forward = false,
                            _ => { }
                        }
                    },
                    Key::A => {
                        match action {
                            Action::Press => left = true,
                            Action::Release => left = false,
                            _ => { }
                        }
                    },
                    Key::S => {
                        match action {
                            Action::Press => backward = true,
                            Action::Release => backward = false,
                            _ => { }
                        }
                    },
                    Key::D => {
                        match action {
                            Action::Press => right = true,
                            Action::Release => right = false,
                            _ => { }
                        }
                    },
                    _ => { },
                },
                _ => { }
            }
        }

        if forward {
            camera_pos += camera_speed * camera_front;
        } else if backward {
            camera_pos -= camera_speed * camera_front;
        }
        if right {
            camera_pos += camera_front.cross(&camera_up).normalize() * camera_speed;
        } else if left {
            camera_pos -= camera_front.cross(&camera_up).normalize() * camera_speed;
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

        iter = iter + 0.05;

        window.swap_buffers();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}
