use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensoringRegion {
    pub id: String,
    pub detection_type: DetectionType,
    pub mask_type: MaskType,
    pub censoring_method: CensoringMethod,
    pub polygon: Vec<(f64, f64)>,
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectionType {
    Nudity,
    TextRedaction,
    CustomKeyword,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaskType {
    Polygon,
    Blob,
    PointCloud,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CensoringMethod {
    Blackout,
    Blur,
    Pixelate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensoringProject {
    pub id: String,
    pub name: String,
    pub image_path: String,
    pub original_image_data: Option<Vec<u8>>,
    pub regions: Vec<CensoringRegion>,
    pub created_at: String,
    pub modified_at: String,
    pub settings: ProjectSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub detect_nudity: bool,
    pub nudity_locale: String,
    pub detect_text_redaction: bool,
    pub redaction_keywords: Vec<String>,
    pub target_audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub regions: Vec<DetectedRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedRegion {
    pub detection_type: String,
    pub bbox: BoundingBox,
    pub confidence: f64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub ollama_endpoint: String,
    pub ollama_model: String,
    pub default_censoring_method: String,
    pub supported_locales: Vec<LocaleConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocaleConfig {
    pub code: String,
    pub name: String,
    pub nudity_standards: NudityStandards,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NudityStandards {
    pub censor_female_nipples: bool,
    pub censor_genitals: bool,
    pub censor_buttocks: bool,
    pub strictness_level: i32,
}
