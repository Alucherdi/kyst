use std::io::stdout;
use crossterm::event::KeyCode;
use crossterm::style::{SetForegroundColor, SetBackgroundColor, Print, ResetColor, Color};
use crossterm::{execute, cursor, terminal};
use crossterm::terminal::{Clear, ClearType};

use crate::key_handler::KeyHandler;

pub enum AppEvent {
    End,
    MoveY(i16),
    SendKeyStroke(char),
    SendSpecial(KeyCode),
    ChangeMode(AppMode),
    Non,
}

pub enum AppMode {
    Normal,
    Insert,
}


pub struct App {
    key_handler: KeyHandler,
    selection: i16,
    search: String,
    mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            key_handler: KeyHandler,
            mode: AppMode::Normal,
            search: String::new(),
            selection: 0,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let options: Vec<&str> = vec![
            "option 1",
            "option 2",
            "option 3"
        ];

        terminal::enable_raw_mode().unwrap();

        self.clear_screen();
        loop {
            execute!(
                stdout(),
                Print(format!("Prompt: {}\n", self.search)),
                cursor::MoveLeft(8 + (self.search.len() as u16)),
            )?;

            for (i, option) in options.iter()
                .filter(|&x| x.contains(&self.search))
                .enumerate() {
                execute!(
                    stdout(),
                    SetForegroundColor(
                        if i == self.selection as usize { Color::Black }
                        else { Color::Reset }
                    ),
                    SetBackgroundColor(
                        if i == self.selection as usize { Color::White }
                        else { Color::Reset }
                    ),
                    Print(option.to_string() + "\n"),
                    cursor::MoveLeft(option.len() as u16),
                    ResetColor
                )?;
            }

            match self.key_handler.read_key(&self.mode) {
                AppEvent::End => {
                    break;
                },
                AppEvent::MoveY(to) => {
                    self.selection += to;
                    if self.selection < 0 { self.selection = 0; };
                },
                AppEvent::ChangeMode(new_mode) => {
                    self.mode = new_mode;
                },
                AppEvent::SendKeyStroke(ks) => {
                    self.search.push(ks);
                },
                AppEvent::SendSpecial(kc) => {
                    match kc {
                        KeyCode::Backspace => {
                            self.search.pop();
                        },
                        _ => {}
                    }
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
