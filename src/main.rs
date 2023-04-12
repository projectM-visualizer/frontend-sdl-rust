mod app;
mod dummy_audio;

fn main() -> Result<(), String> {
    let config = app::default_config();
    // TODO: parse args here for config
    // config.preset_path = Some("./presets/test".to_string());

    let mut app = app::App::new(Some(config));
    app.init();

    app.main_loop();

    Ok(())
}
