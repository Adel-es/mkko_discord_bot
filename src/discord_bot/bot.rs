use serenity::{
    async_trait, 
    model::{
        channel::Message, 
        gateway::Ready,
    }, 
    prelude::*,
};
use serenity::client::bridge::gateway::GatewayIntents; 
use std::env; 
use super::auth; 
use super::config; 

pub struct Handler; 

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!test" {
            let value = auth::is_guild_user(947467280827154432, 538799426479849472).await;
            println!("{}", value); 
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


/* pub fn send_direct_message(dicord_user_id : String, code : u64){

} */ 

pub async fn start_discord_bot() {
    let token = config::DISCORD_BOT_TOKEN; 
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("[discord_bot] Error creating client"); 
    
    if let Err(why) = client.start().await {
        eprintln!("[discord_bot]  Client error: {:?}", why);
    }
}