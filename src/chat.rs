use crate::database::{add_context, get_context, Message};
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use sqlx::{Pool, Sqlite};

pub async fn create_completion(
    msg: &String,
    db: &Pool<Sqlite>,
    channel: u64,
) -> Result<String, ()> {
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "Your are a nice Discord chat bot called Saku".to_string(),
        name: None,
    }];

    for ctx in get_context(&db, channel as i64).await {
        messages.push(ChatCompletionMessage {
            role: match ctx.role.as_str() {
                "user" => ChatCompletionMessageRole::User,
                "assistant" => ChatCompletionMessageRole::Assistant,
                _ => unreachable!(),
            },
            content: ctx.content,
            name: None,
        });
    }

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: msg.to_owned(),
        name: None,
    });

    add_context(
        &db,
        Message {
            role: "user".to_string(),
            content: msg.to_owned(),
            channel: channel as i64,
        },
    )
    .await;
    let completions = ChatCompletion::builder("gpt-3.5-turbo", messages).create().await;

    match completions {
        Ok(completions) => match completions {
            Ok(completions) => {
                let completion = completions.choices.first().unwrap().message.clone();
                add_context(
                    &db,
                    Message {
                        role: "assistant".to_string(),
                        content: completion.content.clone(),
                        channel: channel as i64,
                    },
                )
                .await;
                Ok(completion.content)
            }
            Err(why) => {
                println!("Error creating completion: {:?}", why);
                Err(())
            }
        },
        Err(why) => {
            println!("Error creating completion: {:?}", why);
            Err(())
        }
    }
}
