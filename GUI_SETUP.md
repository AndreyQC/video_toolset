# GUI Setup and Usage Guide

This guide explains how to set up and run the Video Scene Detection Tool with the Tauri GUI.

## Prerequisites

### 1. Install Node.js and npm
```bash
# Install Node.js (version 18 or higher)
# Download from: https://nodejs.org/

# Verify installation
node --version
npm --version
```

### 2. Install Tauri CLI
```bash
# Install @tauri-apps/cli globally
npm install -g @tauri-apps/cli

# Or use npx to run without installing globally
npx @tauri-apps/cli --version
```

### 3. Install Frontend Dependencies
```bash
# Navigate to project root
cd video_toolset

# Install dependencies
npm install
```

## Development

### Run in Development Mode
This command starts both the Rust backend and the Svelte frontend in development mode:

```bash
cargo tauri dev
```

Or using npx:
```bash
npx tauri dev
```

**What happens:**
1. Starts the Vite dev server on port 1420
2. Compiles and runs the Rust backend with Tauri
3. Opens the desktop application window
4. Supports hot reload for frontend changes

### Build for Production

```bash
# Build the application
cargo tauri build
```

This creates distributable packages in `src-tauri/target/release/bundle/`.

## Project Structure

```
video_toolset/
├── src/
│   ├── gui/              # Tauri backend commands
│   │   └── mod.rs
│   ├── components/       # Svelte components
│   │   ├── VideoPlayer.svelte
│   │   ├── RoiEditor.svelte
│   │   ├── ParameterPanel.svelte
│   │   └── SceneList.svelte
│   ├── App.svelte       # Main Svelte component
│   ├── main.js          # Frontend entry point
│   └── main.rs          # Rust entry point
├── index.html            # HTML template
├── package.json          # Node.js dependencies
├── vite.config.js        # Vite configuration
├── tauri.conf.json       # Tauri configuration
└── Cargo.toml           # Rust dependencies
```

## Frontend Components

### 1. VideoPlayer.svelte
- Displays the selected video file
- Video controls (play, pause, seek)
- Loads video when file is selected

### 2. RoiEditor.svelte
- Interactive ROI editor with drag-and-drop
- Input fields for precise coordinates
- "Center" and "Fit to Video" buttons
- Canvas overlay for visual editing

### 3. ParameterPanel.svelte
- Threshold slider (0-255)
- Minimum scene length slider
- Blur size slider
- Real-time parameter adjustment

### 4. SceneList.svelte
- Displays detected scenes
- Shows timestamp and frame number
- Scrollable list view

### 5. App.svelte
- Main layout and component coordination
- State management
- Invokes Tauri backend commands

## Backend Commands (Tauri API)

The GUI frontend communicates with the Rust backend via these commands:

### Configuration Management
- `load_config(config_path)` - Load YAML configuration
- `save_config(config_path, config)` - Save configuration to YAML
- `get_config()` - Get current configuration
- `update_roi(roi)` - Update ROI parameters
- `get_roi()` - Get current ROI
- `update_threshold(threshold)` - Update detection threshold
- `update_min_scene_len(len)` - Update minimum scene length
- `update_blur_size(size)` - Update blur kernel size

### Video Processing
- `detect_scenes(video_path, output_dir)` - Run scene detection
- `get_scenes()` - Get detected scenes
- `get_video_metadata(video_path)` - Get video information

## Usage Workflow

1. **Select Video**
   - Click "Choose File" and select a video file
   - Click "Load Video" to load metadata

2. **Adjust ROI** (Optional)
   - Drag on the canvas to select region of interest
   - Or use input fields for precise coordinates
   - Click "Fit to Video" to analyze entire frame

3. **Configure Parameters**
   - Adjust threshold for sensitivity
   - Set minimum scene length
   - Configure blur size for noise reduction

4. **Run Detection**
   - Click "Run Detection"
   - Progress is shown in status bar
   - Results appear in the scene list

5. **Save Configuration**
   - Click "Save Config" to save current settings
   - Configuration saved to YAML file

## Troubleshooting

### Build Errors

#### "Failed to find node_modules"
```bash
npm install
```

#### "Failed to compile Svelte"
- Check Svelte component syntax
- Ensure all imports are correct
- Run `npm run dev` to check frontend separately

#### "Rust compilation errors"
- Ensure all Rust dependencies are installed
- Check that OpenCV is properly configured
- Run `cargo check` to verify

### Runtime Errors

#### "Failed to open video"
- Check file path format (must be absolute path)
- Verify video codec is supported
- Ensure video file is not corrupted

#### "Tauri command failed"
- Check backend is properly registered
- Verify function signatures match
- Check error messages in dev tools console

### Development Issues

#### Frontend not loading
- Check that port 1420 is available
- Restart the dev server: `cargo tauri dev`

#### Changes not reflected
- Ensure hot reload is working
- Check browser console for errors
- Restart dev server if needed

## Environment Variables

Ensure these are set for OpenCV (Windows):

```powershell
$env:OPENCV_LINK_LIBS = "opencv_world4"
$env:OPENCV_LINK_PATHS = "C:\vcpkg\installed\x64-windows\lib"
$env:OPENCV_INCLUDE_PATHS = "C:\vcpkg\installed\x64-windows\include"
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

## Performance Tips

1. **Use Release Build** for better performance:
   ```bash
   cargo tauri build --release
   ```

2. **Optimize Video Size** for faster processing

3. **Use ROI** to reduce analysis area

4. **Adjust Parameters** for your video type

## Next Steps

### Enhancements
- [ ] Add video thumbnail generation for scenes
- [ ] Export scene list to CSV/TXT
- [ ] Batch processing multiple videos
- [ ] Scene preview with thumbnails
- [ ] Playback synchronized with scene list

### Advanced Features
- [ ] Multiple ROI support
- [ ] Custom detection algorithms
- [ ] Heatmap visualization
- [ ] Frame difference preview
- [ ] Performance profiling tools
