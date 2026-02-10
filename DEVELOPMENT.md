# Development Guide

This document provides comprehensive guidance for developing and maintaining the Censor application.

## Development Environment Setup

### Prerequisites

- **Node.js**: 18.0.0 or higher
- **Rust**: 1.70.0 or higher
- **Tauri CLI**: 1.5.x
- **Ollama**: Latest version with `qwen3-vl:30b` model

### Quick Start

1. **Clone and Install**
   ```bash
   cd d:\Projects\Censor
   npm install
   npm install -g @tauri-apps/cli
   ```

2. **Start Ollama** (in separate terminal)
   ```bash
   ollama serve
   # Ensure qwen3-vl:30b is available
   ollama list
   ```

3. **Run Development Server**
   ```bash
   npm run tauri:dev
   ```

## Project Structure

```
Censor/
├── src/                          # Vue.js Frontend
│   ├── components/
│   │   ├── CanvasEditor.vue     # Interactive canvas for mask editing
│   │   ├── RegionList.vue       # Sidebar with region management
│   │   └── SettingsModal.vue    # Configuration UI
│   ├── stores/
│   │   ├── imageStore.ts        # Image operations state
│   │   ├── projectStore.ts      # Project management state
│   │   └── configStore.ts       # Configuration state
│   ├── utils/
│   │   ├── api.ts              # Tauri API wrapper
│   │   └── imageUtils.ts       # Image processing utilities
│   ├── App.vue                  # Main application component
│   ├── main.ts                  # Vue entry point
│   └── style.css                # Global styles (dark theme)
├── src-tauri/                   # Rust Backend
│   ├── src/
│   │   ├── main.rs             # Tauri app initialization
│   │   ├── commands.rs         # IPC command handlers
│   │   ├── models.rs           # Data structures
│   │   ├── ollama.rs           # Ollama API client
│   │   ├── image_processor.rs  # Image processing logic
│   │   ├── config.rs           # Configuration management
│   │   └── state.rs            # Application state
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri configuration
│   └── build.rs                # Build script
├── index.html                   # HTML entry point
├── vite.config.ts              # Vite configuration
├── tsconfig.json               # TypeScript configuration
├── package.json                # Node dependencies
└── README.md                    # User documentation
```

## Key Features & Implementation

### 1. Image Detection via Ollama

**File**: `src-tauri/src/ollama.rs`

The `OllamaClient` sends images to the Qwen3-VL model and parses responses to extract:
- Detection type (nudity, text, etc.)
- Bounding boxes
- Confidence scores
- Labels

```rust
let client = OllamaClient::new(endpoint, model);
let response = client.detect_content(image_base64, "nudity", "us").await?;
```

**Extending Detection**:
- Modify prompts in `ollama.rs` to add new detection types
- Locales are configured in `config.rs` with customizable standards

### 2. Polygon-Based Masking

**File**: `src-tauri/src/image_processor.rs`

Instead of rectangles, masks are polygons with natural shapes:
- **Circular masks** for body parts (nipples, genitals) using `bbox_to_circular_mask()`
- **Blob masks** for irregular shapes using `bbox_to_blob_mask()`
- Includes polygon fill algorithm using scanline technique

```rust
let polygon = ImageProcessor::bbox_to_circular_mask(x, y, w, h);
ImageProcessor::apply_blackout(&mut image, &polygon)?;
```

### 3. Multiple Censoring Methods

Applied to each region individually:
- **Blackout**: Solid black overlay
- **Blur**: Gaussian blur with configurable radius
- **Pixelate**: Mosaic/pixelation effect

### 4. Human-in-the-Loop Adjustment

**File**: `src/components/CanvasEditor.vue`

Interactive features:
- Visualize detected masks as polygon outlines
- Drag control points to adjust polygons
- Freehand drawing of new masks
- Real-time preview

### 5. Project Persistence

Projects saved as `.censor` files with:
- Original image data (base64)
- All detected regions with metadata
- User adjustments to masks
- Settings and preferences

## Development Workflow

### Adding a New Detection Type

1. **Update models** (`src-tauri/src/models.rs`):
   ```rust
   pub enum DetectionType {
       Nudity,
       TextRedaction,
       YourNewType,  // Add here
   }
   ```

2. **Update Ollama prompt** (`src-tauri/src/ollama.rs`):
   ```rust
   "your_new_type" => "Detect XYZ in image..."
   ```

3. **Update frontend settings** (`src/components/SettingsModal.vue`):
   ```vue
   <input v-model="detectionSettings.detect_your_new_type" type="checkbox" />
   ```

### Adding a New Locale

1. **Add locale config** (`src-tauri/src/config.rs`):
   ```rust
   LocaleConfig {
       code: "xx".to_string(),
       name: "Country".to_string(),
       nudity_standards: NudityStandards {
           censor_female_nipples: true,
           censor_genitals: true,
           censor_buttocks: false,
           strictness_level: 5,
       },
   }
   ```

2. **Update UI** (`src/components/SettingsModal.vue`):
   ```vue
   <option value="xx">Country</option>
   ```

### Adding a New Censoring Effect

1. **Implement in ImageProcessor** (`src-tauri/src/image_processor.rs`):
   ```rust
   fn apply_swirl(image: &mut RgbaImage, polygon: &[(f64, f64)]) -> Result<(), String> {
       // Implementation here
   }
   ```

2. **Update CensoringMethod enum** (`src-tauri/src/models.rs`):
   ```rust
   pub enum CensoringMethod {
       Blur,
       MotionBlur,  // Add here
   }
   ```

3. **Update command handler** (`src-tauri/src/commands.rs`):
   ```rust
   CensoringMethod::MotionBlur => {
       Self::apply_motion_blur(&mut image, &region.polygon)?;
   }
   ```

## Building for Production

```bash
# Build optimized binary
npm run tauri:build

# Output: src-tauri/target/release/censor.exe (Windows)
#         src-tauri/target/release/censor (macOS/Linux)
```

## Testing

### Manual Testing Checklist

- [ ] Image upload works
- [ ] Ollama detection completes
- [ ] Regions display correctly as polygons
- [ ] Masks can be adjusted by dragging
- [ ] New masks can be drawn
- [ ] All censoring methods work
- [ ] Project saves and loads correctly
- [ ] Settings persist
- [ ] Remote Ollama connection works

### Performance Testing

```bash
# Test large images (5000x5000+ pixels)
# Test complex scenes (100+ detected regions)
# Profile CPU usage during censoring
# Measure Ollama response times
```

## Debugging

### Backend Debugging (Rust)

```bash
RUST_LOG=debug npm run tauri:dev
```

### Frontend Debugging

- DevTools available in development mode
- Canvas rendering can be inspected
- Network calls to backend visible in console

### Ollama Issues

```bash
# Check Ollama is running
curl http://localhost:11434/api/tags

# Check model availability
ollama list

# Stream logs
ollama serve
```

## Performance Optimization Tips

1. **Image Processing**:
   - Resize large images before processing
   - Use blur instead of pixelate for faster rendering
   - Cache processed results

2. **Ollama**:
   - Enable GPU acceleration if available
   - Consider running on server for batch processing
   - Tune model quantization for speed vs quality

3. **Frontend**:
   - Use requestAnimationFrame for canvas drawing
   - Debounce frequent state updates
   - Lazy-load components

## Common Issues & Solutions

### Build Fails with Linking Errors
```bash
# Windows MSVC setup
cargo install cargo-vcvars
cargo vcvars setup

# Or use GNU toolchain
rustup target install x86_64-pc-windows-gnu
```

### Memory Issues with Large Images
- Process images in streaming fashion
- Implement chunked image loading
- Use downsampling for preview

### Ollama Timeouts
- Increase timeout in `ollama.rs`
- Use smaller models for faster inference
- Run on GPU-accelerated hardware

## Code Style Guidelines

- **Rust**: Follow `rustfmt` defaults
- **TypeScript/Vue**: Use ESLint config
- **Comments**: Document non-obvious logic
- **Error Handling**: Use Result types, avoid unwrap()

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/your-feature

# Make changes
git add .
git commit -m "feat: describe changes"

# Push and create PR
git push origin feature/your-feature
```

## Release Checklist

- [ ] Update version in `package.json` and `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run full test suite
- [ ] Build production binaries
- [ ] Tag release: `git tag v1.x.x`
- [ ] Create GitHub release with binaries

## Resources

- [Tauri Documentation](https://tauri.app/docs)
- [Vue 3 Guide](https://vuejs.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Ollama API](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [imageproc crate](https://docs.rs/imageproc/)

---

For questions or issues, refer to the main README.md or create an issue.
