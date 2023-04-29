use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configs {
    pub token: String,
    pub owner_id: u64,
    pub database_url: String,
    pub openai_key: String
}

impl Configs {
    pub fn new() -> Self {
        Figment::new()
            .merge(Env::prefixed("SAKU_"))
            .extract()
            .unwrap()
    }
}
