use std::{collections::HashSet, sync::Arc};

use serenity::{
    client,
    async_trait,
    framework::{standard::macros::{ group, command }, StandardFramework, standard::CommandResult},
    http::Http,
    model::{prelude::*, event::ResumedEvent, gateway::Ready, id::UserId},
    prelude::*,
};

use tracing::{error, info};

use super::config;
use super::error::Error;

pub struct DiscordBot {
    config: config::DiscordConfig,

    pub client: client::Client,

    bot_id: UserId,
    //owners: HashSet<UserId>,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _:Context, ready: Ready) {
        info!("Connectd as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

impl DiscordBot {
    pub async fn new(config: config::DiscordConfig) -> Result<DiscordBot, Box<dyn std::error::Error> >{
        let http = Http::new_with_token(&config.token);
        let mut owners: HashSet<UserId> = HashSet::new();
        let bot_id: UserId;

        match http.get_current_application_info().await {
            Ok(info) => {
                owners.insert(info.owner.id);
                bot_id = info.id;
            },
            Err(why) => {
                return Err(Error::new(format!("Could not access application info: {:?}", why)));
            }
        };

        let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix("~")).group(&GENERAL_GROUP);

        let client = Client::builder(&config.token)
            .framework(framework)
            .event_handler(Handler)
            .await?;

        Ok(DiscordBot {
            config,
            client,
            bot_id,
            //owners,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(why) = self.client.start().await {
            return Err(Error::new(format!("Error starting client!: {:?}", why)))
        }

        Ok(())
    }
}
