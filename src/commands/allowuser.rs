use crate::{
    database::{
        users::{self, UserStates},
        virtualusers::{self, VirtualUser, VirtualUserRoles},
    },
    Context, Error,
};
use poise::{command, AutocompleteChoice};

pub async fn autocomplete_virtual_user<'a>(
    ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice<i64>> {
    let db = &ctx.data().db;
    let uid = ctx.author().id.0 as i64;

    if let Some(user) = users::get_by_discord_id(db, uid).await {
        let vus = virtualusers::get_virtual_users(db, user.id, VirtualUserRoles::Admin)
            .await
            .unwrap();
        let mut choices = Vec::new();

        for vu in vus {
            let user = users::get_by_id(db, vu.virtual_user_id).await.unwrap();

            choices.push(AutocompleteChoice {
                name: user.name,
                value: vu.virtual_user_id,
            });
        }

        choices.into_iter()
    } else {
        vec![].into_iter()
    }
}

///Allow user
#[command(slash_command)]
pub async fn au(
    ctx: Context<'_>,
    #[rename = "virtual"]
    #[autocomplete = "autocomplete_virtual_user"]
    vu: i64,
    user: poise::serenity_prelude::User,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    if let Some(author) = users::get_by_discord_id(db, ctx.author().id.0 as i64).await {
        let admins = virtualusers::get_virtual_by_role(db, vu, VirtualUserRoles::Admin).await?;
        if admins.iter().any(|x| x.user_id == author.id) {
            let user = if let Some(user) = users::get_by_discord_id(db, user.id.0 as i64).await {
                user.id
            } else {
                users::add_user(
                    db,
                    &users::User {
                        id: 0,
                        discord_id: user.id.0 as i64,
                        name: user.name,
                        state: UserStates::Normal,
                        virtal: false,
                    },
                )
                .await?
            };

            virtualusers::add_virtual_user(
                db,
                &VirtualUser {
                    id: 0,
                    user_id: user,
                    virtual_user_id: vu,
                    role: VirtualUserRoles::Normal
                },
            )
            .await?;

            ctx.say(":)").await?;
        } else {
            ctx.say(":(").await?;
        }
    } else {
        ctx.say(":(").await?;
    }

    Ok(())
}
