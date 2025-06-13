use serde::Serialize;

pub mod model;
pub mod noise;
pub mod resolution;
pub mod sampler;
pub mod uc_preset;
pub mod v4;

use model::Model;
use noise::Noise;
use resolution::Resolution;
use sampler::Sampler;
use uc_preset::UCPreset;
use v4::*;

fn _impl_preset_v3() -> ImagePreset {
    todo!()
}

fn _impl_preset_v3_furry() -> ImagePreset {
    todo!()
}

fn _impl_preset_v4() -> ImagePreset {
    todo!()
}

fn _impl_preset_v4_curated() -> ImagePreset {
    todo!()
}

#[derive(Clone, Debug, Serialize)]
pub struct ImagePreset {
    #[serde(rename = "input")]
    pub prompt: String,
    pub model: Model,
    pub parameters: Parameters,
}

#[derive(Debug)]
pub struct ImagePresetBuilder {
    prompt: String,
    model: Model,
    parameters: Parameters,
}

#[derive(Clone, Debug, Serialize)]
pub struct Parameters {
    // this option will automatically converted to `uc`
    negative_prompt: Option<String>,
    n_samples: Option<u32>,
    sampler: Option<Sampler>,
    steps: Option<u32>,
    scale: Option<f32>,
    cfg_rescale: Option<f32>,
    seed: Option<u32>,
    noise_schedule: Option<Noise>,
    // this option will ignored when actual request is sent
    #[serde(rename = "qualityToggle")]
    // this option will ignored when actual request is sent
    quality_toggle: Option<bool>,
    #[serde(rename = "ucPreset")]
    uc_preset: Option<u32>,
    // this parameters are required to use v4 and v4.5 model
    v4_prompt: Option<Prompt>,
    v4_negative_prompt: Option<NegativePrompt>,
    uncond_scale: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
    controlnet_strength: Option<f32>,
    deliberate_euler_ancestral_bug: Option<bool>,
    prefer_brownian: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ParametersBuilder {
    negative_prompt: Option<String>,
    n_samples: Option<u32>,
    sampler: Option<Sampler>,
    steps: Option<u32>,
    scale: Option<f32>,
    cfg_rescale: Option<f32>,
    seed: Option<u32>,
    noise_schedule: Option<Noise>,
    #[serde(rename = "qualityToggle")]
    quality_toggle: Option<bool>,
    #[serde(rename = "ucPreset")]
    uc_preset: Option<u32>,
    v4_prompt: Option<Prompt>,
    v4_negative_prompt: Option<NegativePrompt>,
    uncond_scale: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
    controlnet_strength: Option<f32>,
    deliberate_euler_ancestral_bug: Option<bool>,
    prefer_brownian: Option<bool>,
}

impl ImagePreset {
    pub fn from(model: Model) -> Self {
        match model {
            Model::AnimeV3 => todo!(),
            Model::AnimeV3Inpainting => unimplemented!("unimplemented model"),
            Model::FurryV3 => todo!(),
            Model::FurryV3Inpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4Curated => todo!(),
            Model::AnimeV4 => todo!(),
            Model::AnimeV4CuratedInpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4Inpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4_5 => todo!(),
            Model::AnimeV4_5Curated => todo!(),
        }
    }

    pub fn builder() -> ImagePresetBuilder {
        ImagePresetBuilder::new()
    }
}

impl ImagePresetBuilder {
    pub fn new() -> Self {
        ImagePresetBuilder {
            prompt: Default::default(),
            model: Model::default(),
            parameters: Parameters::new(),
        }
    }

    pub fn from(model: Model) -> Self {
        match model {
            Model::AnimeV3 => todo!(),
            Model::AnimeV3Inpainting => unimplemented!("unimplemented model"),
            Model::FurryV3 => todo!(),
            Model::FurryV3Inpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4Curated => todo!(),
            Model::AnimeV4 => todo!(),
            Model::AnimeV4CuratedInpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4Inpainting => unimplemented!("unimplemented model"),
            Model::AnimeV4_5 => todo!(),
            Model::AnimeV4_5Curated => todo!(),
        }
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn parameters(mut self, parameters: Parameters) -> Self {
        // TODO: check if `quality_toggle` or `uc_preset` is set.
        // if so, quality tags should be injected into prompt or negative prompts.
        self.parameters = parameters;
        self
    }

    pub fn build(mut self) -> ImagePreset {
        if self.parameters.quality_toggle.unwrap() {
            let mut new_prompt = self.parameters.v4_prompt.clone().unwrap();

            match self.model {
                Model::AnimeV4Curated => {
                    new_prompt.caption.base_caption = format!(
                        "{}, very aesthetic, masterpiece, no text, -0.8::feet::, rating:general",
                        self.prompt
                    );
                    self.parameters.v4_prompt = Some(new_prompt);
                }
                Model::AnimeV4 => {
                    new_prompt.caption.base_caption = format!(
                        "{}, no text, best quality, very aesthetic, absurdres",
                        self.prompt
                    );
                    self.parameters.v4_prompt = Some(new_prompt);
                }
                Model::AnimeV4_5Curated => {
                    new_prompt.caption.base_caption = format!(
                        "{}, very aesthetic, masterpiece, no text, -0.8::feet::, rating:general",
                        self.prompt
                    );
                    self.parameters.v4_prompt = Some(new_prompt);
                }
                Model::AnimeV4_5 => {
                    new_prompt.caption.base_caption =
                        format!("{}, very aesthetic, masterpiece, no text", self.prompt);
                    self.parameters.v4_prompt = Some(new_prompt);
                }
                _ => unimplemented!(),
            }
        }

        ImagePreset {
            prompt: self.prompt,
            model: self.model,
            parameters: self.parameters,
        }
    }
}

impl Parameters {
    pub fn new() -> Self {
        ParametersBuilder::new().build()
    }

    pub fn builder() -> ParametersBuilder {
        ParametersBuilder::new()
    }
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self {
            negative_prompt: Some(Default::default()),
            n_samples: Some(1u32),
            sampler: Some(Sampler::default()),
            steps: Some(23u32),
            scale: Some(5.5f32),
            cfg_rescale: Some(0.0f32),
            seed: Option::<u32>::None,
            noise_schedule: Some(Noise::default()),
            quality_toggle: Some(true),
            uc_preset: Some(UCPreset::default() as u32),
            v4_prompt: Some(Default::default()),
            v4_negative_prompt: Some(Default::default()),
            uncond_scale: Some(1.0f32),
            width: Some(Resolution::default().as_resolution().0),
            height: Some(Resolution::default().as_resolution().1),
            // I don't know what exactly the below options do,
            // but Web API uses them, so added them...
            controlnet_strength: Some(1.0f32),
            deliberate_euler_ancestral_bug: Some(false),
            prefer_brownian: Some(true),
        }
    }

    pub fn from(other: Parameters) -> Self {
        ParametersBuilder {
            negative_prompt: other.negative_prompt,
            n_samples: other.n_samples,
            sampler: other.sampler,
            steps: other.steps,
            scale: other.scale,
            cfg_rescale: other.cfg_rescale,
            seed: other.seed,
            noise_schedule: other.noise_schedule,
            quality_toggle: other.quality_toggle,
            uc_preset: other.uc_preset,
            v4_prompt: other.v4_prompt,
            v4_negative_prompt: other.v4_negative_prompt,
            uncond_scale: other.uncond_scale,
            width: other.width,
            height: other.height,
            controlnet_strength: other.controlnet_strength,
            deliberate_euler_ancestral_bug: other.deliberate_euler_ancestral_bug,
            prefer_brownian: other.prefer_brownian,
        }
    }

    pub fn negative_prompt(mut self, negative_prompt: String) -> Self {
        self.negative_prompt = Some(negative_prompt);
        self
    }

    pub fn resolution(mut self, resolution: Resolution) -> Self {
        let resolution = resolution.as_resolution();
        self.width = Some(resolution.0);
        self.height = Some(resolution.1);
        self
    }

    pub fn n_samples(mut self, n_samples: u32) -> Self {
        self.n_samples = Some(n_samples);
        self
    }

    pub fn sampler(mut self, sampler: Sampler) -> Self {
        self.sampler = Some(sampler);
        self
    }

    pub fn steps(mut self, steps: u32) -> Self {
        self.steps = Some(steps);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn cfg_rescale(mut self, cfg_scale: f32) -> Self {
        self.cfg_rescale = Some(cfg_scale);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn noise_schedule(mut self, noise_schedule: Noise) -> Self {
        self.noise_schedule = Some(noise_schedule);
        self
    }

    pub fn quality_toggle(mut self, quality_toggle: bool) -> Self {
        self.quality_toggle = Some(quality_toggle);
        self
    }

    pub fn uc_preset(mut self, uc_preset: UCPreset) -> Self {
        self.uc_preset = Some(uc_preset as u32);
        self
    }

    pub fn v4_prompt(mut self, v4_prompt: Option<Prompt>) -> Self {
        self.v4_prompt = v4_prompt;
        self
    }

    pub fn v4_negative_prompt(mut self, v4_negative_prompt: Option<NegativePrompt>) -> Self {
        self.v4_negative_prompt = v4_negative_prompt;
        self
    }

    pub fn uncond_scale(mut self, uncond_scale: f32) -> Self {
        self.uncond_scale = Some(uncond_scale);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn build(self) -> Parameters {
        Parameters {
            negative_prompt: self.negative_prompt,
            n_samples: self.n_samples,
            sampler: self.sampler,
            steps: self.steps,
            scale: self.scale,
            cfg_rescale: self.cfg_rescale,
            seed: self.seed,
            noise_schedule: self.noise_schedule,
            quality_toggle: self.quality_toggle,
            uc_preset: self.uc_preset,
            v4_prompt: self.v4_prompt,
            v4_negative_prompt: self.v4_negative_prompt,
            uncond_scale: self.uncond_scale,
            width: self.width,
            height: self.height,
            controlnet_strength: self.controlnet_strength,
            deliberate_euler_ancestral_bug: self.deliberate_euler_ancestral_bug,
            prefer_brownian: self.prefer_brownian,
        }
    }
}
