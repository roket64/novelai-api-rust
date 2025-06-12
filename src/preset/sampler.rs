use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum Sampler {
    #[serde(rename = "k_euler")]
    Euler,
    #[default]
    #[serde(rename = "k_euler_ancestral")]
    EulerAncestral,

    #[serde(rename = "k_dpmpp_2m")]
    Dpmpp2m,
    #[serde(rename = "k_dpmpp_2m_sde")]
    Dpmpp2mSDE,
    #[serde(rename = "k_dpmpp_2s_ancestral")]
    Dpmpp2sAncestral,
    #[serde(rename = "k_dpmpp_sde")]
    DpmppSDE,
    #[serde(rename = "k_dpm_2")]
    Dpm2,
    #[serde(rename = "k_dpm_2_ancestral")]
    Dpm2Ancestral,

    #[serde(rename = "ddim")]
    Ddim,
}

impl Sampler {
    pub fn as_str(self) -> &'static str {
        match self {
            Sampler::Euler => "k_euler",
            Sampler::EulerAncestral => "k_euler_ancestral",
            Sampler::Dpmpp2m => "k_dpmpp_2m",
            Sampler::Dpmpp2mSDE => "k_dpmpp_2m_sde",
            Sampler::Dpmpp2sAncestral => "k_dpmpp_2s_ancestral",
            Sampler::DpmppSDE => "k_dpmpp_sde",
            Sampler::Dpm2 => "k_dpm2",
            Sampler::Dpm2Ancestral => "k_dpm2_ancestral",
            Sampler::Ddim => "ddim",
        }
    }

    pub fn to_string(self) -> String {
        String::from(self.as_str())
    }
}

impl Display for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
