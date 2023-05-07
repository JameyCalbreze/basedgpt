use std::ops::Deref;
use std::sync::Arc;
use async_openai::{
    Client as OpenAiClient
};
use async_openai::types::{CreateImageRequestArgs, ImageData, ImageSize, ResponseFormat};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::user::User;
use crate::util::openai_utils::remove_message_prefix;

pub async fn handle_image_command(ctx: &Context, msg: &Message) -> CommandResult {

    let prompt = remove_message_prefix(&msg.content);

    // Let's hardcode a test image command
    let openai_client = OpenAiClient::new();
    let response = openai_client.images().create(CreateImageRequestArgs::default()
        .prompt(prompt)
        .size(ImageSize::S512x512)
        .response_format(ResponseFormat::Url)
        .build()?).await?;

    let image_url = Arc::clone(&response.data[0]);
    let image_data = image_url.deref().clone();
    let url = match image_data {
        ImageData::Url(v) => {
            String::from(v.deref())
        }
        ImageData::B64Json(v) => { "".to_string() } // Hard coded for URLs
    };

    msg.reply(ctx, url).await?;

    Ok(())
}

pub enum ImageCommandSize {
    Small,
    Medium,
    Large,
}

impl ImageCommandSize {
    pub fn to_string(self) -> String {
        match self {
            ImageCommandSize::Small  => { String::from("256x256")   }
            ImageCommandSize::Medium => { String::from("512x512")   }
            ImageCommandSize::Large  => { String::from("1024x1024") }
        }
    }
}

struct ImageCommand {
    size: ImageCommandSize,
    prompt: String,
    num_images: usize,
    user: Option<User>
}

impl ImageCommand {
    pub fn default() -> ImageCommand {
        ImageCommand {
            size: ImageCommandSize::Small,
            prompt: "".to_string(),
            num_images: 1,
            user: None
        }
    }
}


