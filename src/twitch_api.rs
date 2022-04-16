use std::{
    error, 
    sync::atomic::{ 
        AtomicBool, Ordering 
    } 
};

use serde::{ Deserialize, Serialize };

use super::error::Error;
use super::config;

const STREAMS_URI: &str = "https://api.twitch.tv/helix/streams?";
const TOKEN_URI: &str = "https://id.twitch.tv/oauth2/token?";

//static mut  GLOBAL_STREAM_STATUS: AtomicBool = AtomicBool::new(false);

pub struct TwitchAPI {
    config: config::TwitchConfig,

    pub stream_status: AtomicBool,
}

impl TwitchAPI {
    pub fn new(config: config::TwitchConfig) -> Result<TwitchAPI, Box<dyn error::Error>> {
        Ok(
            TwitchAPI {
                config,
                stream_status: AtomicBool::new(false),
            }
        )
    }

    /*
            tokio::spawn(
            async move {
                let interval = tokio::time::interval(tokio::time::Duration::from_secs(900));

                
            }
        );
    */
    
    // pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
    //     self.config.save()
    // }

    /*
    is_live takes a user name for twitch and returns if they are currently live

    returns an error if the twitch api call fails
    */
    pub async fn is_live(&mut self, user: &str) -> Result<bool, Box<dyn error::Error>> {
        let res = reqwest::Client::new().get(STREAMS_URI.to_owned() + "user_login=" + user)
            .header("Authorization", "Bearer ".to_string() + &self.config.token)
            .header("Client-Id",&self.config.id)
            .send()
            .await?
            .text()
            .await?;

        #[derive(Serialize, Deserialize, Debug)]
        struct Streams {
            data: Vec<Stream>,
            pagination: Pagination,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Pagination {}

        #[derive(Serialize, Deserialize, Debug)]
        struct Stream {
            id: String,
            user_id: String,
            user_login: String,
            user_name: String,
            game_id: String,
            game_name: String,
            r#type: String,
            title: String,
            viewer_count: i32,
            started_at: String,
            language: String,
            thumbnail_url: String,
            tag_ids: Vec<String>,
            is_mature: bool,
        }

        match serde_json::from_str::<Streams>(&res) {
            Ok(data) => {
                if data.data.len() > 0 {
                    if !self.stream_status.load(Ordering::SeqCst) {
                        *self.stream_status.get_mut() = true;  
                    }
                    return Ok(true)
                }
                return Ok(false)
            },
            Err(_) => {
                #[derive(Serialize, Deserialize, Debug)]
                struct ErrorData {
                    error: String,
                    status: u16,
                    message: String,
                }

                let err: ErrorData = serde_json::from_str(&res)?;
                if err.status == 401 {
                    let res = reqwest::Client::new()
                        .post(TOKEN_URI.to_owned() + "client_id=" 
                            + &self.config.id 
                            + "&client_secret=" 
                            + &self.config.secret 
                            + "&grant_type=client_credentials")
                    .send()
                    .await?
                    .text()
                    .await?;

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Token {
                        access_token: String,
                        expires_in: i32,
                        token_type: String,
                    }

                    self.config.token = serde_json::from_str::<Token>(&res)?.access_token;

                    return Err(Error::new(format!("OAUTH was out of date!")))
                } else {
                    return Err(Error::new(format!("Error when making twitch api request: {:?}", err)))
                }
            }
        }
    }
}

