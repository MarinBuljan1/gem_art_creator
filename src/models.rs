use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GemCount {
    pub floss: String,
    pub count: u32,
    pub hex: String,
}

#[derive(Clone, PartialEq)]
pub enum ImageFitOption {
    Fit,
    Crop,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ColorMappingMode {
    Nearest,
    AdaptiveLightnessStretch,
}

#[derive(Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Color {
    pub value: String,
    pub floss_number: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DmcColorPrecomputed {
    pub floss: String,
    pub dmc_name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
    pub lab_l: f32,
    pub lab_a: f32,
    pub lab_b: f32,
    pub blended_r: u8,
    pub blended_g: u8,
    pub blended_b: u8,
}
