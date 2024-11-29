use std::{env, fs::{create_dir_all, File, OpenOptions}, io::{self, BufReader}, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use crate::theme::DEFAULT;

pub const APPLICATION: &str = "lime";

pub enum ConfigError {
    UnsupportedPlatform,
    CreateDirectory,
    Read(io::Error),
    Write(io::Error),
    Decode(serde_json::Error),
    Encode(serde_json::Error)
}

pub fn find_directory(application: &str) -> Result<PathBuf, ConfigError> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(app_data) = env::var("APPDATA") {
            let mut path = PathBuf::from(app_data);
            path.push(application);
            return Ok(path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = env::var("HOME") {
            let mut path = PathBuf::from(home);
            path.push("Library");
            path.push("Application Support");
            path.push(application);
            return Ok(path);
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
            let mut path = PathBuf::from(xdg_config_home);
            path.push(application);
            return Ok(path);
        } else if let Ok(home) = env::var("HOME") {
            let mut path = PathBuf::from(home);
            path.push(".config");
            path.push(application);
            return Ok(path);
        }
    }

    return Err(ConfigError::UnsupportedPlatform);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub theme: String
}

impl Config {
    pub fn new() -> Self {
        Self {
            theme: DEFAULT.name.to_string()
        }
    }

    pub fn load() -> Result<Option<Self>, ConfigError> {
        let mut path = find_directory(APPLICATION)?;
        path.push("config.json");

        if path.exists() {
            match File::open(path) {
                Ok(file) => {
                    let reader = BufReader::new(file);

                    match from_reader(reader) {
                        Ok(config) => Ok(Some(config)),
                        Err(error) => Err(ConfigError::Decode(error))
                    }
                },
                Err(error) => {
                    Err(ConfigError::Read(error))
                }
            }
        } else {
            return Ok(None);
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        match find_directory(APPLICATION) {
            Ok(mut path) => {                
                if !path.exists() {
                    create_dir_all(path.clone())
                        .map_err(|_| ConfigError::CreateDirectory)?;
                }

                path.push("config.json");

                let writer = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)
                    .map_err(|error| ConfigError::Write(error))?;

                to_writer_pretty(writer, &self)
                    .map_err(|error| ConfigError::Encode(error))?;

                Ok(())
            },
            Err(error) => Err(error)
        }
    }
}