use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Bearer Token for the API Authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NAIAccessToken {
    #[serde(rename = "accessToken")]
    pub value: String,
}

impl Display for NAIAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NAIAccessToken {
    pub fn new(token: String) -> Self {
        NAIAccessToken { value: token }
    }
}