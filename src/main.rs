use chat::create_completion;
use commands::get_commands;
use configs::Configs;
use database::{channels::Channel, users::User, Database};
use openai::set_key;
use poise::{
    serenity_prelude::{GatewayIntents, UserId},
    Event, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use crate::database::{
    channels::{self, ChannelStates},
    users::{self, UserStates},
};

mod chat;
mod commands;
mod configs;
mod database;

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
                                    };
                                    channel.id = channels::add_channel(db, &channel).await.unwrap();
                                    channel
                                };

                                let should_reply = {
                                    if msg.is_private() {
                                        user.state == UserStates::DmEnabled
                                    } else {
                                        channel.state == ChannelStates::Enabled
                                            && user.state != UserStates::Blocked
                                    }
                                };

                                if should_reply {
                                    let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

                                    let chat_completion =
                                        create_completion(msg.content.clone(), user, channel, db)
                                            .await;
                                    if let Ok(chat_completion) = chat_completion {
                                        msg.channel_id
                                            .say(&ctx.http, chat_completion)
                                            .await
                                            .unwrap();
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
