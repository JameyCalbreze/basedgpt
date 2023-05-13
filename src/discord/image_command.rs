use crate::util::openai_utils::remove_message_prefix;
use async_openai::types::{CreateImageRequestArgs, ImageData, ImageSize, ResponseFormat};
use async_openai::Client as OpenAiClient;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use std::ops::Deref;
use std::sync::Arc;

pub async fn handle_image_command(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = remove_message_prefix(&msg.content);

    let openai_client = OpenAiClient::new();
    let response = openai_client
        .images()
        .create(
            CreateImageRequestArgs::default()
                // Tracking the discord user id for moderation purposes
                .user(msg.author.id.to_string())
                .prompt(prompt)
                .size(ImageSize::S512x512)
                .response_format(ResponseFormat::Url)
                .build()?,
        )
        .await;

    match response {
        Ok(i) => {
            log::info!("Call to create image succeeded");
            let image_url = Arc::clone(&i.data[0]);
            let image_data = image_url.deref().clone();
            let url = match image_data {
                ImageData::Url(v) => String::from(v.deref()),
                ImageData::B64Json(v) => "".to_string(), // Not Supported
            };
            msg.reply(ctx, url).await?;
            Ok(())
        }
        Err(e) => {
            log::error!("Call to create image failed with {}", e.to_string());
            let mut error_response = String::from("CreateImage Err: ");
            error_response.push_str(&e.to_string());
            msg.reply(ctx, &error_response).await?;
            Ok(())
        }
    }
}
