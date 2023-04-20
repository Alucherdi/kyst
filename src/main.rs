use std::process::Command;

pub mod config;
pub mod path_resolver;
pub mod key_handler;
pub mod renderer;
pub mod app;

use crate::config::load_config;

fn main() {
    let conf = load_config();

    let mut app = app::App::new(
        &conf.path
    );

    let selection = app.run()
        .expect("Error while running");

    app.clear_screen();

    let raw_command = conf.command
        .replace("{name}", selection.1.split("/").last().unwrap())
        .replace("{path}", &selection.1);

    if selection.0 {
        // if linux
        Command::new("sh")
            .current_dir(selection.1)
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
