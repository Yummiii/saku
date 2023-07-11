use num_enum::IntoPrimitive;
use poise::ChoiceParameter;
use sqlx::Type;

#[derive(IntoPrimitive, Clone, Copy, Type, ChoiceParameter, PartialEq, Debug)]
#[repr(u8)]
pub enum Models {
    Gpt3,
    Gpt4,
    Gpt4_32K
}

impl Models {
    pub fn model(&self) -> String {
        let model = match self {
            Models::Gpt3 => "gpt-3.5-turbo",
            Models::Gpt4 => "gpt-4",
            Models::Gpt4_32K => "gpt-4-32k",
        };
        model.to_string()
    }
}
