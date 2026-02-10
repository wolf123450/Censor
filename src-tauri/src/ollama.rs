use reqwest::Client;
use serde_json::json;
use crate::models::{OllamaResponse, DetectedRegion, BoundingBox};

pub struct OllamaClient {
    endpoint: String,
    model: String,
    client: Client,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: String) -> Self {
        OllamaClient {
            endpoint,
            model,
            client: Client::new(),
        }
    }

    pub async fn detect_content(
        &self,
        image_base64: String,
        detection_type: &str,
        locale: &str,
    ) -> Result<OllamaResponse, String> {
        let prompt = match detection_type {
            "nudity" => format!(
                "Analyze this image for nudity according to {} standards. \
                Provide bounding boxes for: female nipples, genitals, buttocks. \
                Return JSON with regions array containing: detection_type, bbox (x, y, width, height), confidence (0-1), label",
                locale
            ),
            "text_redaction" => format!(
                "Analyze this image for sensitive text or PII including: names, addresses, phone numbers, email, social security numbers, credit card numbers, swear words, slurs. \
                Return JSON with regions array containing: detection_type, bbox (x, y, width, height), confidence (0-1), label"
            ),
            _ => "Analyze image and return regions to censor".to_string(),
        };

        let request_body = json!({
            "model": self.model,
            "prompt": prompt,
            "images": [image_base64],
            "stream": false,
            "format": "json"
        });

        let response = self.client
            .post(&format!("{}/api/generate", self.endpoint))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Ollama returned status: {}", response.status()));
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        // Parse response - handle streaming format
        let response_data: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Extract regions from response
        if let Some(response_obj) = response_data.get("response") {
            if let Some(response_str) = response_obj.as_str() {
                // Try to extract JSON from response
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response_str) {
                    if let Some(regions_arr) = parsed.get("regions").and_then(|v| v.as_array()) {
                        let regions: Vec<DetectedRegion> = regions_arr
                            .iter()
                            .filter_map(|region| {
                                let bbox = region.get("bbox")?;
                                Some(DetectedRegion {
                                    detection_type: region
                                        .get("detection_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown")
                                        .to_string(),
                                    bbox: BoundingBox {
                                        x: bbox.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                        y: bbox.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                        width: bbox.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                        height: bbox.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0),
                                    },
                                    confidence: region
                                        .get("confidence")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5),
                                    label: region
                                        .get("label")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown")
                                        .to_string(),
                                })
                            })
                            .collect();

                        return Ok(OllamaResponse { regions });
                    }
                }
            }
        }

        // Return empty regions if parsing fails
        Ok(OllamaResponse { regions: vec![] })
    }

    pub async fn test_connection(&self) -> Result<bool, String> {
        let response = self
            .client
            .get(&format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .map_err(|e| format!("Connection failed: {}", e))?;

        Ok(response.status().is_success())
    }
}
