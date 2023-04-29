use crate::{
    database::{
        usage::{self, Usage},
        users,
    },
    models::Models,
    Context, Error,
};
use chrono::Datelike;
use itertools::Itertools;
use poise::{command, serenity_prelude};
use serde::Deserialize;

///Usage log
#[command(slash_command)]
pub async fn ul(
    ctx: Context<'_>,
    user: Option<serenity_prelude::User>,
    #[min = 1]
    #[max = 12]
    month: Option<u32>,
) -> Result<(), Error> {
    let db = &ctx.data().db;
    let rates_url = &ctx.data().rates_url;

    let month = month.unwrap_or(chrono::Utc::now().month()) as i32;
    let usages = if let Some(user) = user {
        let user = users::get_by_discord_id(db, user.id.0 as i64)
            .await
            .unwrap();
        usage::get_user_usage_from_month(db, user.id, month)
            .await
            .unwrap()
    } else {
        usage::get_usage_from_month(db, month).await.unwrap()
    };

    let usages = usages
        .into_iter()
        .into_group_map_by(|x| x.user)
        .into_iter()
        .collect::<Vec<(i64, Vec<Usage>)>>();

    let mut msg = String::new();
    // println!("{:#?}", usages);

    for (user, usages) in usages {
        let user = users::get_by_id(db, user).await.unwrap();
        let mut total = 0.0;
        let mut total_tokens = 0;

        for usage in usages {
            let price = if usage.model == Models::Gpt4 {
                let price_prompt = usage.prompt_tokens as f32 * 0.000030;
                let price_completion = usage.completion_tokens as f32 * 0.000060;
                convert(price_prompt + price_completion, rates_url).await
            } else {
                convert(
                    (usage.prompt_tokens + usage.completion_tokens) as f32 * 0.000002,
                    rates_url,
                )
                .await
            };

            total_tokens += usage.prompt_tokens + usage.completion_tokens;
            total += price * usage.multiplier.unwrap_or(1.);
        }

        msg += &format!("{} [{}] = R${} ({})\n\n", user.name, user.discord_id, total, total_tokens);
    }

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("Usage log for {}", month));
            e.description(msg);
            e.color(0x660066)
        })
    }).await?;

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

async fn convert(usd: f32, rates_url: &String) -> f32 {
    let response: Exchange = reqwest::get(rates_url).await.unwrap().json().await.unwrap();
    usd * response.rates.brl

    // let total_prompt = usage.iter().fold(0, |acc, x| acc + x.prompt_tokens);
    // let prompt_price = total_prompt as f32 * 0.000030;

    // let total_completion = usage.iter().fold(0, |acc, x| acc + x.completion_tokens);
    // let completion_price = total_completion as f32 * 0.000060;

    // (
    //     total_prompt,
    //     prompt_price * response.rates.brl,
    //     total_completion,
    //     completion_price * response.rates.brl,
    // )
}
