# Project File Structure & Purposes

Quick reference guide for all files in the Censor project.

## Directory Layout & File Purposes

```
Censor/
│
├── 📄 README.md                    # Main user documentation and feature overview
├── 📄 INSTALL.md                   # Installation and setup instructions
├── 📄 DEVELOPMENT.md               # Developer guide and setup
├── 📄 ARCHITECTURE.md              # System architecture and design
├── 📄 CONTRIBUTING.md              # Contribution guidelines
├── 📄 CHANGELOG.md                 # Version history and changes
├── 📄 LICENSE                      # MIT License
│
├── 🏗️ Configuration Files
├── 📄 package.json                 # Node.js dependencies and scripts
├── 📄 tsconfig.json                # TypeScript compiler options
├── 📄 tsconfig.node.json           # TypeScript for Vite config
├── 📄 vite.config.ts               # Vite build configuration
├── 📄 .eslintrc.json               # ESLint code style rules
├── 📄 .gitignore                   # Git ignore patterns
├── 📄 .env.example                 # Environment variable template
├── 📄 index.html                   # HTML entry point
│
├── 📁 src/                         # Vue.js Frontend Source
│   ├── 📄 main.ts                  # Pinia & Vue initialization
│   ├── 📄 App.vue                  # Root component, workflow management
│   ├── 📄 style.css                # Global styles with dark theme
│   │
│   ├── 💫 components/              # Vue Components
│   │   ├── CanvasEditor.vue        # Interactive canvas for image & masks
│   │   ├── RegionList.vue          # Sidebar with region listing
│   │   └── SettingsModal.vue       # Settings dialog
│   │
│   ├── 🏪 stores/                  # Pinia State Management
│   │   ├── imageStore.ts           # Image operations and Ollama calls
│   │   ├── projectStore.ts         # Project save/load logic
│   │   └── configStore.ts          # Configuration state
│   │
│   └── 🛠️ utils/                   # Utility Functions
│       ├── api.ts                  # Tauri IPC command wrapper
│       └── imageUtils.ts           # Image processing helpers
│
├── 📁 src-tauri/                   # Rust Backend Source
│   ├── 📄 build.rs                 # Tauri build script
│   ├── 📄 tauri.conf.json          # Tauri configuration
│   ├── 📄 Cargo.toml               # Rust dependencies
│   │
│   └── 📁 src/                     # Rust Source Code
│       ├── 📄 main.rs              # Tauri app entry point & modules
│       ├── 📄 commands.rs          # IPC command handlers
│       ├── 📄 models.rs            # Project & detection data types
│       ├── 📄 ollama.rs            # Ollama API client
│       ├── 📄 image_processor.rs   # Image processing & censoring
│       ├── 📄 config.rs            # Configuration loader
│       ├── 📄 state.rs             # Application state types
│       └── 📄 build_config.rs      # Build-time configuration
│
├── 📁 public/                      # Static assets (placeholder)
│
└── 📄 build.sh / build.bat         # Production build scripts
```

## File Purposes & Key Content

### Documentation
- **README.md**: Feature overview, quick start, API reference
- **INSTALL.md**: System requirements, Ollama setup, troubleshooting
- **DEVELOPMENT.md**: Dev environment, project structure, workflow
- **ARCHITECTURE.md**: System design, data flow, components
- **CONTRIBUTING.md**: Contribution guidelines, code style
- **CHANGELOG.md**: Version history and roadmap
- **PROJECT_MAP.md**: This file

### Configuration
- **package.json**: npm scripts, Node dependencies
- **.eslintrc.json**: TypeScript/Vue code style
- **tsconfig.json**: TypeScript compiler settings
- **vite.config.ts**: Frontend build configuration
- **Cargo.toml**: Rust dependencies and profiles
- **tauri.conf.json**: Tauri window and security config

### Frontend (Vue.js)
- **main.ts**: Initializes Pinia store and Vue app
- **App.vue**: Main container, routing between screens (upload/editor)
- **style.css**: Dark theme with CSS variables
- **CanvasEditor.vue**: Canvas rendering, mask overlay, drawing
- **RegionList.vue**: Lists detected regions, method selector
- **SettingsModal.vue**: Ollama and detection configuration
- **imageStore.ts**: Encapsulates Ollama API calls from frontend
- **projectStore.ts**: Project CRUD operations
- **configStore.ts**: Application settings state
- **api.ts**: Tauri IPC command wrappers
- **imageUtils.ts**: Polygon math, polygon simplification, etc.

### Backend (Rust)
- **main.rs**: Tauri initialization, command dispatcher
- **commands.rs**: IPC handlers for all frontend actions
- **models.rs**: `CensoringRegion`, `CensoringProject`, `DetectionType`, etc.
- **ollama.rs**: HTTP client for Ollama API
- **image_processor.rs**: 
  - Load/save images
  - Apply censoring effects (blur, pixelate, blackout)
  - Polygon mask conversion from bounding boxes
  - Polygon fill algorithm (scanline)
- **config.rs**: Locale configuration with nudity standards
- **state.rs**: `AppState` type definitions
- **build_config.rs**: Performance tuning constants

### Build Scripts
- **build.bat**: Windows build script
- **build.sh**: Linux/macOS build script
- **Both**: Run tests, compile frontend, build Tauri app

## How to Find Everything

### "I want to modify the UI..."
→ Look in `src/components/` and `src/style.css`

### "I want to change how censoring works..."
→ Check `src-tauri/src/image_processor.rs`

### "I want to add a new detection method..."
→ Update `src-tauri/src/ollama.rs` and `src-tauri/src/models.rs`

### "I want to support a new locale..."
→ Modify `src-tauri/src/config.rs`

### "I want to understand the architecture..."
→ Read `ARCHITECTURE.md`

### "I want to contribute..."
→ Start with `CONTRIBUTING.md` then `DEVELOPMENT.md`

### "I'm setting up a new machine..."
→ Follow `INSTALL.md`

### "Something is broken..."
→ Check `README.md` troubleshooting section

### "I want to build a release..."
→ Run `build.sh` or `build.bat`

## Development Tips

### Quick Edits
```bash
# Most changes don't require rebuild
npm run tauri:dev
# Modify files and see changes instantly (hot reload)
```

### Debugging
```bash
# Backend errors visible in terminal
# Frontend errors visible in DevTools
# Use Rust logs: RUST_LOG=debug npm run tauri:dev
```

### Adding Features
1. **Data Model**: Update `src-tauri/src/models.rs`
2. **Backend Logic**: Implement in `src-tauri/src/`
3. **IPC Handler**: Add to `src-tauri/src/commands.rs`
4. **Frontend UI**: Add Vue component or update existing
5. **API Binding**: Add to `src/utils/api.ts`
6. **State**: Add to appropriate store in `src/stores/`

---

Generated: February 10, 2026
For latest updates, check README.md
