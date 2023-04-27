use crate::{
    database::{
        channels::{self, Channel, ChannelStates},
        users::{self, User, UserStates},
    },
    Context, Error,
};
use cuid2::cuid;
use poise::command;

/// Change state
#[command(slash_command, subcommands("channel", "user"), owners_only)]
pub async fn cs(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Change channel state
#[command(slash_command)]
pub async fn channel(
    ctx: Context<'_>,
    channel: poise::serenity_prelude::Channel,
    state: ChannelStates,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let res =
        if let Some(mut channel) = channels::get_by_discord_id(db, channel.id().0 as i64).await {
            channel.state = state;
            channels::change_state(db, &channel).await
        } else {
            channels::add_channel(
                db,
                &Channel {
                    id: 0,
                    discord_id: channel.id().0 as i64,
                    ccid: cuid(),
                    state,
                    system: None,
                },
            )
            .await
            .map(|_| ())
        };

    if let Err(e) = res {
        ctx.say(format!(":( Error: {}", e)).await?;
    } else {
        ctx.say(":)").await?;
    }

    Ok(())
}

/// Change user state
#[command(slash_command)]
pub async fn user(
    ctx: Context<'_>,
    user: poise::serenity_prelude::User,
    state: UserStates,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let res = if let Some(mut user) = users::get_by_discord_id(db, user.id.0 as i64).await {
        user.state = state;
        users::change_state(db, &user).await
    } else {
        users::add_user(
            db,
            &User {
                id: 0,
                discord_id: user.id.0 as i64,
                name: user.name,
                state,
            },
        )
        .await
        .map(|_| ())
    };

    if let Err(e) = res {
        ctx.say(format!(":( Error: {}", e)).await?;
    } else {
        ctx.say(":)").await?;
    }

    Ok(())
}
