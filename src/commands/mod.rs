use crate::{Context, Data, Error};
use std::error::Error as StdError;

mod changestate;
mod clearcontext;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub fn get_commands() -> Vec<poise::Command<Data, Box<(dyn StdError + Send + Sync + 'static)>>> {
    vec![register(), changestate::cs(), clearcontext::cc()]
}
