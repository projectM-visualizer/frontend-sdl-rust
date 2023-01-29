mod app;
mod dummy_audio;

fn main() -> Result<(), String> {
    let config = app::default_config();
    // TODO: parse args here for config
    // config.preset_path = Some("/usr/local/share/projectM/presets".to_string());

    let mut app = app::App::new(Some(config));

    app.main_loop();

    Ok(())
}
