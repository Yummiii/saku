use crate::{Context, Error, database::channels};
use poise::command;

/// Set system
#[command(slash_command, owners_only)]
pub async fn ss(ctx: Context<'_>, system: Option<String>) -> Result<(), Error> {
    let db = &ctx.data().db;
    if let Some(mut channel) = channels::get_by_discord_id(db, ctx.channel_id().0 as i64).await {
        channel.system = system;
        channels::set_system(db, &channel).await?;
        ctx.say("System updated :)").await?;
    } else {
        ctx.say(":(").await?;
    }
    Ok(())
}
