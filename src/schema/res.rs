use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum ResPreset {
    #[serde(rename = "small-portrait")]
    SmallPortrait = 0,
    #[serde(rename = "small-landscape")]
    SmallLandscape = 1,
    #[serde(rename = "small-square")]
    SmallSquare = 2,

    #[default]
    #[serde(rename = "normal-portrait")]
    NormalPortrait = 3,
    #[serde(rename = "normal-landscape")]
    NormalLandscape = 4,
    #[serde(rename = "normal-sqaure")]
    NormalSquare = 5,

    #[serde(rename = "large-portrait")]
    LargePortrait = 6,
    #[serde(rename = "large-landscape")]
    LargeLandscape = 7,

    #[serde(rename = "landsquare")]
    LandSquare = 8,

    #[serde(rename = "wallpaper-portrait")]
    WallpaperPortrait = 9,
    #[serde(rename = "wallpaper-landscape")]
    WallpaperLandscape = 10,
}

impl ResPreset {
    pub fn as_str(self) -> &'static str {
        match self {
            ResPreset::SmallPortrait => "small-portrait",
            ResPreset::SmallLandscape => "small-landscape",
            ResPreset::SmallSquare => "small-square",
            ResPreset::NormalPortrait => "normal-portrait",
            ResPreset::NormalLandscape => "normal-landscape",
            ResPreset::NormalSquare => "normal-square",
            ResPreset::LargePortrait => "large-portrait",
            ResPreset::LargeLandscape => "large-landscape",
            ResPreset::LandSquare => "landsquare",
            ResPreset::WallpaperPortrait => "wallpaper-portrait",
            ResPreset::WallpaperLandscape => "wallpaper-landscape",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }

    pub fn as_resolution(self) -> (u32, u32) {
        match self {
            ResPreset::SmallPortrait => (512, 768),
            ResPreset::SmallLandscape => (768, 512),
            ResPreset::SmallSquare => (640, 640),
            ResPreset::NormalPortrait => (832, 1216),
            ResPreset::NormalLandscape => (1216, 832),
            ResPreset::NormalSquare => (1024, 1024),
            ResPreset::LargePortrait => (1024, 1536),
            ResPreset::LargeLandscape => (1536, 1024),
            ResPreset::LandSquare => (1472, 1472),
            ResPreset::WallpaperPortrait => (1088, 1920),
            ResPreset::WallpaperLandscape => (1920, 1088),
        }
    }
}

impl Display for ResPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
