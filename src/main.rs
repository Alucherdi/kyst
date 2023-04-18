use crossterm::terminal::disable_raw_mode;

use std::fs;
use serde_json::Value;

pub mod path_resolver;
pub mod key_handler;
pub mod renderer;
pub mod app;

fn main() {
    let raw_config: String = String::from_utf8_lossy(
        &fs::read("/home/alucherdi/.config/kyst/kyst.json").unwrap()
    ).parse().unwrap();

    let config: Value = serde_json::from_str(&raw_config)
        .unwrap();

    let mut app = app::App::new(
        config["path"].as_str().unwrap(),
        config["command"].as_str().unwrap()
    );

    app.run().expect("Error while running");
    app.clear_screen();

    disable_raw_mode().unwrap();
}
