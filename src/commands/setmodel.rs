use crate::{database::{channels, contexts}, models::Models, Context, Error};
use cuid2::cuid;
use poise::command;

/// Set model
#[command(slash_command)]
pub async fn sm(ctx: Context<'_>, model: Models) -> Result<(), Error> {
    let db = &ctx.data().db;
    if let Some(mut channel) = channels::get_by_discord_id(db, ctx.channel_id().0 as i64).await {
        channel.model = model;
        channel.ccid = cuid();
        channels::set_model(db, &channel).await?;
        channels::set_ccid(db, &channel).await?;
        contexts::update_cid_for_channel(db, channel.id, &channel.ccid).await?;
        ctx.say("Model updated :)").await?;
    } else {
        ctx.say(":(").await?;
    }
    Ok(())
}
