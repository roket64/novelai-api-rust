use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum Resolution {
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

impl Resolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Resolution::SmallPortrait => "small-portrait",
            Resolution::SmallLandscape => "small-landscape",
            Resolution::SmallSquare => "small-square",
            Resolution::NormalPortrait => "normal-portrait",
            Resolution::NormalLandscape => "normal-landscape",
            Resolution::NormalSquare => "normal-square",
            Resolution::LargePortrait => "large-portrait",
            Resolution::LargeLandscape => "large-landscape",
            Resolution::LandSquare => "landsquare",
            Resolution::WallpaperPortrait => "wallpaper-portrait",
            Resolution::WallpaperLandscape => "wallpaper-landscape",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }

    pub fn as_resolution(self) -> (u32, u32) {
        match self {
            Resolution::SmallPortrait => (512, 768),
            Resolution::SmallLandscape => (768, 512),
            Resolution::SmallSquare => (640, 640),
            Resolution::NormalPortrait => (832, 1216),
            Resolution::NormalLandscape => (1216, 832),
            Resolution::NormalSquare => (1024, 1024),
            Resolution::LargePortrait => (1024, 1536),
            Resolution::LargeLandscape => (1536, 1024),
            Resolution::LandSquare => (1472, 1472),
            Resolution::WallpaperPortrait => (1088, 1920),
            Resolution::WallpaperLandscape => (1920, 1088),
        }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
