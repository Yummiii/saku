use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption,
    },
};
use sqlx::{Pool, Sqlite};

use crate::database::{add_context, Message};


pub async fn run(options: &[CommandDataOption], db: &Pool<Sqlite>, channel: u64) -> String {
    let file = options
        .iter()
        .find(|option| option.name == "file")
        .and_then(|option| option.value.as_ref())
        .and_then(|value| value.as_str());

    let context = options
        .iter()
        .find(|option| option.name == "context")
        .and_then(|option| option.value.as_ref())
        .and_then(|value| value.as_str());

    if file.is_some() || context.is_some() {
        let txt = context.unwrap();
        add_context(&db, Message {
            channel: channel as i64,
            role: "user".to_string(),
            content: txt.to_string(),
        }).await;
        return "Contexto adicionado :)".to_string();
    } else {
        return "Arquivo ou texto pls".to_string();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ac")
        .description("Add to Context")
        .create_option(|option| {
            option
                .name("context")
                .description("Context to add")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("file")
                .description("Image to add to context")
                .kind(CommandOptionType::Attachment)
                .required(false)
        })
}
