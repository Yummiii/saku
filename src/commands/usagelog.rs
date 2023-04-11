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
use serde::Deserialize;

///Usage log
#[command(slash_command)]
pub async fn ul(ctx: Context<'_>, user: Option<serenity_prelude::User>) -> Result<(), Error> {
    let db = &ctx.data().db;
    let rates_url = &ctx.data().rates_url;

    let month = chrono::Utc::now().month() as i32;

    if let Some(user) = user {
        let user = users::get_by_discord_id(db, user.id.0 as i64)
            .await
            .unwrap();

        let usage = usage::get_user_usage_from_month(db, user.id, month)
            .await
            .unwrap();

        let (total_prompt, prompt_price, total_completion, completion_price) =
            calc(usage, rates_url).await;

        ctx.say(format!(
            "<@{}> já usou {} tokens e deve R${}",
            user.discord_id,
            total_prompt + total_completion,
            prompt_price + completion_price
        ))
        .await?;
    } else {
        let usage = usage::get_usage_from_month(db, month).await.unwrap();

        let usage_grouped = usage
            .into_iter()
            .into_group_map_by(|x| x.user)
            .into_iter()
            .collect::<Vec<(i64, Vec<usage::Usage>)>>();

        let mut msg = String::new();
        for user_usage in usage_grouped {
            let (total_prompt, prompt_price, total_completion, completion_price) =
                calc(user_usage.1, rates_url).await;
            let user = users::get_by_id(db, user_usage.0).await.unwrap();

            msg += &format!(
                "<@{}>: {} tokens, R${}\n\n",
                user.discord_id,
                total_prompt + total_completion,
                prompt_price + completion_price
            );
        }
        ctx.say(msg).await?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct Exchange {
    #[serde(rename = "conversion_rates")]
    pub rates: ConversionRates,
}

#[derive(Deserialize)]
struct ConversionRates {
    #[serde(rename = "BRL")]
    pub brl: f32,
}

async fn calc(usage: Vec<Usage>, rates_url: &String) -> (i32, f32, i32, f32) {
    let response: Exchange = reqwest::get(rates_url).await.unwrap().json().await.unwrap();

    let total_prompt = usage.iter().fold(0, |acc, x| acc + x.prompt_tokens);
    let prompt_price = total_prompt as f32 * 0.000030;

    let total_completion = usage.iter().fold(0, |acc, x| acc + x.completion_tokens);
    let completion_price = total_completion as f32 * 0.000060;

    (
        total_prompt,
        prompt_price * response.rates.brl,
        total_completion,
        completion_price * response.rates.brl,
    )
}
