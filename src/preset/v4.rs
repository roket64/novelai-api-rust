use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "caption")]
pub struct Caption {
    pub base_caption: String,
    pub char_captions: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "v4_prompt")]
pub struct Prompt {
    pub caption: Caption,
    pub use_coords: bool,
    pub use_order: bool,
    pub legacy_uc: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename = "v4_negative_prompt")]
pub struct NegativePrompt {
    pub caption: Caption,
    pub use_coords: bool,
    pub use_order: bool,
    pub legacy_uc: bool,
}

impl Default for Caption {
    fn default() -> Self {
        Self {
            base_caption: Default::default(),
            char_captions: Default::default(),
        }
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            caption: Default::default(),
            use_coords: false,
            use_order: true,
            legacy_uc: false,
        }
    }
}

impl Default for NegativePrompt {
    fn default() -> Self {
        Self {
            caption: Default::default(),
            use_coords: false,
            use_order: true,
            legacy_uc: false,
        }
    }
}
