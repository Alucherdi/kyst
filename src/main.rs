use colored::Colorize;

use glob::glob;

use console::Term;
use console::Key;

fn get_dirs(path: &str) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();

    let search = glob(path)
        .expect("Error reading path");

    for entry in search {
        match entry {
            Ok(dir) => {
                if dir.is_dir() {
                    dirs.push(dir.display().to_string());
                }
            },
            Err(_) => println!("Error"),
        }
    }

    dirs
}

fn print_dirs(selected: u16) -> String {
    let dirs = get_dirs("/home/alucherdi/hj/*/*");
    let mut res: String = dirs[0].clone();

    let mut it: u16 = 0;
    for dir in dirs {
        let line: colored::ColoredString;

        if it == selected {
            line = dir.blue();
            res = dir;
        } else {
            line = dir.white();
        }

        it += 1;
        println!("{}", line);
    }

    res
}

enum Mode {
    Insert,
    Selection
}

fn main() {
    let stdout = Term::buffered_stdout();

    let mut mode = Mode::Selection;

    let mut selected_dir: String;
    let mut selection: u16 = 0;

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", selection);
        selected_dir = print_dirs(selection);


        if let Ok(key) = stdout.read_key() {

            match mode {
                Mode::Selection => {
                    match key {
                        Key::Enter => {
                            println!("{}", selected_dir);
                            break;
                        },

                        Key::Char('j') => selection += 1,
                        Key::Char('k') => if selection > 0 { selection -= 1 },
                        Key::Char('i') => mode = Mode::Insert,

                        _ => {},
                    }
                },

                Mode::Insert => {
                },
            }
        }
    }
}
