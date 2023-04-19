use std::{env, fs};

use serde_json::{Value, from_str};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub command: String
}

impl Config {
   pub fn new(path: &str, command: &str) -> Config {
       Config {
           path: path.to_string(),
           command: command.to_string(),
       }
   }

   pub fn empty() -> Config {
       Config {
           path: env::current_dir()
               .unwrap()
               .to_str()
               .unwrap()
               .to_string() + "/*",
           command: "tmux new -s {name}".to_string()
       }
   }
}

pub fn load_config() -> Option<Config> {
    let home = match env::var_os("HOME") {
        Some(var) => var
            .to_str()
            .unwrap()
            .to_string(),
        None => {
            // HOME env does not exist
            return None
        }
    };

    let config_path = format!(
        "{}/.config/kyst/kyst.json",
        home
    );

    let config_path = match fs::read(config_path) {
        Ok(content) => content,
        Err(_) => {
            // HOME/.config/kyst/kyst.json does not exist
            return None
        },
    };

    let raw_config: String = String::from_utf8_lossy(
        &config_path
    ).parse().unwrap();

    let json_config: Value = from_str(&raw_config)
        .unwrap();

    Some(Config::new(
        json_config["path"]
            .as_str()
            .unwrap(),

        json_config["command"]
            .as_str()
            .unwrap()
    ))

}
