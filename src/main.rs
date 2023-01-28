use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm_rs::core::*;

fn main() -> Result<(), String> {
    // setup sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // let audio_subsystem = sdl_context.audio()?;

    // create window
    let window = video_subsystem.window("frontend-sdl2-rust", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    // create canvas/renderer
    let mut canvas = window.into_canvas().build()
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
