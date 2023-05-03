use crate::{
    commands::allowuser::autocomplete_virtual_user,
    database::{
        channels::{self, ChannelStates},
        users,
        virtualusers::{self, VirtualUserRoles},
    },
    models::Models,
    Context, Error,
};
use cuid2::cuid;
use poise::command;

///Set virtual
#[command(slash_command)]
pub async fn sv(
    ctx: Context<'_>,
    #[rename = "virtual"]
    #[autocomplete = "autocomplete_virtual_user"]
    vu: i64,
    channel: poise::serenity_prelude::Channel,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    if let Some(author) = users::get_by_discord_id(db, ctx.author().id.0 as i64).await {
        let admins = virtualusers::get_virtual_by_role(db, vu, VirtualUserRoles::Admin).await?;
        if admins.iter().any(|x| x.user_id == author.id) {
            let channel = if let Some(channel) =
                channels::get_by_discord_id(db, channel.id().0 as i64).await
            {
                channel.id
            } else {
                channels::add_channel(
                    db,
                    &channels::Channel {
                        id: 0,
                        discord_id: channel.id().0 as i64,
                        ccid: cuid(),
                        state: ChannelStates::Disabled,
                        system: None,
                        model: Models::Gpt3,
                        virtual_user: None,
                    },
                )
                .await?
            };

            channels::set_virtual_user(db, channel, vu).await?;
            ctx.say(":)").await?;
        } else {
            ctx.say(":(").await?;
        }
    } else {
        ctx.say(":(").await?;
    }

    Ok(())
}
