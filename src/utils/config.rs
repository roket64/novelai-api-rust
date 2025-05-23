use serde::Serialize;

use crate::schema::{
    action::Action,
    model::{Model, NAIModel},
    noise::Noise,
    res::ResPreset,
    sampler::Sampler,
    uc_preset::UCPreset,
};

/// Collection of accessible user configurations on web.
#[derive(Debug, Default, Serialize)]
pub struct Parameters {
    /// Negative strings to use for the image.
    pub negative_prompt: Option<String>,
    /// Name of resolution preset to use for the image.
    // i think this doesn't need to be supplied to request directly
    // pub res_preset: Option<ResPreset>,
    /// Number of images to return.
    pub n_samples: Option<u32>,
    pub sampler: Option<Sampler>,
    /// Number of iterations the AI refine the image.
    pub steps: Option<u32>,
    /// Value of prompt guidance.
    pub scale: Option<f32>,
    /// Value of prompt guidance rescale.
    pub cfg_rescale: Option<f32>,
    /// Value of the seed to use for the image.
    pub seed: Option<u32>,
    /// Name of nosise schedule to use for the image.
    pub noise_schedule: Option<Noise>,
    // I don't know why this is in camel case.
    /// Value to toggle `Add Quality Tags`.
    #[serde(rename = "qualityToggle")]
    pub quality_toggle: Option<bool>,
    // I don't know why this is also in camel case .
    /// Value of the `Undesired Content Preset`. \
    #[serde(rename = "ucPreset")]
    pub uc_preset: Option<u32>,
    pub uncond_scale: Option<f32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    // unknown parameters included in web-api
    pub controlnet_strength: Option<f32>,
    pub deliberate_euler_ancestral_bug: Option<bool>,
    pub prefer_brownian: Option<bool>,
}

#[derive(Debug, Default)]
pub struct ParametersBuilder {
    negative_prompt: Option<String>,
    // res_preset: Option<ResPreset>,
    n_samples: Option<u32>,
    sampler: Option<Sampler>,
    steps: Option<u32>,
    scale: Option<f32>,
    cfg_rescale: Option<f32>,
    seed: Option<u32>,
    noise_schedule: Option<Noise>,
    quality_toggle: Option<bool>,
    uc_preset: Option<u32>,
    uncond_scale: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
    controlnet_strength: Option<f32>,
    deliberate_euler_ancestral_bug: Option<bool>,
    prefer_brownian: Option<bool>,
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self {
            negative_prompt: Default::default(),
            // res_preset: Some(ResPreset::default()),
            n_samples: Some(1u32),
            sampler: Some(Sampler::default()),
            steps: Some(23u32),
            scale: Some(5.5f32),
            cfg_rescale: Some(0.0f32),
            seed: Option::<u32>::None,
            noise_schedule: Some(Noise::default()),
            quality_toggle: Some(true),
            uc_preset: Some(UCPreset::default() as u32),
            uncond_scale: Some(1.0f32),
            width: Some(ResPreset::default().as_resolution().0),
            height: Some(ResPreset::default().as_resolution().1),
            controlnet_strength: Some(1.0f32),
            deliberate_euler_ancestral_bug: Some(false),
            prefer_brownian: Some(true),
        }
    }

    pub fn from(model: Model) -> Self {
        if !model.is_v4() {
            // v1-3 models
            match model {
                Model::AnimeV3 => todo!(),
                Model::AnimeV3Inpainting => todo!(),
                Model::FurryV3 => todo!(),
                Model::FurryV3Inpainting => todo!(),
                _ => unreachable!(),
            }
        } else {
            // v4 models

            match model {
                Model::AnimeV4Curated => todo!(),
                Model::AnimeV4 => todo!(),
                Model::AnimeV4CuratedInpainting => todo!(),
                Model::AnimeV4Inpainting => todo!(),
                _ => unreachable!(),
            }
        }
        todo!()
    }

    pub fn negative_prompt(mut self, negative_prompt: String) -> Self {
        self.negative_prompt = Some(negative_prompt);
        self
    }

    pub fn res_preset(mut self, res_preset: ResPreset) -> Self {
        let resolution = res_preset.as_resolution();
        // self.res_preset = Some(res_preset);
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

    pub fn build(self) -> Parameters {
        if self.quality_toggle.unwrap() {}

        if self.uc_preset.is_some() {}

        Parameters {
            negative_prompt: self.negative_prompt,
            // res_preset: self.res_preset,
            n_samples: self.n_samples,
            sampler: self.sampler,
            steps: self.steps,
            scale: self.scale,
            cfg_rescale: self.cfg_rescale,
            seed: self.seed,
            noise_schedule: self.noise_schedule,
            quality_toggle: self.quality_toggle,
            uc_preset: self.uc_preset,
            uncond_scale: self.uncond_scale,
            width: self.width,
            height: self.height,
            controlnet_strength: self.controlnet_strength,
            deliberate_euler_ancestral_bug: self.deliberate_euler_ancestral_bug,
            prefer_brownian: self.prefer_brownian,
        }
    }
}

/// Collection of required configurations to use image API.
#[derive(Debug, Serialize)]
pub struct ImageConfig {
    /// String to use for the image.
    #[serde(rename = "input")]
    pub prompt: String,
    /// Name of the AI model to generate image.
    pub model: Model,
    /// Name of action to the AI should perform.
    pub action: Action,
    pub parameters: Parameters,
}

impl ImageConfig {
    pub fn new(input: String, parameters: Parameters) -> Self {
        let model = Model::default();
        let action = Action::default();

        Self {
            prompt: input,
            model,
            action,
            parameters,
        }
    }

    pub fn builder(input: &str) -> ImageConfigBuilder {
        ImageConfigBuilder::new(input)
    }
}

#[derive(Debug)]
pub struct ImageConfigBuilder {
    pub input: String,
    pub model: Model,
    pub action: Action,
    pub parameters: Parameters,
}

impl ImageConfigBuilder {
    pub fn new(prompt: &str) -> Self {
        Self {
            input: prompt.to_string(),
            model: Model::default(),
            action: Action::default(),
            parameters: Default::default(),
        }
    }

    // TODO: implement this
    pub fn from_model(model: Model) -> Self {
        todo!()
    }

    pub fn input(mut self, input: &str) -> Self {
        self.input = input.to_string();
        self
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn params(mut self, params: Parameters) -> Self {
        self.parameters = params;
        self
    }

    pub fn build(self) -> ImageConfig {
        // TOOD: implment quality tags
        // if self.parameters.quality_toggle.unwrap() {
        //     self.input = String::from("asdf");
        // }

        ImageConfig {
            prompt: self.input,
            model: self.model,
            action: self.action,
            parameters: self.parameters,
        }
    }
}

// TODO: make a builder for this
/// Full collection of configurations for the image API, including the hidden ones on web.
#[derive(Debug, Serialize)]
pub struct _FullMetadata {
    /// Text prompt to generate image from. required.
    prompt: String,
    /// Model to use for the generaton. optional.
    model: Option<String>,
    /// Action to perform. optional.
    action: Option<String>,
    /// Resolution preset to use for the image. optional.
    res_preset: Option<String>,

    negative_prompt: Option<String>,
    #[serde(rename = "qualityToggle")]
    /// Whether to automatically append quality tags to the prompt, optional.
    quality_toggle: Option<String>,
    /// Preset value of undesired content, optional.
    /// Range: 0-3, 0: Heavy, 1: Light, 2: Human Focus, 3: None
    #[serde(rename = "ucPreset")]
    uc_preset: Option<String>,

    /// Width of the image to generate, optional.
    /// This overrides `res_preset` parameter.
    width: Option<u32>,
    /// Height of the image to generate, optional.
    /// This overrides `res_preset` parameter.
    height: Option<u32>,
    /// Number of images to return, optional.
    n_samples: Option<u32>,

    steps: Option<u32>,
    /// Value of prompt guidance, optional.
    scale: Option<u32>,
    /// Whether to enable descrisper, optional.
    dynamic_thresholding: Option<bool>,
    /// Random seed to use for the image, optional.
    seed: Option<u32>,
    /// Unknown
    extra_noise_seed: Option<u32>,
    sampler: Option<u32>,
    /// Whether to enable SMEA, optional.
    sm: Option<bool>,
    /// Whether to enable SMEA DYN, optional.
    sm_dyn: Option<bool>,
    /// Undesired content strength, optional.
    /// value must be in between [0, 1.5]
    uncond_scale: Option<f32>,
    /// Prompt guidance rescale, optional.
    /// value must be in between [0-1]
    cfg_rescale: Option<f32>,
    /// Noise Schedule, optional.
    noise_schedule: Option<String>,

    /// Base64-encoded image for Image to Image, optional
    image: Option<String>,
    /// The value should be in between [0.01, 0.99], optional
    strength: Option<f32>,
    /// The value should be in between [0.0, 0.99], optional.
    noise: Option<f32>,
    /// The value should be in between [0.1, 2], optional.
    controlnet_strength: Option<f32>,
    /// Base64-encoded PNG ControlNet mask, optional.
    controlnet_condition: Option<f32>,
    /// Control tool use for the ControlNet, optional.
    controlnet_model: Option<String>,

    add_original_image: Option<bool>,
    /// Base64-encoded black and white image to use as a mask for inpainting, optional.
    mask: Option<String>,

    /// List of base64-encoded images to use as base images for Vibe Transfer, optional.
    reference_image_multiple: Option<Vec<String>>,
    referece_information_extracted_multiple: Option<Vec<f32>>,
    /// The strength AI uses to emulate visual cues, optional.
    referece_strength_multiple: Option<Vec<f32>>,

    params_version: Option<u32>,
    legacy: Option<bool>,
    legacy_v3_extend: Option<bool>,
}
