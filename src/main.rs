extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod render;
mod vertex;
mod uniform;
mod model;
mod camera;
mod controls;
mod collide;
mod maps;
mod glm_utils;

use glfw::*;
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

fn make_timer() -> impl FnMut() -> f32 {
    let mut delta_millis = 0.0;
    let mut last_frame = 0.0;

    let start = SystemTime::now();

    move || {
        let millis_since_start = SystemTime::now()
            .duration_since(start)
            .unwrap()
            .as_millis() as f32;

        delta_millis = millis_since_start - last_frame;
        last_frame = millis_since_start;
        delta_millis
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = create_window(glfw);

    gl::load_with(
        |s| window.get_proc_address(s) as *const std::os::raw::c_void
    );

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::Enable(gl::DEPTH_TEST);
    }

    // let vertices = obj::read_lines().unwrap().compute_faces();

    let cubes = maps::read_map("assets/first.map");

    let program = render::Program::use_program_from_sources("src/shaders/vert.shdr", "src/shaders/frag.shdr");
    // let light_program = render::Program::use_program_from_sources("src/shaders/vert.shdr", "src/shaders/light_frag.shdr");

    // light_program.set_used();
    // let light_model_transform = uniform::Uniform::get_uniform_location(light_program.id, "model").unwrap();
    // let light_transform = uniform::Uniform::get_uniform_location(light_program.id, "transform").unwrap();

    program.set_used();

    let transform = uniform::Uniform::get_uniform_location(program.id, "transform").unwrap();
    let model_transform = uniform::Uniform::get_uniform_location(program.id, "model").unwrap();

    let light_pos = glm::vec3(5.0, 5.0, 15.0);
    let _light_cube = model::Model::test_cube_model(light_pos);
    let _light_scale = glm::scaling(&glm::vec3(0.1, 0.1, 0.1));

    let light_color = uniform::Uniform::get_uniform_location(program.id, "light_color").unwrap();
    let object_color = uniform::Uniform::get_uniform_location(program.id, "object_color").unwrap();
    let light_pos_transform = uniform::Uniform::get_uniform_location(program.id, "light_pos").unwrap();

    // unsafe {
    //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // }

    let projection = glm::perspective(
        WIDTH as f32 / HEIGHT as f32,
        std::f32::consts::PI / 3.0,
        0.1,
        1000.0
    );

    let mut camera = camera::Camera::new();
    let mut controls = controls::Controls::new(&mut camera);
    let mut timer = make_timer();

    while !window.should_close() {
        let delta_millis = timer();

        glfw.poll_events();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Q, _, _, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    controls.key_move_callback(key, action);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    controls.mouse_callback(x as f32, y as f32);
                }
                _ => { }
            }
        }
        controls.update(delta_millis, &cubes);

        let view = controls.camera.view();

        // light_program.set_used();
        // {
        //     let model = light_cube.isometry.to_homogeneous() * light_scale;
        //     model_transform.set_uniform_matrix4fv(&model);
        //     light_model_transform.set_uniform_matrix4fv(&model);

        //     let model_view_projection = projection.into_inner() * view * model;

        //     light_transform.set_uniform_matrix4fv(&model_view_projection);
        //     light_cube.draw();
        // }

        program.set_used();

        light_color.set_uniform_vec3(&glm::vec3(1.0, 1.0, 1.0));
        object_color.set_uniform_vec3(&glm::vec3(1.0, 0.5, 0.31));
        light_pos_transform.set_uniform_vec3(&light_pos);

        for cube in &cubes {
            let model = cube.translation;
            model_transform.set_uniform_matrix4fv(&model);
            // light_model_transform.set_uniform_matrix4fv(&model);

            let model_view_projection = projection * view * model;

            transform.set_uniform_matrix4fv(&model_view_projection);
            cube.draw();
        }

        window.swap_buffers();
    }

}
