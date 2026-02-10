# Installation & Setup Guide

Complete step-by-step guide to get Censor running on your system.

## Table of Contents
1. [System Requirements](#system-requirements)
2. [Install Ollama](#install-ollama)
3. [Install Development Tools (for building from source)](#install-development-tools)
4. [Download & Run Censor](#download--run-censor)
5. [Configuration](#configuration)
6. [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum
- **OS**: Windows 10+, macOS 10.13+, or Linux (Ubuntu 18.04+)
- **RAM**: 8 GB (4 GB minimum, but slower)
- **Disk**: 20 GB (for Ollama models)
- **GPU**: Not required, but highly recommended for Ollama

### Recommended
- **OS**: Windows 11, macOS 12+, Ubuntu 22.04+
- **RAM**: 16 GB
- **Disk**: 30 GB+ SSD
- **GPU**: NVIDIA/AMD with CUDA/ROCm support for faster processing

## Install Ollama

Ollama is required to run the vision model. Install it before using Censor.

### macOS / Linux

```bash
curl -L https://ollama.ai/install.sh | sh

# Start Ollama (runs as service on macOS, background on Linux)
ollama serve
```

### Windows

1. Download from https://ollama.ai/download
2. Run the installer
3. Start Ollama (automatically runs in background)

### Verify Installation

```bash
# Check if Ollama is running (should return model list)
curl http://localhost:11434/api/tags

# If empty, pull the model
ollama pull qwen3-vl:30b
```

### Download Qwen3-VL Model

```bash
# This will take 5-30 minutes depending on internet speed
ollama pull qwen3-vl:30b

# Verify download
ollama list
# Should show: qwen3-vl:30b  (your qwen3-vl version)
```

**Note**: The model is ~18-20 GB. Ensure you have sufficient disk space.

## Install Development Tools (for building from source)

Skip this section if you're using a pre-built release.

### Windows

1. **Install Rust**
   ```powershell
   # Download and run from https://rustup.rs/
   # Or use:
   irm https://astro.build/install/windows.ps1 | iex
   
   # Verify
   rustc --version
   cargo --version
   ```

2. **Install Node.js**
   - Download from https://nodejs.org/
   - Choose LTS version (18+)
   - Run installer

3. **Install Git** (optional, for version control)
   - Download from https://git-scm.com/

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (using Homebrew recommended)
brew install node

# Verify
rustc --version
node --version
```

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
sudo apt update
sudo apt install nodejs npm

# Verify
rustc --version
node --version
```

## Download & Run Censor

### Option 1: Pre-built Release (Recommended)

1. Download from [GitHub Releases](https://github.com/yourrepo/censor/releases)
2. Extract/Install
3. Run the application
4. Skip to [Configuration](#configuration)

**File Locations After Installation:**
- **Windows**: `C:\Program Files\Censor\censor.exe`
- **macOS**: `/Applications/Censor.app`
- **Linux**: `/opt/censor/censor`

### Option 2: Build from Source

1. **Clone Repository**
   ```bash
   git clone https://github.com/yourrepo/censor.git
   cd censor
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Development Mode** (with hot reload)
   ```bash
   npm run tauri:dev
   ```

4. **Production Build**
   ```bash
   # Windows/macOS/Linux
   npm run tauri:build
   
   # Output: src-tauri/target/release/censor[.exe]
   ```

## Configuration

### First Launch

1. **Open Censor**
2. **Settings** ⚙️ button in top-right
3. **Verify Ollama Endpoint**
   - Default: `http://localhost:11434`
   - Click **Test Connection**
   - Should show: ✓ Connected successfully!

### Settings

#### Ollama Configuration
- **Endpoint**: Local or remote Ollama server
- **Model**: Default is `qwen3-vl:30b`
  - For faster processing on weak hardware: `qwen2-vl:7b`
  - For better accuracy: `qwen3-vl:75b` (if you have 75GB+ VRAM)

#### Detection Settings
- **Detect Nudity**: Enable/disable nudity detection
- **Nudity Locale**: 
  - **US**: American decency standards
  - **EU**: European standards (more lenient)
  - **JP**: Japanese standards (strictest)
- **Detect Text/PII**: Enable text and personal information detection

#### Default Censoring Method
- **Blur**: Gaussian blur (fastest)
- **Pixelate**: Mosaic effect (moderate)
- **Blackout**: Black overlay (slightly slower)

### Performance Tuning

If processing is slow:

1. **Use Blur instead of Pixelate**
   - Blur is 20-30% faster

2. **Enable GPU Acceleration for Ollama**
   ```bash
   # NVIDIA GPUs
   ollama serve --gpu

   # AMD GPUs (ROCm)
   ollama serve --gpu=rocm
   ```

3. **Use Smaller Model** (if accuracy acceptable)
   - `ollama pull qwen2-vl:7b`
   - Much faster on weak hardware

4. **Resize Images Before Processing**
   - Process at 2048x2048 instead of 4096x4096

## Troubleshooting

### "Cannot connect to Ollama"

**Solution**:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# If not responding, start Ollama
ollama serve

# If on remote machine, verify network connection
ping your-ollama-server
```

### "Model not found"

**Solution**:
```bash
# Check available models
ollama list

# If empty or wrong model, pull it
ollama pull qwen3-vl:30b

# This takes 5-30 minutes
```

### Application won't start

**Windows**:
```powershell
# Check for .NET requirements
# If error about runtime, install from:
# https://dotnet.microsoft.com/download

# Try running with administrator privileges
```

**macOS**:
```bash
# If security warning appears:
# System Preferences → Security & Privacy
# Click "Open Anyway" for Censor

# Check logs
log stream --predicate 'process == "censor"'
```

**Linux**:
```bash
# Check dependencies
ldd ./censor

# May need to install libraries
sudo apt install libg++ libssl-dev pkg-config
```

### Out of Memory

**Symptoms**: Application crashes or becomes unresponsive during image processing

**Solutions**:
1. Close other applications
2. Reduce image resolution before uploading
3. Use smaller Ollama model
4. Increase system virtual memory/page file

### GPU Not Used by Ollama

```bash
# Check NVIDIA
nvidia-smi

# Check if using GPU in Ollama
ollama serve

# Should show CUDA/GPU in output

# Install NVIDIA Container Runtime if needed
# https://github.com/NVIDIA/nvidia-docker
```

### Network Issues with Remote Ollama

```bash
# Test connectivity
curl http://your-server:11434/api/tags

# If timeout, check firewall
# If refused, ensure Ollama listening on correct interface

# Start Ollama on specific interface
OLLAMA_HOST=0.0.0.0:11434 ollama serve
```

## Uninstallation

### Windows
- Control Panel → Programs → Uninstall
- Or run uninstaller from installation folder

### macOS
- Drag Censor.app to Trash
- Optional: Remove configuration files
  ```bash
  rm -rf ~/Library/Application\ Support/com.censor.app
  ```

### Linux
- Package manager: `sudo apt remove censor`
- Or delete installation directory

## Next Steps

1. **Load First Image**: Click upload area or drag image
2. **Review**: Check detected regions in sidebar
3. **Adjust**: Drag masks if needed
4. **Censor**: Select method for each region
5. **Export**: Save censored image or project

See [README.md](README.md) for user guide and [DEVELOPMENT.md](DEVELOPMENT.md) for development.

---

**Still having issues?** Check [GitHub Issues](https://github.com/yourrepo/censor/issues) or create new issue.
