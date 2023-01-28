use projectm_rs::core::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // create window
    // get screen dimensions
    let mut display_index = 0;
    let display_mode = video_subsystem.desktop_display_mode(display_index)?;
    let mut window_width = display_mode.w as u32;
    let mut window_height = display_mode.h as u32;
    let window = video_subsystem
        .window("frontend-sdl2-rust", window_width, window_height)
        .opengl()
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    // create canvas/renderer
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .expect("could not make a canvas");


    // projectm::init
    let projectm_handle = unsafe {
        projectm::create()
    };

    unsafe {
        projectm::set_window_size(projectm_handle, canvas.output_size().unwrap().0.try_into().unwrap(), canvas.output_size().unwrap().1.try_into().unwrap())
    }
    println!("projectm initialized!");

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

        // projectm::render
        unsafe {
            projectm::render_frame(projectm_handle);
        }
        
        // present/render
        canvas.present();
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

    unsafe {
        projectm::pcm_add_int16(projectm_handle, &pcm_data[0][0], 512, 2)
    }
}
