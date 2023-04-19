use std::io::stdout;
use std::cmp::min;
use crossterm::event::KeyCode;
use crossterm::{execute, cursor, terminal};
use crossterm::terminal::{Clear, ClearType, size};

use crate::key_handler::read_key;
use crate::path_resolver::get_dirs;
use crate::renderer::{render_header, render_options};

pub enum AppEvent {
    End,
    MoveY(i16),
    SendKeyStroke(char),
    SendSpecial(KeyCode),
    Non,
}

pub struct App {
    selection: i16,
    search: String,
    path: String,
}

impl App {

    pub fn new(path: &str) -> App {
        println!("{}", path);
        App {
            search: String::new(),
            selection: 0,
            path: path.to_string()
        }
    }

    pub fn run(&mut self) -> std::io::Result<(bool, String)> {
        let options: Vec<String> = get_dirs(&self.path);
        let mut filtered_options: Vec<&String>;
        let mut screen_options: Vec<&String>;
        let mut term_size: (u16, u16);
        let mut limit: usize;
        let mut offset: usize = 0;
        let mut success = false;

        terminal::enable_raw_mode().unwrap();

        loop {
            self.clear_screen();

            render_header(&self.search);

            filtered_options = options
                .iter()
                .filter(|&x| x.contains(&self.search))
                .collect();

            term_size = size()?;

            limit = min(
                filtered_options.len(),
                term_size.1 as usize - 2
            ) + offset;

            if limit > options.len() { limit = options.len() - 1 };

            screen_options = filtered_options[offset..limit].to_vec();

            render_options(screen_options, self.selection, term_size);

            match read_key() {
                AppEvent::End => {
                    break;
                },
                AppEvent::MoveY(to) => {
                    self.selection += to;

                    if self.selection < 0 {
                        if offset > 0 {
                            offset -= 1;
                            limit -= 1;
                        }
                        self.selection = 0;
                    }

                    if self.selection > ((limit - offset) as i16) - 1 {
                        if limit < filtered_options.len() - 1 { offset += 1; }
                        self.selection -= 1;
                    }
                },
                AppEvent::SendKeyStroke(ks) => {
                    self.search.push(ks);
                    offset = 0;
                    self.selection = 0;
                },
                AppEvent::SendSpecial(kc) => {
                    match kc {
                        KeyCode::Backspace => {
                            self.search.pop();
                        },
                        KeyCode::Enter => {
                            success = true;
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }

        self.clear_screen();
        terminal::disable_raw_mode().unwrap();
        Ok((success, filtered_options[self.selection as usize].to_string()))
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}
