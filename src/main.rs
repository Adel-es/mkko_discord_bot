use std::sync::mpsc::{SyncSender, RecvError};
use std::thread;
use std::vec; 
use std::net::{TcpListener, TcpStream, Shutdown};
use serenity::futures::channel::mpsc::Receiver;
use serenity::{
    async_trait, 
    model::{
        channel::Message, 
        gateway::Ready,
        prelude::GuildId
    }, 
    prelude::*,
};
use serenity::client::bridge::gateway::GatewayIntents; 
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

mod connection;
mod discord_bot;
use std::str; 

struct Handler{
    is_first : AtomicBool, 
    receiver : Arc<tokio::sync::Mutex<mpsc::Receiver<String>>> 
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx : Context, _guilds: Vec<GuildId>){
        println!("Cache build successfully!"); 

        if !self.is_first.load(Ordering::Acquire){
            let arc_receiver = Arc::clone(&self.receiver); 
            tokio::spawn(handle_tcp_back(arc_receiver));           
        }
        self.is_first.swap(true, Ordering::Relaxed); 
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!test" {
            let value = discord_bot::auth::is_guild_user(947467280827154432, 538799426479849472).await;
            println!("{}", value); 
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn handle_tcp_back(receiver : Arc<tokio::sync::Mutex<mpsc::Receiver<String>>>  ){
    loop{
        let re = receiver.lock().await;
        let s = (*re).recv().unwrap();
        println!("{}", s); 
        let msg:connection::msg_struct::Message = serde_json::from_str(&s).unwrap_or(
            connection::msg_struct::Message::SendAuthCode{discord_id :538799426479849472, code : "fail".to_string() }
        ); 
        match msg {
            connection::msg_struct::Message::SendAuthCode{ discord_id, code } => {
                discord_bot::auth::send_direct_message(discord_id, &code).await.unwrap(); 
            }
            _ => {
                println!("no matching action in discord_bot"); 
            }
        } 
    }
}


fn handle_tcp_front(mut stream: TcpStream, sender : Arc<mpsc::SyncSender<String>>) {
    let mut data = [0 as u8; 256]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                println!("get messages"); 
                sender.send(str::from_utf8(&data[0..size]).unwrap().to_string()).unwrap(); 
                println!("send backend message")
            }
            true 
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn start_tcp_server(sender : Arc<SyncSender<String>>){
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let arc_sender = Arc::clone(&sender); 
                thread::spawn(move || {
                    handle_tcp_front(stream, arc_sender); 
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}


#[tokio::main]
async fn main() {

    // tcp handler back end 
    let ( backend_sender, backend_receiver) = mpsc::sync_channel(10); 
    let sender = Arc::new(backend_sender); 

    thread::spawn(move||{
        start_tcp_server(sender); 
    }); 

    //discord bot start 
    let token = discord_bot::config::DISCORD_BOT_TOKEN; 
    let mut client = Client::builder(token)
        .event_handler(Handler {
            is_first : AtomicBool::new(false), 
            receiver : Arc::new(tokio::sync::Mutex::new(backend_receiver)), 
        })
        .intents(GatewayIntents::all())
        .await
        .expect("[discord_bot] Error creating client"); 
    
    if let Err(why) = client.start().await {
        eprintln!("[discord_bot]  Client error: {:?}", why);
    }
}