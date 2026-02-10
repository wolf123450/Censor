# Censor Application Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    DESKTOP APPLICATION                       │
│                     (Tauri + Vue.js)                         │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
        ┌───────▼─────────┐        ┌────────▼──────────┐
        │  FRONTEND       │        │  BACKEND         │
        │  (Vue.js)       │        │  (Rust)          │
        │                 │        │                   │
        │ • Canvas Editor │        │ • Tauri Commands  │
        │ • Region List   │◄──────►│ • Image Processor │
        │ • Settings      │  IPC   │ • Ollama Client   │
        │ • State Mgmt    │        │ • Config Mgmt     │
        └─────────────────┘        └────────┬──────────┘
                                            │
                            ┌───────────────┴───────────────┐
                            │                               │
                    ┌───────▼────────┐          ┌──────────▼────┐
                    │  LOCAL FILES   │          │  OLLAMA       │
                    │                │          │  SERVER       │
                    │ • Projects     │          │               │
                    │ • Cache        │          │ • Qwen3-VL    │
                    │ • Settings     │          │ • Detection   │
                    └────────────────┘          └───────────────┘
```

## Data Flow

### Image Processing Pipeline

```
1. USER UPLOADS IMAGE
   └─> File → Base64 encoding → Loaded into component

2. CONTENT DETECTION
   └─> Vue sends to Rust backend (IPC)
       └─> OllamaClient sends to Ollama Server
           └─> Returns: Regions with bounding boxes
           └─> Convert BBoxes → Polygons (Circular/Blob)

3. DISPLAY DETECTED REGIONS
   └─> Canvas renders image
       └─> SVG overlay shows polygon masks
       └─> Side panel lists regions with confidence

4. HUMAN ADJUSTMENT
   └─> User drags control points or draws new masks
       └─> Real-time polygon updates
       └─> No image processing yet

5. APPLY CENSORING
   └─> User clicks "Apply Effects"
       └─> Backend processes each region with selected method
           ├─> Blackout: Fill polygon with black
           ├─> Blur: Gaussian blur on masked area
           └─> Pixelate: Average colors in blocks
       └─> Returns: Censored image (Base64)

6. SAVE/EXPORT
   └─> Save as project (.censor JSON)
       └─> All mask data preserved for future edits
   └─> Export as PNG/JPG
       └─> Final censored image
```

## Component Communication

```
App.vue (Main)
├── CanvasEditor.vue
│   ├── Displays image
│   ├── Shows polygon overlays (SVG)
│   ├── Handles mask drawing
│   └── Emits: add-region, update-region
├── RegionList.vue
│   ├── Lists all regions
│   ├── Shows detection type & confidence
│   ├── Method selector (Blur/Pixelate/Blackout)
│   └── Emits: select-region, delete-region, update-method
└── SettingsModal.vue
    ├── Ollama config
    ├── Detection settings
    ├── Default censoring method
    └── Emits: close

State Management (Pinia Stores)
├── imageStore
│   ├── currentImage (Base64)
│   ├── loading state
│   └── Error tracking
├── projectStore
│   ├── currentProject
│   ├── regions array
│   ├── save/load logic
│   └── Project metadata
└── configStore
    ├── Ollama settings
    ├── Detection preferences
    └── UI preferences
```

## Backend Architecture

### Command Handlers (IPC)

```rust
pub async fn detect_content(
    image_base64: String,
    detection_type: String,
    locale: String,
    config_json: String
) -> Result<String, String> {
    // 1. Parse config
    // 2. Create OllamaClient
    // 3. Send to Ollama
    // 4. Convert BBboxes to polygons
    // 5. Return CensoringRegion array
}

pub async fn apply_censoring(
    image_base64: String,
    regions_json: String
) -> Result<String, String> {
    // 1. Load image from base64
    // 2. For each region:
    //    - Select censoring method
    //    - Apply effect to polygon area
    // 3. Return censored image as base64
}

pub fn save_project(project_json: String, save_path: String) -> Result<String, String> {
    // 1. Parse project
    // 2. Serialize to JSON
    // 3. Write .censor file
    // 4. Return success
}

pub fn load_project(project_path: String) -> Result<String, String> {
    // 1. Read and parse .censor file
    // 2. Validate data
    // 3. Return project
}
```

### Image Processing Pipeline

```rust
ImageProcessor
├── load_image(path_or_base64) → RgbaImage
├── apply_censoring(image, regions) → RgbaImage
│   └── For each region:
│       ├── apply_blackout(polygon)
│       ├── apply_blur(polygon)
│       └── apply_pixelate(polygon)
├── bbox_to_circular_mask() → Polygon
├── bbox_to_blob_mask() → Polygon
├── fill_polygon_mask() → Scanline fill
└── image_to_base64(image) → String
```

### Ollama Integration

```
OllamaClient
├── new(endpoint, model)
├── test_connection() → bool
└── detect_content(image, type, locale)
    └── POST /api/generate
        ├── Image: Base64-encoded JPEG/PNG
        ├── Prompt: Instruction for vision model
        ├── Model: qwen3-vl:30b
        └── Returns: JSON with detected regions
```

## File Formats

### Project File (.censor)

```json
{
  "id": "unique-project-id",
  "name": "filename.jpg",
  "image_path": "/path/to/original",
  "original_image_data": "base64-encoded-image",
  "regions": [
    {
      "id": "region-id",
      "detection_type": "nudity|text_redaction|custom",
      "mask_type": "polygon|blob",
      "censoring_method": "blur|pixelate|blackout",
      "polygon": [[x1, y1], [x2, y2], ...],
      "confidence": 0.95,
      "metadata": {
        "label": "detected-label",
        "detection_type": "raw-detection-type"
      }
    }
  ],
  "created_at": "ISO-8601-timestamp",
  "modified_at": "ISO-8601-timestamp",
  "settings": {
    "detect_nudity": true,
    "nudity_locale": "us",
    "detect_text_redaction": false,
    "target_audience": "general"
  }
}
```

## Thread Safety & Concurrency

- **Tauri**: Handles IPC thread management automatically
- **Tokio**: Async runtime for image processing
- **AppState**: Protected by Mutex for concurrent access
- **Image processing**: Stateless functions, parallelizable

## Performance Characteristics

| Operation | Typical Time | Factors |
|-----------|--------------|---------|
| Image Upload | <10ms | File size |
| Ollama Detection | 5-30s | Image complexity, GPU availability |
| Apply Censoring | 100-500ms | Image size, region count |
| Save Project | 50-200ms | File I/O speed |
| Load Project | 50-200ms | File I/O speed |

## Error Handling

```
User Action → Frontend Validation
           → Send to Backend via IPC
           → Rust Error Handling
           ├─ Network errors (Ollama)
           ├─ File I/O errors
           ├─ Image processing errors
           └─ Return error string to frontend
           → Frontend Display Error Message
```

## Security Considerations

- Images handled locally (no external transmission except to Ollama)
- No authentication required (desktop app)
- Project files are plain JSON (encrypt if needed)
- Ollama connection should be on localhost or trusted network

---

For implementation details, see specific files in `DEVELOPMENT.md`
