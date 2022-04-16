mod twitch_api;
pub mod error;
mod config;
mod discord;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use std::sync::atomic::{ 
    AtomicBool, Ordering 
};

pub struct Wingman {
    running: bool,

    pub discord_bot: discord::DiscordBot,
    pub twitch_api: twitch_api::TwitchAPI,

    tx: Sender<bool>,
    rx: Receiver<bool>,
}

impl Wingman {
    pub async fn new() -> Result<Wingman, Box<dyn std::error::Error>> {
        let config = config::Config::load()?;

        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        let wingman = Wingman {
            running: false,
            discord_bot: discord::DiscordBot::new(config.discord_config.clone()).await?,
            twitch_api: twitch_api::TwitchAPI::new(config.twitch_config.clone())?,
            tx,
            rx,
        };
        
        Ok(wingman)
    }

    pub async fn run(&mut self) -> Result<(), Box< dyn std::error::Error>> {
        self.discord_bot.run().await?;

        Ok(())
    }

    pub async fn is_live(&mut self, user: &str)  -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.twitch_api.is_live(&user).await?)
    }
}

impl Drop for Wingman {
    fn drop(&mut self) {
        //self.twitch_api.save().expect("Could not save twitch_api config to file!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_config() {}
}
