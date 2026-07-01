# Censor - Image Content Moderation App

> **Status: Incomplete / Proof of Concept**
> This project was not brought to a working state. The core limitation is that the vision models tested (Qwen3-VL and others via Ollama) were not capable of reliably returning accurate region coordinates for censorship masking. The UI, project management, and masking pipeline are implemented, but the AI detection results are not usable in practice. Preserved here for reference and as a starting point if model capabilities improve.

A desktop application for intelligent image content moderation using Ollama's Qwen3-VL vision model. Built with Tauri and Vue.js.

## Features

- 🎯 **Intelligent Content Detection** via Ollama using Qwen3-VL:30B
  - Nudity detection with locale-specific standards (US, EU, Japan)
  - Text/PII redaction (names, phone numbers, email, credit cards, etc.)
  - Custom keyword detection

- 🎨 **Advanced Masking & Censoring**
  - Polygon-based masks with blob/irregular shapes (not just rectangles)
  - Multiple censoring methods:
    - **Blackout** - Complete black coverage
    - **Blur** - Gaussian blur effect
    - **Pixelate** - Mosaic/pixelation effect
  - Apply multiple censoring methods to different regions in the same image

- 🖱️ **Human-in-the-Loop Adjustment**
  - Interactive canvas for adjusting detected regions
  - Draw custom masks freehand
  - Modify existing masks by dragging control points
  - Real-time preview of censoring effects

- 💾 **Project Management**
  - Save projects with all metadata and mask data
  - Load previous projects for adjustment
  - Export censored images in PNG/JPG format

- ⚙️ **Flexible Configuration**
  - Easy Ollama endpoint configuration (local or remote)
  - Customizable detection settings per locale
  - Default censoring method selection
  - Extensible architecture for adding new locales

## Tech Stack

- **Frontend**: Vue.js 3 + TypeScript + Pinia
- **Desktop**: Tauri (Rust-based, lightweight)
- **Backend**: Rust with image processing capabilities
- **Vision Model**: Ollama with qwen3-vl:30b
- **Image Processing**: 
  - `image` crate for loading/saving
  - `imageproc` for polygon filling and effects
  - Custom polygon masking algorithm

## Requirements

- [Ollama](https://ollama.ai/) installed and running locally or accessible remotely
- `qwen3-vl:30b` model pulled in Ollama: `ollama pull qwen3-vl:30b`
- Node.js 18+ (for development)
- Rust 1.70+ (for building)

## Installation & Setup

### Prerequisites

1. **Install Ollama**
   ```bash
   # macOS/Linux
   curl -L https://ollama.ai/install.sh | sh
   
   # Or download from https://ollama.ai
   ```

2. **Pull the Qwen3-VL model**
   ```bash
   ollama pull qwen3-vl:30b
   ```

3. **Start Ollama server** (runs on `http://localhost:11434` by default)
   ```bash
   ollama serve
   ```

### Build & Development

1. **Install dependencies**
   ```bash
   npm install
   ```

2. **Development mode** (with hot reload)
   ```bash
   npm run tauri:dev
   ```

3. **Build for production**
   ```bash
   npm run tauri:build
   ```

   The built executable will be in `src-tauri/target/release/censor`

## Usage Guide

### Basic Workflow

1. **Load Image**
   - Click upload area or drag-and-drop an image file
   - App automatically sends image to Ollama for analysis

2. **Review Detected Regions**
   - Left sidebar shows all detected censoring regions
   - Click region to select it
   - View detection type and confidence score

3. **Adjust Masks**
   - Toggle "Show Masks" to visualize detected regions
   - Click control points (circles) to move/reshape polygons
   - Click "Draw Mask" to manually add new regions

4. **Apply Censoring**
   - Select censoring method for each region (Blur/Pixelate/Blackout)
   - Click "Apply Effects" to process the image
   - Preview censored result

5. **Save & Export**
   - Save project to reload later with mask data
   - Export censored image as PNG/JPG

### Settings

- **Ollama Endpoint**: Configure local or remote Ollama server
- **Model**: Select vision model (default: qwen3-vl:30b)
- **Detection Settings**:
  - Toggle nudity detection
  - Select locale for decency standards
  - Toggle text/PII redaction
- **Default Censoring Method**: Choose default effect

## Project File Format

Project files (`.censor`) are JSON-based containing:
```json
{
  "id": "project-id",
  "name": "image-name",
  "image_path": "/path/to/original",
  "original_image_data": "base64-encoded-image",
  "regions": [
    {
      "id": "region-id",
      "detection_type": "nudity|text_redaction|custom",
      "mask_type": "polygon|blob|pointcloud",
      "censoring_method": "blackout|blur|pixelate",
      "polygon": [[x1, y1], [x2, y2], ...],
      "confidence": 0.95,
      "metadata": {...}
    }
  ],
  "created_at": "2026-02-10T...",
  "modified_at": "2026-02-10T...",
  "settings": {...}
}
```

## Architecture

### Frontend (Vue.js)
- **App.vue**: Main container and workflow management
- **CanvasEditor.vue**: Interactive canvas for mask visualization and drawing
- **RegionList.vue**: Sidebar with region details and controls
- **SettingsModal.vue**: Configuration interface
- **Stores** (Pinia):
  - `imageStore`: Image operations
  - `projectStore`: Project management
  - `configStore`: Configuration

### Backend (Rust/Tauri)
- **main.rs**: Tauri app initialization
- **commands.rs**: IPC command handlers
- **ollama.rs**: Ollama API integration
- **image_processor.rs**: Image processing and mask application
- **config.rs**: Configuration management
- **models.rs**: Data structures

## Extending with New Locales

Add a new locale to `src-tauri/src/config.rs`:

```rust
LocaleConfig {
    code: "uk".to_string(),
    name: "United Kingdom".to_string(),
    nudity_standards: NudityStandards {
        censor_female_nipples: true,
        censor_genitals: true,
        censor_buttocks: false,
        strictness_level: 5,
    },
}
```

## Performance Optimization

- Caches Ollama responses when possible
- Efficient polygon-based masking (no unnecessary rectangle processing)
- Multi-threaded image processing via Tokio
- Optimized builds with LTO and single codegen unit

## Troubleshooting

### "Cannot connect to Ollama"
- Ensure Ollama is running: `ollama serve`
- Check endpoint in Settings (default: `http://localhost:11434`)
- Verify model is pulled: `ollama list`

### "Model not found"
- Pull the model: `ollama pull qwen3-vl:30b`
- Verify: `ollama list`

### Build fails
- Ensure Rust 1.70+: `rustc --version`
- Update Rust: `rustup update`
- Clear build cache: `cargo clean`

### Image processing is slow
- Reduce image resolution before processing
- Use blur instead of pixelate for faster processing
- Ensure Ollama has GPU acceleration if available

## API Reference

### Tauri Commands

```typescript
// Detect content in image
detect_content(image_base64: string, detection_type: 'nudity' | 'text_redaction', locale: string, config_json: string): Promise<CensoringRegion[]>

// Apply censoring effects to image
apply_censoring(image_base64: string, regions_json: string): Promise<string>

// Save project to file
save_project(project_json: string, save_path: string): Promise<{saved: boolean, path: string}>

// Load project from file
load_project(project_path: string): Promise<CensoringProject>

// Get configuration
get_config(): Promise<AppConfig>

// Update configuration
set_config(config_json: string): Promise<{updated: boolean}>

// Adjust censoring region
adjust_region(region_json: string, new_polygon_json: string): Promise<CensoringRegion>
```

## License

MIT License - See LICENSE file for details

## Contributing

This is a production-ready application. For improvements:
1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## Support

For issues and feature requests, please open an issue on the GitHub repository.

---

**Built with ❤️ for content moderation professionals**
