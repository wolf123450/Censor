# Changelog

All notable changes to the Censor project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-10

### Added
- Initial production release
- Image content detection via Ollama with Qwen3-VL:30b
- Support for nudity detection with locale-specific standards (US, EU, Japan)
- Text/PII redaction detection (names, phone numbers, emails, credit cards)
- Polygon-based masking system with blob and circular mask generation
- Three censoring methods: Blackout, Blur, Pixelate
- Interactive canvas editor for mask adjustment
- Real-time polygon control point manipulation
- Freehand mask drawing capability
- Project management system with .censor file format
- Extensible locale and detection configuration system
- Light/dark theme with professional dark-first design
- Tauri desktop application framework
- Vue.js 3 frontend with Pinia state management
- Comprehensive error handling and logging
- Ollama endpoint configuration (local and remote)
- Batch region processing
- Image export in multiple formats

### Technical Details
- Built with Rust backend for performance
- Custom polygon filling algorithm for efficient masking
- Gaussian blur and pixelation effects
- Base64 image encoding for seamless IPC
- Async image processing with Tokio
- Type-safe Rust and TypeScript throughout

## [Future Releases]

### Planned Features
- Batch processing of multiple images
- Video content moderation
- Custom model support beyond Qwen3-VL
- Advanced ML model parameters configuration
- Mask preset library
- Undo/Redo functionality
- Image comparison before/after
- Multiple language UI support
- Cloud integration (optional)
- Command-line interface
- Plugin system for custom effects

### Performance Improvements
- GPU-accelerated image processing
- Multithreaded region processing
- Progressive image loading
- Mask caching

### UI/UX Enhancements
- Keyboard shortcuts
- Right-click context menus
- Mask templates/presets
- Zoom and pan on canvas
- History panel
- Region filtering and search
- Drag-and-drop region reordering

---

For detailed technical information, see [DEVELOPMENT.md](DEVELOPMENT.md)
and [ARCHITECTURE.md](ARCHITECTURE.md).
