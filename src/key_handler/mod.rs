use crossterm::event::{self, KeyCode};
use std::time::Duration;

use crossterm::event::{Event, KeyEvent};

use crate::app::AppEvent;

pub struct KeyHandler;

impl KeyHandler {
    pub fn read_key(&self) -> AppEvent {
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
                    KeyEvent {
                        code: KeyCode::Char('j'),
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => {
                        return AppEvent::MoveY(1);
                    },
                    _ => {
                        println!("{:?}", event.code);
                    },
                }
            }
        }

        AppEvent::Non
    }
}
