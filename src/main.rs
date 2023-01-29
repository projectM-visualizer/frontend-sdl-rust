use projectm_sdl::app::App;

fn main() -> Result<(), String> {
    let mut app = App::new();

    app.main_loop();

    Ok(())
}
