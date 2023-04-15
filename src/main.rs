use crossterm::terminal::disable_raw_mode;

pub mod path_resolver;
pub mod key_handler;
pub mod renderer;
pub mod app;

fn main() {
    let mut app = app::App::new();

    app.run().expect("Error while running");
    app.clear_screen();

    disable_raw_mode().unwrap();
}
