use crossterm::event::{self, KeyCode};
use std::time::Duration;

use crossterm::event::{Event, KeyEvent};

use crate::app::{AppEvent, AppMode};

pub struct KeyHandler;

impl KeyHandler {
    pub fn read_key(&self, app_mode: &AppMode) -> AppEvent {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => {
                        return AppEvent::End;
                    },
                    _ => {}
                }

                match app_mode {
                    AppMode::Normal => {
                        match event {
                            KeyEvent {
                                code: KeyCode::Char('n'),
                                modifiers: event::KeyModifiers::CONTROL,
                                kind: _,
                                state: _,
                            } => {
                                return AppEvent::MoveY(1);
                            },
                            KeyEvent {
                                code: KeyCode::Char('p'),
                                modifiers: event::KeyModifiers::CONTROL,
                                kind: _,
                                state: _,
                            } => {
                                return AppEvent::MoveY(-1);
                            },
                            KeyEvent {
                                code: KeyCode::Char('i'),
                                modifiers: _,
                                kind: _,
                                state: _,
                            } => {
                                return AppEvent::ChangeMode(AppMode::Insert);
                            },
                            _ => {
                            },
                        }
                    },

                    AppMode::Insert => {
                        match event.code { 
                            KeyCode::Char(c) => {
                                return AppEvent::SendKeyStroke(c);
                            },
                            KeyCode::Enter => {
                                return AppEvent::SendSpecial(KeyCode::Enter);
                            },
                            KeyCode::Backspace => {
                                return AppEvent::SendSpecial(KeyCode::Backspace);
                            },
                            KeyCode::Esc => {
                                return AppEvent::ChangeMode(AppMode::Normal);
                            },
                            _ => {},
                        }
                    }
                }
            }
        }

        AppEvent::Non
    }
}
