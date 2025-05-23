use serde::Serialize;

use crate::{
    schema::{action::Action, model::Model},
    utils::config::Parameters,
};

struct V3ImageConfig {
}

struct V4ImageConfig {
    pub v4_prompt: V4Prompt,
    pub v4_negative_prompt: V4NegativePrompt,
}

struct V4Prompt {}

struct V4NegativePrompt {}
