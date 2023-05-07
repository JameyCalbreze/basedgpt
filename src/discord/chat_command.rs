use crate::util;
use async_openai::Client as OpenAiClient;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use log::{info};

use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    Role,
};
use util::constants::CHAT_MODEL_GPT_3_5_TURBO;

pub async fn handle_chat_gpt_command(ctx: &Context, msg: &Message) -> CommandResult {
    info!("handle_chat_gpt_command");
    println!("Entered chatgpt!");
    // Now we want to take the message as is and just send it back
    // to the server to see what the content is
    let ai_client: OpenAiClient = OpenAiClient::new();

    let request = convert_message_to_chat_command(msg);

    let chat_response = ai_client.chat().create(request).await;

    match chat_response {
        Ok(response) => {
            for choice in response.choices {
                println!(
                    "{}: Role: {}  Content: {}",
                    choice.index, choice.message.role, choice.message.content
                );
                msg.reply(ctx, &choice.message.content).await?;
            }
        }
        Err(e) => {
            println!("Error Calling ChatGPT: {}", e.to_string());
        }
    };

    Ok(())
}

/*
    Take in a discord message which is meant to kick off a chat gpt request and
    create the async openai request to be sent off
*/
pub fn convert_message_to_chat_command(message: &Message) -> CreateChatCompletionRequest {
    const MAX_TOKENS: u16 = 512;
    let request = CreateChatCompletionRequestArgs::default()
        .model(CHAT_MODEL_GPT_3_5_TURBO)
        .max_tokens(MAX_TOKENS) // Arbitrary limit
        .messages([ChatCompletionRequestMessage {
            role: Role::User, // This represents a user of my discord bot. Are they an admin? or User?
            content: util::openai_utils::remove_message_prefix(&message.content),
            name: Some(String::from(&message.author.name)),
        }])
        .build();

    match request {
        Ok(r) => { r }
        Err(e) => {
            println!("ChatHandler: {}", e.to_string());
            CreateChatCompletionRequest::default()
        }
    }
}
