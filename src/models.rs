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

#[derive(Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Color {
    pub value: String,
    pub floss_number: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}
