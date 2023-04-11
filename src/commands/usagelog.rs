use crate::{
    database::{
        usage::{self, Usage},
        users,
    },
    Context, Error,
};
use chrono::Datelike;
use itertools::Itertools;
use poise::{command, serenity_prelude};

///Usage log
#[command(slash_command)]
pub async fn ul(ctx: Context<'_>, user: Option<serenity_prelude::User>) -> Result<(), Error> {
    let db = &ctx.data().db;
    let month = chrono::Utc::now().month() as i32;

    if let Some(user) = user {
        let user = users::get_by_discord_id(db, user.id.0 as i64)
            .await
            .unwrap();

        let usage = usage::get_user_usage_from_month(db, user.id, month)
            .await
            .unwrap();

        let (total_prompt, prompt_price, total_completion, completion_price) = calc(usage);

        ctx.say(format!(
            "<@{}> já usou {} tokens e deve ${}",
            user.discord_id,
            total_prompt + total_completion,
            prompt_price + completion_price
        ))
        .await?;
    } else {
        let usage = usage::get_usage_from_month(db, month).await.unwrap();
        let usage_grouped = usage
            .into_iter()
            .group_by(|x| x.user)
            .into_iter()
            .map(|(user, group)| {
                let values: Vec<usage::Usage> = group.collect();
                (user, values)
            })
            .collect::<Vec<(i64, Vec<usage::Usage>)>>();

        let mut msg = String::new();
        for user_usage in usage_grouped {
            let (total_prompt, prompt_price, total_completion, completion_price) =
                calc(user_usage.1);
            let user = users::get_by_id(db, user_usage.0).await.unwrap();

            msg += &format!(
                "<@{}>: {} tokens, ${}\n\n",
                user.discord_id,
                total_prompt + total_completion,
                prompt_price + completion_price
            );
        }
        ctx.say(msg).await?;
    }

    Ok(())
}

fn calc(usage: Vec<Usage>) -> (i32, f32, i32, f32) {
    let total_prompt = usage.iter().fold(0, |acc, x| acc + x.prompt_tokens);
    let prompt_price = total_prompt as f32 * 0.000030;

    let total_completion = usage.iter().fold(0, |acc, x| acc + x.completion_tokens);
    let completion_price = total_completion as f32 * 0.000060;

    (
        total_prompt,
        prompt_price,
        total_completion,
        completion_price,
    )
}
