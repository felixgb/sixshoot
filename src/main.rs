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
mod texture;

use glfw::*;
use program::{ModelProgram, LightProgram, load_shader_file};
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use program::uniform::get_uniform_location;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

const SHADOW_WIDTH: usize = 2048;
const SHADOW_HEIGHT: usize = 2048;

type Events = Receiver<(f64, WindowEvent)>;

fn load_programs() -> (ModelProgram, LightProgram) {
    let vert_shader = load_shader_file("src/shaders/vert.shdr", gl::VERTEX_SHADER);
    let frag_shader = load_shader_file("src/shaders/frag.shdr", gl::FRAGMENT_SHADER);
    let light_vert_shader = load_shader_file("src/shaders/light_vert.shdr", gl::VERTEX_SHADER);
    let light_frag_shader = load_shader_file("src/shaders/light_frag.shdr", gl::FRAGMENT_SHADER);

    let model_program = ModelProgram::from_shaders(&vert_shader, &frag_shader);
    let light_program = LightProgram::from_shaders(&light_vert_shader, &light_frag_shader);

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

#[allow(unused_assignments)]
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

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = create_window(glfw);

    gl::load_with(
        |s| window.get_proc_address(s) as *const std::os::raw::c_void
    );

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let cube_texture = texture::prepare_textures("assets/container.jpg");

    let mut all_models = maps::read_map("assets/first.map", cube_texture);

    let (program, light_program) = load_programs();

    let projection = glm::perspective(
        WIDTH as f32 / HEIGHT as f32,
        std::f32::consts::PI / 3.0,
        0.1,
        1000.0
    ); 

    let light_projection = glm::ortho(
        -50.0,
        50.0,
        -50.0,
        50.0,
        1.0,
        75.0
    );

    let light_inv_view = glm::vec3(25.0, 25.0, 60.0);

    let light_view = glm::look_at(
        &light_inv_view,
        &glm::vec3(15.0, 0.0, 0.0),
        &glm::vec3(0.0, 1.0, 0.0)
    );

    let mut camera = camera::Camera::new();
    let mut controls = controls::Controls::new(&mut camera);
    let mut mark_time = start_timer();

    let depth_bias_loc = get_uniform_location(program.program.id, "light_space").unwrap();
    let shadow_map_uniform = get_uniform_location(program.program.id, "shadowMap").unwrap();
    let texture_uniform = get_uniform_location(program.program.id, "ourTexture").unwrap();

    // ---------------------------------------------------------------------

    let mut depth_map_fbo_location: gl::types::GLuint = 0;
    let mut depth_map_location: gl::types::GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut depth_map_fbo_location);

        gl::GenTextures(1, &mut depth_map_location);
        gl::BindTexture(gl::TEXTURE_2D, depth_map_location);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as gl::types::GLint,
            SHADOW_WIDTH as gl::types::GLint,
            SHADOW_HEIGHT as gl::types::GLint,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null_mut() as *const gl::types::GLvoid
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as gl::types::GLint);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_MODE, gl::COMPARE_REF_TO_TEXTURE as gl::types::GLint);

        gl::BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo_location);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map_location, 0);

        gl::DrawBuffer(gl::NONE);
        gl::ReadBuffer(gl::NONE);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("framebuffer broken");
        }
    }

    let newcube = model::Model::test_cube_model(glm::vec3(5.0, 1.5, 20.0), cube_texture);
    all_models.push(newcube);

    let quad_vertex_buffer_data = vec![ 
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        -1.0,  1.0, 0.0,
        -1.0,  1.0, 0.0,
        1.0, -1.0, 0.0,
        1.0,  1.0, 0.0,
    ];
    let mut quad_vertexbuffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut quad_vertexbuffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertexbuffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_vertex_buffer_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            quad_vertex_buffer_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }
    let debug_v = load_shader_file("src/shaders/passthrough_vert.shdr", gl::VERTEX_SHADER);
    let debug_f = load_shader_file("src/shaders/simple_texture.shdr", gl::FRAGMENT_SHADER);

    let debug_program = program::Program::from_shaders(&debug_v, &debug_f).unwrap();
    let debug_texture_loc = get_uniform_location(debug_program.id, "test_texture").unwrap();

    program.program.set_used();
    texture_uniform.set_uniform_int(0);
    shadow_map_uniform.set_uniform_int(1);

    debug_program.set_used();
    debug_texture_loc.set_uniform_int(0);

    // ---------------------------------------------------------------------
    while !window.should_close() {
        let delta_millis = mark_time();
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

        light_program.program.set_used();
        light_program.mvp.set_vp(&light_view, &light_projection);

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo_location);
            gl::Viewport(0, 0, SHADOW_WIDTH as i32, SHADOW_HEIGHT as i32);
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::FRONT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, depth_map_location);

            for cube in &all_models {
                let model = cube.translation;
                light_program.mvp.set_m(&model);

                cube.draw_no_textures();
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // Reset
            // gl::Disable(gl::CULL_FACE);
            gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view = controls.camera.view();

        program.program.set_used();
        program.mvp.set_vp(&view, &projection);
        program.lights.light_pos.set_uniform_vec3(&light_inv_view);
        program.lights.view_pos.set_uniform_vec3(&controls.pos_now());
        let depth_mvp = light_projection * light_view;
        depth_bias_loc.set_uniform_matrix4fv(&depth_mvp);

        for cube in &all_models {
            let model = cube.translation;

            program.mvp.set_m(&model);

            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, cube_texture);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, depth_map_location);
            }
            cube.draw_no_textures();
        }

        // unsafe {
        //     gl::Viewport(0, 0, 512, 512);
        // }
        // debug_program.set_used();
        // unsafe {
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     gl::BindTexture(gl::TEXTURE_2D, depth_map_location);
        // }
        // unsafe {
        //     vertex::vertex_attrib_pointer(0, 3, 0, 0);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertexbuffer);
        //     gl::DrawArrays(gl::TRIANGLES, 0, 6);
        // }

        window.swap_buffers();
    }

}
