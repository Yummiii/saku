use crate::database::{
    channels::Channel,
    contexts,
    usage::{self, Usage},
    users::{self, User},
    Database,
};
use chrono::Utc;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

pub async fn create_completion(
    msg: String,
    user: User,
    channel: Channel,
    db: &Database,
) -> Result<String, String> {
    let mut messages = vec![ChatCompletionMessage {
        content: channel
            .system
            .unwrap_or("Your are a nice Discord chat bot called Saku".to_string()),
        name: None,
        role: ChatCompletionMessageRole::System,
    }];

    for ctx in contexts::get_channel_context(db, channel.id).await {
        let (name, role) = if let Some(user_id) = ctx.user {
            if let Some(user) = self::users::get_by_id(db, user_id).await {
                (Some(user.name), ChatCompletionMessageRole::User)
            } else {
                (None, ChatCompletionMessageRole::User)
            }
        } else {
            (None, ChatCompletionMessageRole::Assistant)
        };

        messages.push(ChatCompletionMessage {
            content: ctx.message,
            name,
            role,
        });
    }

    contexts::add_context(
        db,
        &contexts::Context {
            id: 0,
            role: "user".to_string(),
            message: msg.clone(),
            active: true,
            created_at: Utc::now().timestamp(),
            cid: channel.ccid.clone(),
            channel: channel.id,
            user: Some(user.id),
        },
    )
    .await
    .unwrap();

    messages.push(ChatCompletionMessage {
        content: msg,
        name: Some(user.name),
        role: ChatCompletionMessageRole::User,
    });

    let completion = ChatCompletion::builder("gpt-4", messages)
        .create()
        .await
        .unwrap();
    if let Ok(completion) = completion {
        let msg = &completion.choices[0].message.content;
        let usage = completion.usage.unwrap();

        usage::add_usage(
            db,
            &Usage {
                id: 0,
                created_at: Utc::now().timestamp(),
                completion_tokens: usage.completion_tokens as i32,
                prompt_tokens: usage.prompt_tokens as i32,
                cid: channel.ccid.clone(),
                user: user.id,
            },
        )
        .await
        .unwrap();

        contexts::add_context(
            db,
            &contexts::Context {
                id: 0,
                role: "assistant".to_string(),
                message: msg.clone(),
                active: true,
                created_at: Utc::now().timestamp(),
                cid: channel.ccid,
                channel: channel.id,
                user: None,
            },
        )
        .await
        .unwrap();

        Ok(msg.clone())
    } else {
        let err = completion.err().unwrap();
        Err(err.message)
    }
}
