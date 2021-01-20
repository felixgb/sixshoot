extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod buffer;
mod camera;
mod collide;
mod controls;
mod glm_utils;
mod maps;
mod model;
mod obj;
mod program;
mod vertex;

use glfw::*;
use program::{ModelProgram, LightProgram, load_shader_file};
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use image::EncodableLayout;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

type Events = Receiver<(f64, WindowEvent)>;

fn load_programs() -> (ModelProgram, LightProgram) {
    let vert_shader = load_shader_file("src/shaders/vert.shdr", gl::VERTEX_SHADER);
    let frag_shader = load_shader_file("src/shaders/frag.shdr", gl::FRAGMENT_SHADER);
    let light_frag_shader = load_shader_file("src/shaders/light_frag.shdr", gl::FRAGMENT_SHADER);

    let model_program = ModelProgram::from_shaders(&vert_shader, &frag_shader);
    let light_program = LightProgram::from_shaders(&vert_shader, &light_frag_shader);

    (model_program, light_program)
}

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

fn start_timer() -> impl FnMut() -> f32 {
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

fn prepare_textures() {
    let mut texture_id: gl::types::GLuint = 0;
    let image = image::open("assets/container.jpg").unwrap().to_rgb8();
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as gl::types::GLint,
            512,
            512,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_bytes().as_ptr() as *const gl::types::GLvoid
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
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

    let thingy = obj::read_lines().unwrap().compute_faces();
    let thingy_model = model::Model::new(&thingy, glm::vec3(5.0, 1.5, 20.0));

    let mut all_models = maps::read_map("assets/first.map");
    all_models.push(thingy_model);

    let (program, light_program) = load_programs();

    let light_pos = glm::vec3(5.0, 1.5, 15.0);
    let light_cube = model::Model::test_cube_model(light_pos);
    let light_scale = glm::scaling(&glm::vec3(0.1, 0.1, 0.1));

    let projection = glm::perspective(
        WIDTH as f32 / HEIGHT as f32,
        std::f32::consts::PI / 3.0,
        0.1,
        1000.0
    );

    let mut camera = camera::Camera::new();
    let mut controls = controls::Controls::new(&mut camera);
    let mut mark_time = start_timer();

    while !window.should_close() {
        let delta_millis = mark_time();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        glfw.poll_events();

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

        controls.update(delta_millis, &all_models);

        let view = controls.camera.view();
        light_program.program.set_used();
        light_program.mvp.set_vp(&view, &projection);
        {
            let model = light_cube.translation * light_scale;
            light_program.mvp.set_m(&model);

            light_cube.draw();
        }

        program.program.set_used();
        program.lights.set_light(&light_pos, &glm::vec3(1.0, 1.0, 1.0));

        program.mvp.set_vp(&view, &projection);

        for cube in &all_models {
            let model = cube.translation;
            program.mvp.set_m(&model);
            program.lights.set_object_color(&glm::vec3(1.0, 0.5, 0.31));

            cube.draw();
        }

        window.swap_buffers();
    }

}
