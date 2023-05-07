mod configuration;
mod discord;
mod util;

use std::env;

use crate::discord::chat_command::handle_chat_gpt_command;
use crate::discord::image_command::handle_image_command;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

// There's some sort of annotation processor which is linking together all of the
// individual commands here instead of me having to explicitly call everything
#[group]
#[commands(ping, chat, image)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    configuration::logging::init_logger();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("based ")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Entered ping");
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn chat(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Entered ChatGPT!");
    handle_chat_gpt_command(ctx, msg).await
}

#[command]
async fn image(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Entered ImageGPT");
    handle_image_command(ctx, msg).await
}
