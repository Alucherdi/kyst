use crossterm::event::{self, KeyCode};
use std::time::Duration;

use crossterm::event::{Event, KeyEvent};

use crate::app::AppEvent;


pub fn read_key() -> AppEvent {
    if event::poll(Duration::from_millis(500)).unwrap() {
        if let Event::Key(event) = event::read().unwrap() {
            match event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => return AppEvent::End,
                _ => {}
            };

            match event {
                KeyEvent {
                    code: KeyCode::Char('n'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => return AppEvent::MoveY(1),
                KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => return AppEvent::MoveY(-1),
                _ => {
                    match event.code {
                        KeyCode::Char(c) =>
                            return AppEvent::SendKeyStroke(c),
                        KeyCode::Backspace =>
                            return AppEvent::SendSpecial(KeyCode::Backspace),
                        KeyCode::Enter =>
                            return AppEvent::SendSpecial(KeyCode::Enter),
                        _ => {
                        }
                    };
                }
            };
        }
    }

    AppEvent::Non
}
