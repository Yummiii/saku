use crate::database::{
    channels::{self, ChannelStates},
    users::{self, UserStates},
};
use chat::create_completion;
use commands::get_commands;
use configs::Configs;
use cuid2::cuid;
use database::{channels::Channel, users::User, Database};
use models::Models;
use openai::set_key;
use poise::{
    serenity_prelude::{GatewayIntents, UserId},
    Event, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

mod chat;
mod commands;
mod configs;
mod database;
mod models;

pub struct Data {
    db: Database,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let configs = Configs::new();
    let db = Database::new(&configs.database_url).await;
    db.migrate().await;
    set_key(configs.openai_key);

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: get_commands(),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("'".to_string()),
                ..Default::default()
            },
            owners: vec![UserId(configs.owner_id)].into_iter().collect(),
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        Event::Message { new_message: msg } => {
                            if !msg.author.bot
                                && !(msg.content.starts_with("~~") && msg.content.ends_with("~~"))
                                && !msg.content.starts_with("> ")
                            {
                                let db = &data.db;
                                let user = if let Some(user) =
                                    users::get_by_discord_id(&db, msg.author.id.0 as i64).await
                                {
                                    user
                                } else {
                                    let mut user = User {
                                        id: 0,
                                        discord_id: msg.author.id.0 as i64,
                                        name: msg.author.name.clone(),
                                        state: UserStates::Normal,
                                        virtal: false,
                                    };
                                    user.id = users::add_user(db, &user).await.unwrap();
                                    user
                                };

                                let channel = if let Some(channel) =
                                    channels::get_by_discord_id(&db, msg.channel_id.0 as i64).await
                                {
                                    channel
                                } else {
                                    let mut channel = Channel {
                                        id: 0,
                                        discord_id: msg.channel_id.0 as i64,
                                        state: ChannelStates::Disabled,
                                        ccid: cuid(),
                                        system: None,
                                        model: Models::Gpt3,
                                        virtual_user: None,
                                    };
                                    channel.id = channels::add_channel(db, &channel).await.unwrap();
                                    channel
                                };

                                let should_reply = {
                                    if msg.is_private() {
                                        user.state >= UserStates::DmEnabled
                                    } else {
                                        (channel.state == ChannelStates::Enabled
                                            || channel.state == ChannelStates::NoLogs)
                                            && user.state != UserStates::Blocked
                                    }
                                };

                                if should_reply {
                                    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

                                    let chat_completion =
                                        create_completion(msg.content.clone(), user, channel, db)
                                            .await;
                                    if let Ok(chat_completion) = chat_completion {
                                        for response in split_string(chat_completion, 2000) {
                                            msg.channel_id.say(&ctx.http, response).await.unwrap();
                                        }
                                    } else {
                                        let err = chat_completion.unwrap_err();
                                        msg.reply(&ctx.http, format!(":( {}", err)).await.unwrap();
                                    }

                                    typing.stop().unwrap();
                                }
                            }
                        }
                        _ => {}
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(configs.token)
        .intents(
            GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data { db }) }));

    framework.run().await.unwrap();
}

//eu n faço ideia de como isso funciona, foi o GPT-4 que fez
fn split_string(input: String, max_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut start_index = 0;
    while start_index < input.len() {
        let mut end_index = start_index + max_length;
        if end_index >= input.len() {
            end_index = input.len();
        } else {
            if let Some(last_space) = input[..end_index].rfind(' ') {
                end_index = last_space;
            }
        }
        let chunk = &input[start_index..end_index];
        result.push(chunk.to_string());
        start_index = end_index + 1;
    }
    result
}
