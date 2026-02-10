use serde_json::json;
use uuid::Uuid;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;
use crate::ollama::OllamaClient;
use crate::image_processor::ImageProcessor;
use crate::config::Config;
use crate::AppState;

fn get_current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("2026-02-10T{}Z", now) // Simplified ISO format
}

#[tauri::command]
pub async fn process_image(
    _image_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Create new project
    let project_id = Uuid::new_v4().to_string();
    *state.current_project.lock().unwrap() = Some(project_id.clone());

    Ok(json!({
        "project_id": project_id,
        "image_loaded": true
    }).to_string())
}

#[tauri::command]
pub async fn detect_content(
    image_base64: String,
    detection_type: String, // "nudity" or "text_redaction"
    locale: String,
    config_json: String,
) -> Result<String, String> {
    let config: AppConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let client = OllamaClient::new(
        config.ollama_endpoint.clone(),
        config.ollama_model.clone(),
    );

    // Test connection first
    client.test_connection()
        .await
        .map_err(|e| format!("Cannot connect to Ollama: {}", e))?;

    // Send to Ollama for content detection
    let response = client.detect_content(image_base64, &detection_type, &locale).await?;

    // Convert detected regions to censoring regions with polygon masks
    let censoring_regions: Vec<CensoringRegion> = response
        .regions
        .iter()
        .map(|region| {
            let mask_type = MaskType::Blob; // Use blob masks for natural shapes
            let polygon = if region.label.contains("nipple") || region.label.contains("genital") {
                ImageProcessor::bbox_to_circular_mask(
                    region.bbox.x,
                    region.bbox.y,
                    region.bbox.width,
                    region.bbox.height,
                )
            } else {
                ImageProcessor::bbox_to_blob_mask(
                    region.bbox.x,
                    region.bbox.y,
                    region.bbox.width,
                    region.bbox.height,
                )
            };

            CensoringRegion {
                id: Uuid::new_v4().to_string(),
                detection_type: if detection_type == "nudity" {
                    DetectionType::Nudity
                } else {
                    DetectionType::TextRedaction
                },
                mask_type,
                censoring_method: CensoringMethod::Blur,
                polygon,
                confidence: region.confidence,
                metadata: {
                    let mut m = std::collections::HashMap::new();
                    m.insert("label".to_string(), region.label.clone());
                    m.insert("detection_type".to_string(), region.detection_type.clone());
                    m
                },
            }
        })
        .collect();

    Ok(serde_json::to_string(&censoring_regions)
        .map_err(|e| format!("Serialization failed: {}", e))?)
}

#[tauri::command]
pub async fn apply_censoring(
    image_base64: String,
    regions_json: String,
) -> Result<String, String> {
    let regions: Vec<CensoringRegion> = serde_json::from_str(&regions_json)
        .map_err(|e| format!("Failed to parse regions: {}", e))?;

    let image = ImageProcessor::load_image(&image_base64)?;
    let censored_image = ImageProcessor::apply_censoring(image, &regions)?;
    let result_base64 = ImageProcessor::image_to_base64(&censored_image)?;

    Ok(result_base64)
}

#[tauri::command]
pub fn get_censoring_regions(
    project_json: String,
) -> Result<String, String> {
    let project: CensoringProject = serde_json::from_str(&project_json)
        .map_err(|e| format!("Failed to parse project: {}", e))?;

    Ok(serde_json::to_string(&project.regions)
        .map_err(|e| format!("Serialization failed: {}", e))?)
}

#[tauri::command]
pub fn adjust_region(
    region_json: String,
    new_polygon_json: String,
) -> Result<String, String> {
    let mut region: CensoringRegion = serde_json::from_str(&region_json)
        .map_err(|e| format!("Failed to parse region: {}", e))?;

    let new_polygon: Vec<(f64, f64)> = serde_json::from_str(&new_polygon_json)
        .map_err(|e| format!("Failed to parse polygon: {}", e))?;

    region.polygon = new_polygon;

    Ok(serde_json::to_string(&region)
        .map_err(|e| format!("Serialization failed: {}", e))?)
}

#[tauri::command]
pub fn save_project(
    project_json: String,
    save_path: String,
) -> Result<String, String> {
    let project: CensoringProject = serde_json::from_str(&project_json)
        .map_err(|e| format!("Failed to parse project: {}", e))?;

    let data_path = format!("{}.censor", save_path);
    let content = serde_json::to_string_pretty(&project)
        .map_err(|e| format!("Serialization failed: {}", e))?;

    fs::write(&data_path, content)
        .map_err(|e| format!("Failed to save project: {}", e))?;

    Ok(json!({
        "saved": true,
        "path": data_path,
        "project_id": project.id
    }).to_string())
}

#[tauri::command]
pub fn load_project(
    project_path: String,
) -> Result<String, String> {
    if !Path::new(&project_path).exists() {
        return Err(format!("Project file not found: {}", project_path));
    }

    let content = fs::read_to_string(&project_path)
        .map_err(|e| format!("Failed to read project: {}", e))?;

    let project: CensoringProject = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse project: {}", e))?;

    Ok(serde_json::to_string(&project)
        .map_err(|e| format!("Serialization failed: {}", e))?)
}

#[tauri::command]
pub fn get_config() -> Result<String, String> {
    let config = Config::default();

    Ok(serde_json::to_string(&config.app)
        .map_err(|e| format!("Serialization failed: {}", e))?)
}

#[tauri::command]
pub fn set_config(
    config_json: String,
) -> Result<String, String> {
    let _config: AppConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    // In production, save to config file
    Ok(json!({
        "updated": true
    }).to_string())
}
