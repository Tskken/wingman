use std::{
    fs, error,
};

use serde::{ Deserialize, Serialize };
use toml;

use super::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_config: DiscordConfig,
    pub twitch_config: TwitchConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitchConfig {
    pub id: String,
    pub secret: String,
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn error::Error>> {
        let content = match fs::read_to_string("config.toml") {
            Ok(v) => v,
            Err(err) => {
                return Err(Error::new(format!("Could not load file for: {:?}", err)));
            }
        };
    
        let config: Config = match toml::from_str(&content) {
            Ok(c) => c,
            Err(err) => {
                return Err(Error::new(format!("Could not load data to config for: {:?}", err)));
            }
        };

        Ok(config)
    }

    pub fn get_twitch_config(&self) -> TwitchConfig {
        self.get_twitch_config().clone()
    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        let content = match toml::to_string(&self) {
            Ok(c) => c,
            Err(err) => {
                return Err(Error::new(format!("Could not send config to toml str: {:?}", err)));
            }
        };

        fs::write("config.toml", content)?;

        Ok(())
    }
}