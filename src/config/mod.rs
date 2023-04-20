use std::{env, fs};

use serde_json::{Value, from_str};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub command: String
}

impl Config {
   fn new(path: &str, command: &str) -> Config {
       Config {
           path: path.to_string(),
           command: command.to_string(),
       }
   }

   fn default() -> Config {
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

pub fn load_config() -> Config {
    let default_path = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() + "/*";

    let default_command = "tmux new -s {name}".to_string();

    let home = match env::var_os("HOME") {
        Some(var) => var
            .to_str()
            .unwrap()
            .to_string(),
        None => {
            return Config::default(); 
        }
    };

    let config_path = format!(
        "{}/.config/kyst/kyst.json",
        home
    );

    let raw_config: String = match fs::read(config_path) {
        Ok(config_path) => String::from_utf8_lossy(
            &config_path
        ).parse().unwrap(),
        Err(_) => {
            return Config::default();
        },
    };

    let json_config: Value = match from_str(&raw_config) {
        Ok(jc) => jc,
        Err(_) => {
            return Config::default();
        }
    };


    Config::new(
        if json_config["path"].is_null() {
            &default_path
        } else {
            json_config["path"]
                .as_str()
                .unwrap()
        },

        if json_config["command"].is_null() {
            &default_command 
        } else {
            json_config["command"]
                .as_str()
                .unwrap()
        }
    )
}
