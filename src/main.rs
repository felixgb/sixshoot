extern crate gl;
extern crate sdl2;

pub mod program_load;

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
    let _gl = 
        gl::load_with(|s| video_subsystem
                      .gl_get_proc_address(s) as *const std::os::raw::c_void
                     );

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut events = sdl.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                e => {
                    println!("{:?}", e);
                },
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
