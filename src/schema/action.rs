use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum Action {
    #[default]
    #[serde(rename = "generate")]
    Generate,
    #[serde(rename = "img2img")]
    Img2Img,
    #[serde(rename = "infill")]
    Infill,
}

impl Action {
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Generate => "generate",
            Action::Img2Img => "img2img",
            Action::Infill => "infill",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
