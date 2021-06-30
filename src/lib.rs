use serde::Deserialize;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum KimaiError {
    XdgBaseDirectories(String),
    IO(String),
    Toml(String),
    Utf8(String),
    Config(String),
    Other(String),
}

impl std::error::Error for KimaiError {}

impl fmt::Display for KimaiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KimaiError::XdgBaseDirectories(e) => write!(f, "XDG BaseDirectories Error: {}", e),
            KimaiError::IO(e) => write!(f, "IO Error: {}", e),
            KimaiError::Toml(e) => write!(f, "TOML Error: {}", e),
            KimaiError::Utf8(e) => write!(f, "UTF-8 Error: {}", e),
            KimaiError::Config(e) => write!(f, "Config Error: {}", e),
            KimaiError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<xdg::BaseDirectoriesError> for KimaiError {
    fn from(error: xdg::BaseDirectoriesError) -> KimaiError {
        KimaiError::XdgBaseDirectories(error.to_string())
    }
}

impl From<std::io::Error> for KimaiError {
    fn from(error: std::io::Error) -> KimaiError {
        KimaiError::IO(error.to_string())
    }
}

impl From<toml::de::Error> for KimaiError {
    fn from(error: toml::de::Error) -> KimaiError {
        KimaiError::Toml(error.to_string())
    }
}

impl From<std::str::Utf8Error> for KimaiError {
    fn from(error: std::str::Utf8Error) -> KimaiError {
        KimaiError::Utf8(error.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    host: String,
    user: String,
    password: Option<String>,
    pass_path: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    host: String,
    user: String,
    password: String,
}

impl Config {
    pub fn new(host: String, user: String, password: String) -> Self {
        Config {
            host,
            user,
            password,
        }
    }
    pub fn from_path(path: &Path) -> Result<Self, KimaiError> {
        let config_string = fs::read_to_string(path)?;
        let config_file = toml::from_str::<ConfigFile>(&config_string)?;
        if let Some(p) = config_file.password {
            Ok(Config {
                host: config_file.host,
                user: config_file.user,
                password: p,
            })
        } else if let Some(p) = config_file.pass_path {
            let pass_cmd = Command::new("pass").arg(p).output()?;
            Ok(Config {
                host: config_file.host,
                user: config_file.user,
                password: std::str::from_utf8(&pass_cmd.stdout)?.trim().into(),
            })
        } else {
            Err(KimaiError::Config(
                "No password give in config!".to_string(),
            ))
        }
    }

    pub fn from_xdg() -> Result<Self, KimaiError> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("kimai")?;
        let config_path = xdg_dirs
            .find_config_file("config.toml")
            .ok_or_else(|| KimaiError::Config("config file not found!".to_string()))?;
        Self::from_path(Path::new(&config_path))
    }
}

pub fn main() {
    println!("{:#?}", Config::from_xdg());
}
