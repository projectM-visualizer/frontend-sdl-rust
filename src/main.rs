use projectm_rs::core::*;

use App;

fn main() -> Result<(), String> {
    let mut app = App::new();

    app.init();

    app.main_loop();

    Ok(())
}
