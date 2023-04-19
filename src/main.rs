use crossterm::terminal::disable_raw_mode;

use std::{env, fs, process::Command};
use serde_json::Value;

pub mod path_resolver;
pub mod key_handler;
pub mod renderer;
pub mod app;

fn main() {
    let config_path =
        format!(
            "{}/.config/kyst/kyst.json",
            env::var_os("HOME")
                .unwrap()
                .to_str()
                .unwrap()
        );

    let raw_config: String = String::from_utf8_lossy(
        &fs::read(config_path).unwrap()
    ).parse().unwrap();

    let config: Value = serde_json::from_str(&raw_config)
        .unwrap();

    let mut app = app::App::new(
        config["path"]
            .as_str()
            .unwrap()
    );

    let selection = app.run()
        .expect("Error while running");

    app.clear_screen();

    disable_raw_mode().unwrap();


    let raw_command = config["command"]
        .as_str()
        .unwrap()
        .replace("{name}", selection.1.split("/").last().unwrap())
        .replace("{path}", &selection.1);

    if selection.0 {
        // if linux
        Command::new("sh")
            .args([
                "-c",
                &raw_command
            ])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
