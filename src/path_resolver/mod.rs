pub fn get_dirs(path: &str) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();

    let search = glob::glob(path)
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
