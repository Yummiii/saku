use crate::database::{
    channel_exists, create_channel, is_channel_enabled, set_channel_state, Channel,
};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::CommandDataOption,
    },
};
use sqlx::{Pool, Sqlite};

pub async fn run(options: &[CommandDataOption], db: &Pool<Sqlite>, channel: u64) -> String {
    let state = options[0].value.as_ref().unwrap().as_bool().unwrap();
    let channel = Channel {
        enabled: state,
        id: channel as i64,
    };

    if channel_exists(&db, channel.id).await {
        set_channel_state(&db, &channel).await;
    } else {
        create_channel(&db, &channel).await;
    }

    if is_channel_enabled(&db, channel.id).await {
        "Channel enabled".to_string()
    } else {
        "Channel disabled".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cs")
        .description("Change the channel state")
        .create_option(|option| {
            option
                .name("enabled")
                .description("The state of the channel")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}