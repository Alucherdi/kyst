use std::io::stdout;
use crossterm::{execute, cursor, terminal};
use crossterm::terminal::{Clear, ClearType};

use crate::key_handler::KeyHandler;

pub enum AppEvent {
    End,
    MoveY(u8),
    Non,
}

pub struct App {
    key_handler: KeyHandler
}

impl App {
    pub fn new() -> App {
        App {
            key_handler: KeyHandler,
        }
    }


    pub fn run(&self) -> std::io::Result<()> {
        let options: Vec<&str> = vec![
            "option 1",
            "option 2",
            "option 3"
        ];


        terminal::enable_raw_mode().unwrap();

        self.clear_screen();
        loop {
            for option in &options {
                println!("{}", option);
            }

            match self.key_handler.read_key() {
                AppEvent::End => {
                    break;
                },
                AppEvent::MoveY(to) => {
                }
                _ => {},
            }
            self.clear_screen();
        }

        Ok(())
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}
