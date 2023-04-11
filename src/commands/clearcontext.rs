use cuid2::cuid;
use poise::command;

use crate::{Context, Error, database::{contexts, channels}};

///Clear context
#[command(slash_command)]
pub async fn cc(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;
    if let Some(mut channel) = channels::get_by_discord_id(db, ctx.channel_id().0 as i64).await {
        channel.ccid = cuid();
        channels::set_ccid(db, &channel).await?;
        contexts::deactivate_channel_context(db, channel.id).await?;
        ctx.say("Context cleared :)").await?;
    } else {
        ctx.say(":(").await?;
    }
    Ok(())
}