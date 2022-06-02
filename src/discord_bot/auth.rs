use serenity::http::client::Http; 
use serenity::model::id::{GuildId, UserId}; 
use serenity::futures::StreamExt;
use super::config; 

pub async fn is_guild_user(guild_id : u64, user_id : u64) -> bool{

    let token = config::DISCORD_BOT_TOKEN; 
    let ctx = Http::new_with_token(&token); 

    let gid = GuildId(guild_id); 
    let mut members = gid.members_iter(&ctx).boxed(); 
    while let Some(member_result) = members.next().await {
        if let Ok(member) = member_result{
            if member.user.id == user_id {
                return true; 
            }
        }
    }
    false 
}

pub async fn send_direct_message(user_id : u64, code : &str) -> Result<(), &str>{
    let token = config::DISCORD_BOT_TOKEN; 
    let ctx = Http::new_with_token(&token); 

    let content = format!("verify code : [ {} ], write this code to website", code); 
    let user = UserId(user_id).to_user(&ctx).await.unwrap(); 
    let dm_result = user.direct_message(&ctx, |m| m.content(content)).await;  
    match dm_result {
        Ok(_) => Ok(()), 
        Err(_) => Err("[discord_bot] fail to send code") 
    }
}