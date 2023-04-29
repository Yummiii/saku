use num_enum::IntoPrimitive;
use poise::ChoiceParameter;
use sqlx::Type;

#[derive(IntoPrimitive, Clone, Copy, Type, ChoiceParameter, PartialEq, Debug)]
#[repr(u8)]
pub enum Models {
    Gpt3,
    Gpt4,
}

impl Models {
    pub fn model(&self) -> String {
        let model = match self {
            Models::Gpt3 => "gpt-3.5-turbo",
            Models::Gpt4 => "gpt-4",
        };
        model.to_string()
    }
}
