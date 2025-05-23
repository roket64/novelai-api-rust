use serde::Serialize;

// below constants should be automatically injected

pub const V4_5_CURATED_HEAVY: &'static str = 
    "blurry, lowres, upscaled, artistic error, film grain, scan artifacts, worst quality, bad quality, jpeg artifacts, very displeasing, chromatic aberration, halftone, multiple views, logo, too many watermarks, negative space, blank page";
pub const V4_5_CURATED_LIGHT: &'static str = 
    "blurry, lowres, upscaled, artistic error, scan artifacts, jpeg artifacts, logo, too many watermarks, negative space, blank page";
pub const V4_5_CURATED_HUMAN_FOCUS: &'static str = 
    "blurry, lowres, upscaled, artistic error, film grain, scan artifacts, bad anatomy, bad hands, worst quality, bad quality, jpeg artifacts, very displeasing, chromatic aberration, halftone, multiple views, logo, too many watermarks, @_@, mismatched pupils, glowing eyes, negative space, blank page";

pub const V4_FULL_HEAVY: &'static str = 
    "blurry, lowres, error, film grain, scan artifacts, worst quality, bad quality, jpeg artifacts, very displeasing, chromatic aberration, multiple views, logo, too many watermarks";
pub const V4_FULL_LIGHT: &'static str =
    "blurry, lowres, error, worst quality, bad quality, jpeg artifacts, very displeasing";

pub const V4_CURATED_HEAVY: &'static str = 
    "blurry, lowres, error, film grain, scan artifacts, worst quality, bad quality, jpeg artifacts, very displeasing, chromatic aberration, logo, dated, signature, multiple views, gigantic breasts";
pub const V4_CURATED_LIGHT: &'static str = 
    "blurry, lowres, error, worst quality, bad quality, jpeg artifacts, very displeasing, logo, dated, signature";

pub const V3_HEAVY: &'static str = 
    "nsfw, lowres, {bad}, error, fewer, extra, missing, worst quality, jpeg artifacts, bad quality, watermark, unfinished, displeasing, chromatic aberration, signature, extra digits, artistic error, username, scan, [abstract],";
pub const V2_HEAVY: &'static str = 
    "lowres, bad, text, error, missing, extra, fewer, cropped, jpeg artifacts, worst quality, bad quality, watermark, displeasing, unfinished, chromatic aberration, scan, scan artifacts,";
pub const V3_LIGHT: &'static str =
    "lowres, jpeg artifacts, worst quality, watermark, blurry, very displeasing,";
pub const V2_LIGHT: &'static str =
    "lowres, jpeg artifacts, worst quality, watermark, blurry, very displeasing,";
pub const V3_HUMAN_FOCUS: &'static str = 
    "lowres, {bad}, error, fewer, extra, missing, worst quality, jpeg artifacts, bad quality, watermark, unfinished, displeasing, chromatic aberration, signature, extra digits, artistic error, username, scan, [abstract], bad anatomy, bad hands, @_@, mismatched pupils, heart-shaped pupils, glowing eyes,";

pub const FURRY_V3_HEAVY: &'static str = 
    "{{worst quality}}, [displeasing], {unusual pupils}, guide lines, {{unfinished}}, {bad}, url, artist name, {{tall image}}, mosaic, {sketch page}, comic panel, impact (font), [dated], {logo}, ych, {what}, {where is your god now}, {distorted text}, repeated text, {floating head}, {1994}, {widescreen}, absolutely everyone, sequence, {compression artifacts}, hard translated, {cropped}, {commissioner name}, unknown text, high contrast,";
pub const FURRY_V3_LIGHT: &'static str = 
    "{worst quality}, guide lines, unfinished, bad, url, tall image, widescreen, compression artifacts, unknown text,";

pub const V1_LOW_QUALITY: &'static str = 
    "lowres, text, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry,";
pub const V1_LOW_QUALITY_BAD_ANATOMY: &'static str = 
    "lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry,";

pub const V1_FURRY_BAD_ANATOMY: &'static str = 
    "{worst quality}, low quality, distracting watermark, [nightmare fuel], {{unfinished}}, deformed, outline, pattern, simple background,";
pub const V1_FURRY_LOW_QUALITY: &'static str = 
    "worst quality, low quality, what has science done, what, nightmare fuel, eldritch horror, where is your god now, why,";

#[derive(Debug, Default, Serialize)]
pub enum UCPreset {
    LowQualityBadAnatomy = 0,
    LowQuality = 1,
    BadAnatomy = 2,
    #[default]
    None = 3,
    Heavy = 4,
    Light = 5,
}
