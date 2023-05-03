use crate::{
    database::{
        users::{self, User, UserStates},
        virtualusers::{self, VirtualUser, VirtualUserRoles},
    },
    Context, Error,
};
use poise::command;

///Create user
#[command(slash_command)]
pub async fn cu(ctx: Context<'_>, name: String) -> Result<(), Error> {
    let db = &ctx.data().db;

    let vuid = users::add_user(
        db,
        &User {
            name: name.clone(),
            state: UserStates::Normal,
            virtal: true,
            ..Default::default()
        },
    )
    .await?;

    let uid = if let Some(user) = users::get_by_discord_id(db, ctx.author().id.0 as i64).await {
        user.id
    } else {
        users::add_user(
            db,
            &User {
                discord_id: ctx.author().id.0 as i64,
                name: ctx.author().name.clone(),
                state: UserStates::Normal,
                virtal: false,
                ..Default::default()
            },
        )
        .await?
    };

    virtualusers::add_virtual_user(
        db,
        &VirtualUser {
            id: 0,
            user_id: uid,
            virtual_user_id: vuid,
            role: VirtualUserRoles::Admin
        },
    )
    .await?;

    ctx.say(format!("Virtual user \"{}\" created :)", name))
        .await?;
    Ok(())
}
