use std::fmt::Display;

use serde::Serialize;

use crate::utils::config::ImageConfig;

pub trait NAIModel {
    fn name(self) -> &'static str;

    fn is_v3(self) -> bool;

    fn is_v4(self) -> bool;

    fn is_inpainting(self) -> bool;

    fn config(self) -> ImageConfig;
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum Model {
    #[default]
    #[serde(rename = "nai-diffusion-3")]
    AnimeV3,
    #[serde(rename = "nai-diffusion-3-inpainting")]
    AnimeV3Inpainting,

    #[serde(rename = "nai-diffusion-furry-3")]
    FurryV3,
    #[serde(rename = "nai-diffusion-furry-3-inpainting")]
    FurryV3Inpainting,

    #[serde(rename = "nai-diffusion-4-curated-preview")]
    AnimeV4Curated,
    #[serde(rename = "nai-diffusion-4-full")]
    AnimeV4,
    #[serde(rename = "nai-diffusion-4-curated-inpainting")]
    AnimeV4CuratedInpainting,
    #[serde(rename = "nai-diffusion-4-full-inpainting")]
    AnimeV4Inpainting,
}

impl Model {
    pub fn as_str(self) -> &'static str {
        match self {
            Model::AnimeV3 => "nai-diffusion-3",
            Model::AnimeV3Inpainting => "nai-diffusion-3-inpaiting",
            Model::FurryV3 => "nai-diffusion-furry-3",
            Model::FurryV3Inpainting => "nai-diffusion-furry-3-inpainting",
            Model::AnimeV4Curated => "nai-diffusion-4-curated-preview",
            Model::AnimeV4 => "nai-diffusion-4-full",
            Model::AnimeV4CuratedInpainting => "nai-diffusion-4-curated-inpainting",
            Model::AnimeV4Inpainting => "nai-diffusion-4-full-inpainting",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl NAIModel for Model {
    fn name(self) -> &'static str {
        self.as_str()
    }

    fn is_v3(self) -> bool {
        match self {
            Model::AnimeV3Inpainting => true,
            Model::FurryV3 => true,
            Model::FurryV3Inpainting => true,
            _ => false,
        }
    }

    fn is_v4(self) -> bool {
        match self {
            Model::AnimeV4 => true,
            Model::AnimeV4CuratedInpainting => true,
            Model::AnimeV4Inpainting => true,
            _ => false,
        }
    }

    fn is_inpainting(self) -> bool {
        match self {
            Model::AnimeV3Inpainting => true,
            Model::FurryV3Inpainting => true,
            Model::AnimeV4CuratedInpainting => true,
            Model::AnimeV4Inpainting => true,
            _ => false,
        }
    }

    fn config(self) -> ImageConfig {
        todo!()
    }
}
