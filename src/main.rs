use database::is_channel_enabled;
use dotenv::dotenv;
use openai::set_key;
use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{Interaction, InteractionResponseType},
        GuildId, Message, Ready, UserId,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::env;

mod chat;
mod commands;
mod database;

struct Handler {
    db: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "cc" => {
                    commands::clearcontext::run(
                        &command.data.options,
                        &self.db,
                        command.channel_id.0,
                    )
                    .await
                }
                "ac" => {
                    commands::addcontext::run(&command.data.options, &self.db, command.channel_id.0)
                        .await
                }
                "cs" => {
                    commands::channelstate::run(
                        &command.data.options,
                        &self.db,
                        command.channel_id.0,
                    )
                    .await
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::clearcontext::register(command))
                .create_application_command(|command| commands::addcontext::register(command))
                .create_application_command(|command| commands::channelstate::register(command))
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::clearcontext::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::addcontext::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::channelstate::register(command)
        })
        .await
        .unwrap();
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == UserId(368280970102833153) && msg.content != "" {
            if is_channel_enabled(&self.db, msg.channel_id.0 as i64).await || msg.is_private() {
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

                let response =
                    chat::create_completion(&msg.content, &self.db, msg.channel_id.0).await;
                if let Ok(response) = response {
                    for response in split_string(response, 2000) {
                        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                } else {
                    msg.reply(&ctx.http, "Alguma coisa explodiu :(")
                        .await
                        .unwrap();
                }
                typing.stop().unwrap();
            }
        }
    }
}

fn split_string(input: String, max_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for word in input.split_whitespace() {
        if current.len() + word.len() + 1 > max_length {
            if current.is_empty() {
                current.push_str(&word[..max_length]);
                result.push(current);
                current = String::from(&word[max_length..]);
            } else {
                result.push(current.trim_end().to_string());
                current.clear();
                current.push_str(word);
                current.push(' ');
            }
        } else {
            current.push_str(word);
            current.push(' ');
        }
    }

    if !current.trim_end().is_empty() {
        result.push(current.trim_end().to_string());
    }

    result
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    set_key(env::var("OPENAI_KEY").expect("Expected OPENAI_KEY in environment"));
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    migrate!("./migrations")
        .run(&db)
        .await
        .expect("Couldn't run migrations");

    let mut client = Client::builder(
        token,
        GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_MESSAGES,
    )
    .event_handler(Handler { db })
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
