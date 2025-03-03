use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ssh: Option<SshSection>,
}

#[derive(Debug, Deserialize)]
pub struct SshSection {
    pub derive: Option<DeriveSection>,
    #[serde(flatten)]
    pub servers: HashMap<String, SshServer>,
}

#[derive(Debug, Deserialize)]
pub struct DeriveSection {
    pub appwrite: Option<AppwriteServers>,
}

#[derive(Debug, Deserialize)]
pub struct AppwriteServers {
    #[serde(flatten)]
    pub servers: HashMap<String, AppwriteSshServer>,
}

#[derive(Debug, Deserialize)]
pub struct AppwriteSshServer {
    pub get: Option<String>,
    pub x_appwrite_project: Option<String>,
    pub x_appwrite_key: Option<String>,
    pub port: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SshServer {
    pub ip: Option<String>,
    pub port: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut path = dirs::home_dir().ok_or("failed to get home directory")?;
    path.push(".config/evelynn-cli/config.toml");

    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
