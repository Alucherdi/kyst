pub fn get_dirs(path: &str) -> Vec<String> {
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

pub fn print_dirs(selected: u16) -> String {
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
