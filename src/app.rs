use std::io::stdout;
use std::cmp::min;
use crossterm::event::KeyCode;
use crossterm::style::{SetForegroundColor, SetBackgroundColor, Print, ResetColor, Color};
use crossterm::{execute, cursor, terminal};
use crossterm::terminal::{Clear, ClearType, size};

use crate::key_handler::KeyHandler;
use crate::path_resolver::get_dirs;

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
        let options: Vec<String> = get_dirs("/home/alucherdi/hj/*/*");
        let mut filtered_options: Vec<&String>;
        let mut screen_options: Vec<&String>;

        let mut term_size: (u16, u16);

        let mut limit: usize;
        let mut offset: usize = 0;

        terminal::enable_raw_mode().unwrap();

        self.clear_screen();
        loop {
            self.clear_screen();

            execute!(
                stdout(),
                Print(format!("Prompt[{}]: {}\n", 
                    match self.mode {
                        AppMode::Normal => "n",
                        AppMode::Insert => "i"
                    },
                    self.search)),
                cursor::MoveLeft(11 + (self.search.len() as u16)),
            )?;

            filtered_options = options
                .iter()
                .filter(|&x| x.contains(&self.search))
                .collect();

            term_size = size()?;

            limit = min(
                filtered_options.len(),
                term_size.1 as usize - 2
            ) + offset;

            screen_options = filtered_options[offset..limit].to_vec();

            for (i, option) in screen_options.iter().enumerate() {
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
                    Print(
                        if option.len() > term_size.0 as usize {
                            option
                                .to_string()[..term_size.0 as usize]
                                .to_string() 
                        } else { option.to_string() } +
                        "\n"
                    ),
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

                    if self.selection > ((limit - offset) as i16) - 1 {
                        offset += 1;
                        self.selection -= to;
                    }
                    /*
                    if self.selection < 0 {
                        if offset > 0 {
                            offset -= 1;
                        }

                        self.selection = 0 + offset as i16;
                    };
                    if self.selection >= limit as i16 {
                        if offset < options.len() {
                            offset += 1;
                        }

                        self.selection = (limit - 1) as i16;
                    };
                    */
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
        }

        Ok(())
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}
