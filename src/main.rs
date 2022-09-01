use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm_rs;

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

    // projectm::settings
    let settings = projectm_rs::projectm_settings {
        mesh_x: 96,
        mesh_y: 54,
        fps: 30,
        texture_size: 512,
        window_width: 640,
        window_height: 360,
        preset_duration: 15.0,
        soft_cut_duration: 15.0,
        hard_cut_duration: 60.0,
        hard_cut_enabled: false,
        hard_cut_sensitivity: 0.0,
        beat_sensitivity: 0.5,
        aspect_correction: true,
        easter_egg: 0.5,
        shuffle_enabled: true,
        soft_cut_ratings_enabled: true,
        preset_url: "/presets".as_bytes().as_ptr() as *mut i8,
        title_font_url: "".as_bytes().as_ptr() as *mut i8,
        menu_font_url: "".as_bytes().as_ptr() as *mut i8,
        data_dir: "./".as_bytes().as_ptr() as *mut i8,
    };
    // print_settings(settings);

    // projectm::init
    let projectm_handle = unsafe {
        projectm_rs::projectm_create_settings(&settings, 0)
    };

    unsafe {
        projectm_rs::projectm_select_random_preset(projectm_handle, true);
        projectm_rs::projectm_set_window_size(projectm_handle, settings.window_width.try_into().unwrap(), settings.window_height.try_into().unwrap())
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
            projectm_rs::projectm_render_frame(projectm_handle);    
        }
        
        

        canvas.clear();
        canvas.present();
    }

    // finish okay
    Ok(())
}

fn generate_random_audio_data(projectm_handle: *mut projectm_rs::projectm)
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
        projectm_rs::projectm_pcm_add_int16(projectm_handle, &pcm_data[0][0], 512, 2)
    }
}