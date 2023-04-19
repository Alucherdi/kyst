use std::io::stdout;

use crossterm::{
    execute,
    style::{Print, SetForegroundColor, SetBackgroundColor, Color, ResetColor},
    cursor
};

pub fn render_header(search: &str) {
    execute!(
        stdout(),
        Print(
            format!(
                "Search: {}\n", 
                search
            )
        ),
        cursor::MoveLeft(11 + (search.len() as u16)),
    ).unwrap();
}

pub fn render_options(
    screen_options: Vec<&String>,
    selection: i16,
    term_size: (u16, u16),
) {
    for (i, option) in screen_options.iter().enumerate() {
        execute!(
            stdout(),
            SetForegroundColor(
                if i == selection as usize { Color::Black }
                else { Color::Reset }
            ),
            SetBackgroundColor(
                if i == selection as usize { Color::White }
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
        ).unwrap();
    }
}
