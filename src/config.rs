use serde::Deserialize;
use std::{error::Error, fs};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub webhook_url: String,
    pub username: String,
    pub avatar_url: String,
    pub color: u32,
}

pub fn get_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&config_str)?;
    return Ok(config);
}
