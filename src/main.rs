extern crate gl;
extern crate glfw;
extern crate nalgebra;

mod render;
mod vertex;
mod uniform;
mod model;
mod camera;

use glfw::*;
use nalgebra::*;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

type Events = Receiver<(f64, WindowEvent)>;

fn create_window(glfw: Glfw) -> (Window, Events) {
    let (mut window, events) = glfw.create_window(
        WIDTH,
        HEIGHT,
        "hello i am window",
        glfw::WindowMode::Windowed
    ).expect("failed to create window!");

    window.set_pos(1000, 200);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.make_current();

    (window, events)
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = create_window(glfw);

    gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 1920, 1080);
    }
    // let vertices = obj::read_lines().unwrap().compute_faces();

    let cubes = vec![
        model::Model::test_cube_model(Point3::new(-3.0, 0.0, 0.0)),
        model::Model::test_cube_model(Point3::new(0.0, 0.0, 0.0)),
        model::Model::test_cube_model(Point3::new(3.0, 0.0, 0.0)),
    ];

    let mut iter = 0.0;

    let program = render::Program::use_program_from_sources("src/vert.shdr", "src/frag.shdr");
    let transform = uniform::Uniform::get_uniform_location(program.id, "transform").unwrap();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let projection = Perspective3::new(16.0 / 9.0, std::f32::consts::PI / 2.0, 0.1, 1000.0);

    let mut camera = camera::Camera::new();
    let mut delta_time;
    let mut last_frame = 0.0;
    let start = SystemTime::now();

    let mut last_x = 1920.0 / 2.0;
    let mut last_y = 1080.0 / 2.0;

    let mut yaw = 90.0;
    let mut pitch = 0.0;

    let sensitivity = 0.5;
    let mut forward = false;
    let mut right = false;
    let mut backward = false;
    let mut left = false;

    while !window.should_close() {
        let current_time =
            SystemTime::now().duration_since(start).unwrap().as_millis() as f32;
        delta_time = current_time - last_frame;
        last_frame = current_time;
        let camera_speed = 0.005 * delta_time;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
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
                    camera.front = front.normalize();
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
            camera.pos += camera_speed * camera.front;
        } else if backward {
            camera.pos -= camera_speed * camera.front;
        }
        if right {
            camera.pos += camera.front.cross(&camera.up).normalize() * camera_speed;
        } else if left {
            camera.pos -= camera.front.cross(&camera.up).normalize() * camera_speed;
        }

        let view = camera.view();

        for cube in &cubes {
            let rot = Rotation3::from_euler_angles(0.0, iter, 0.0);

            let model = cube.isometry.to_homogeneous() * rot.to_homogeneous();
            let model_view_projection = projection.into_inner() * view * model;

            transform.set_uniform_matrix4fv(&model_view_projection);

            cube.draw();
        }

        iter += 0.05;

        window.swap_buffers();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}
