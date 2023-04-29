use std::collections::HashMap;

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
use poise::{
    command,
    serenity_prelude::{self, Activity},
};
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
    ctx.defer().await?;
    let db = &ctx.data().db;

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
    let dol_price = get_price().await;

    let mut global_price = 0.0;
    let mut global_tokens = 0;

    for (user, usages) in usages {
        let user = users::get_by_id(db, user).await.unwrap();
        let mut total = 0.0;
        let mut total_tokens = 0;

        for usage in usages {
            let price = if usage.model == Models::Gpt4 {
                let price_prompt = usage.prompt_tokens as f32 * 0.000030;
                let price_completion = usage.completion_tokens as f32 * 0.000060;
                price_prompt + price_completion
            } else {
                (usage.prompt_tokens + usage.completion_tokens) as f32 * 0.000002
            };

            total_tokens += usage.prompt_tokens + usage.completion_tokens;
            total += (price * usage.multiplier.unwrap_or(1.)) * dol_price;
        }

        global_price += total;
        global_tokens += total_tokens;

        msg += &format!(
            "**{}: R${}** ({}) [{}]\n\n",
            user.name, total, total_tokens, user.discord_id
        );
    }

    ctx.serenity_context()
        .set_activity(Activity::watching(&format!(
            "Pessoas gastando R${} | {} tokens",
            global_price.ceil(), global_tokens
        )))
        .await;

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("Usage log for {}", month));
            e.description(msg);
            e.color(0x660066)
        })
    })
    .await
    .unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct Currency {
    pub bid: String,
}

async fn get_price() -> f32 {
    let url = "https://economia.awesomeapi.com.br/last/USD-BRL";
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<HashMap<String, Currency>>()
        .await
        .unwrap();
    response.get("USDBRL").unwrap().bid.parse::<f32>().unwrap()
}
