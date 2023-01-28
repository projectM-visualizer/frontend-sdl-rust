use projectm_rs::core::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;


fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // request GL version
    // TODO: deal with OpenGL ES here
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_flags().debug().set();
    assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    assert_eq!(gl_attr.context_version(), (3, 3));

    // create window
    // get screen dimensions
    let  display_index = 0;
    let display_mode = video_subsystem.desktop_display_mode(display_index)?;
    let  window_width = display_mode.w as u32;
    let  window_height = display_mode.h as u32;
    let window = video_subsystem
        .window("frontend-sdl2-rust", window_width, window_height)
        .opengl()
        .position_centered()
        .allow_highdpi()
        .build()
        .expect("could not initialize video subsystem");

    // create openGL context
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    // initialize projectM
    let projectm_handle = projectm::create() ;

    // get/set window size
    let (width, height) = window.drawable_size(); // highDPI aware
    projectm::set_window_size(projectm_handle, width.try_into().unwrap(), height.try_into().unwrap());

    // events
    let mut event_pump = sdl_context.event_pump()?;

    // renderLoop
    'running: loop {

        // check for event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // generate random audio
        generate_random_audio_data(projectm_handle);

        // render a frame
            projectm::render_frame(projectm_handle);
        
        // swap buffers
        window.gl_swap_window();
    }

    // finish okay
    Ok(())
}

fn generate_random_audio_data(projectm_handle: projectm_handle)
{
    let mut pcm_data: [[libc::c_short; 512]; 2] = [[0; 512]; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        if i % 2 as libc::c_int == 1 as libc::c_int {
            pcm_data[0 as libc::c_int as usize][i as usize] =
                -(pcm_data[0 as libc::c_int as usize][i as usize] as
                      libc::c_int) as libc::c_short;
            pcm_data[1 as libc::c_int as usize][i as usize] =
                -(pcm_data[1 as libc::c_int as usize][i as usize] as
                      libc::c_int) as libc::c_short
        }
        i += 1
    };

        projectm::pcm_add_int16(projectm_handle, &pcm_data[0][0], 512, 2)
}
