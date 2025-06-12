use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub enum Noise {
    #[serde(rename = "native")]
    Native,
    #[default]
    #[serde(rename = "karras")]
    Karras,
    #[serde(rename = "exponential")]
    Exponential,
    #[serde(rename = "polyexponential")]
    Polyexponential,
}

impl Noise {
    pub fn as_str(self) -> &'static str {
        match self {
            Noise::Native => "native",
            Noise::Karras => "karras",
            Noise::Exponential => "exponential",
            Noise::Polyexponential => "polyexponential",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }
}

impl Display for Noise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
