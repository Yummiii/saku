use crate::database::clear_channel_context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};
use sqlx::{Pool, Sqlite};

pub async fn run(_options: &[CommandDataOption], db: &Pool<Sqlite>, channel: u64) -> String {
    clear_channel_context(&db, channel as i64).await;
    "Contexto limpo :)".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("cc").description("Clear Context")
}
