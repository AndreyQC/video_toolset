# Video Scene Detection Tool

A high-performance Rust-based scene detection tool for MPEG video files using OpenCV. Features both CLI and GUI interfaces with advanced ROI (Region of Interest) analysis.

## Features

- 🎬 **Scene Change Detection**: Detects scene changes in MPEG video files using frame difference analysis
- 🎯 **ROI Analysis**: Analyze only specific regions of interest for more accurate detection
- ⚙️ **Configurable Parameters**: Adjustable threshold, minimum scene length, and blur settings
- 📊 **Multiple Interfaces**: CLI and GUI (Tauri) support
- 💾 **JSON Output**: Structured metadata output with timestamps and frame information
- 🏗️ **Modular Architecture**: Clean separation between core processing and UI layers

## Architecture

```
src/
├── main.rs              # Entry point with mode switching
├── core/                # Video processing logic (UI-agnostic)
│   ├── config.rs        # YAML configuration handling
│   ├── processor.rs     # Scene detection algorithm
│   └── output.rs        # JSON metadata generation
├── cli/                 # Command-line interface
└── gui/                 # Tauri GUI (optional)
```

## Prerequisites

### Required Software
- **Rust** (latest stable) - [Install here](https://www.rust-lang.org/tools/install)
- **LLVM/Clang** - Required for opencv crate compilation
- **OpenCV 4.x** - See [SETUP_OPENCV_WINDOWS.md](./SETUP_OPENCV_WINDOWS.md) for detailed Windows setup
- **Visual Studio Build Tools** or **MSVC toolchain**

### Platform Support
- ✅ Windows 10/11 (primary target)
- ⚠️ Linux (with OpenCV installed)
- ⚠️ macOS (with OpenCV installed)

## OpenCV Setup (Windows)

⚠️ **IMPORTANT**: The opencv crate requires LLVM/Clang to build. Follow the detailed guide in [SETUP_OPENCV_WINDOWS.md](./SETUP_OPENCV_WINDOWS.md).

Quick setup using vcpkg (recommended):

```powershell
# Install LLVM
winget install LLVM.LLVM

# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install

# Install OpenCV
.\vcpkg install opencv4:x64-windows

# Set environment variables
[Environment]::SetEnvironmentVariable("OPENCV_LINK_LIBS", "opencv_world4", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_PATHS", "C:\vcpkg\installed\x64-windows\lib", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_INCLUDE_PATHS", "C:\vcpkg\installed\x64-windows\include", "Machine")
[Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "Machine")

# Add to PATH
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\vcpkg\installed\x64-windows\bin", "Machine")
```

## Building

### CLI Only (Default)
```bash
cargo build --release
```

### With GUI Support
```bash
cargo build --release --features gui
```

### All Features
```bash
cargo build --release --all-features
```

## Usage

### Configuration File

Create a YAML configuration file (see `config.example.yaml`):

```yaml
profile_name: "default"

# Detection threshold (0.0 - 255.0)
threshold: 30.0

# Region of Interest (ROI) - absolute pixel coordinates
roi:
  x: 0
  y: 0
  width: 1920
  height: 1080

# Minimum scene length in frames
min_scene_len: 15

# Gaussian blur kernel size (must be odd)
blur_size: 5
```

### CLI Mode

Run scene detection from the command line:

```bash
# Basic usage
cargo run --release --bin video_toolset -- --video input.mp4 --config config.yaml

# Or after building
.\target\release\video_toolset.exe --video input.mp4 --config config.yaml
```

**CLI Options:**
- `--video, -v`: Path to input video file
- `--config, -c`: Path to configuration YAML file
- `--output, -o`: Output directory (default: `./Output/`)

### GUI Mode

**Prerequisites:**
- Node.js 18+ and npm
- Tauri CLI: `npm install -g @tauri-apps/cli`
- Frontend dependencies: `npm install`

**Launch the GUI:**

```bash
# Development mode (with hot reload)
cargo tauri dev

# Production build
cargo tauri build
```

For detailed GUI setup instructions, see [GUI_SETUP.md](./GUI_SETUP.md).

The GUI allows you to:
- Select video files with file browser
- Edit ROI parameters visually with drag-and-drop
- Adjust detection parameters in real-time
- View scene detection results with timestamps
- Save and load configuration profiles

## Scene Detection Algorithm

The tool uses a frame-difference-based algorithm:

1. **Frame Extraction**: Read video frames using OpenCV's VideoCapture
2. **ROI Extraction**: Extract specified region of interest
3. **Preprocessing**: Convert to grayscale and apply Gaussian blur
4. **Difference Analysis**: Calculate absolute difference with previous frame
5. **Mean Calculation**: Compute average pixel difference in ROI
6. **Threshold Comparison**: Detect scene change when mean exceeds threshold
7. **Filtering**: Enforce minimum scene length to avoid false positives

### Algorithm Steps

```
Frame N → Extract ROI → Grayscale → Gaussian Blur
                                      ↓
Frame N-1 → Extract ROI → Grayscale → Gaussian Blur
                                      ↓
                                absdiff() → mean() → compare(threshold)
```

## Output

The tool generates:

- **Output Directory**: `./Output/` (or custom via `--output`)
- **metadata.json**: Scene timestamps and metadata

Example `metadata.json`:

```json
{
  "profile_name": "default",
  "video_path": "input.mp4",
  "video_info": {
    "width": 1920,
    "height": 1080,
    "fps": 30.0,
    "frame_count": 54000
  },
  "scenes": [
    {
      "scene_id": 1,
      "timestamp": "00:01:23.456",
      "frame_number": 2223
    },
    {
      "scene_id": 2,
      "timestamp": "00:03:45.123",
      "frame_number": 6754
    }
  ],
  "detection_config": {
    "threshold": 30.0,
    "roi": {
      "x": 0,
      "y": 0,
      "width": 1920,
      "height": 1080
    },
    "min_scene_len": 15
  }
}
```

## Configuration Parameters

### threshold
- **Range**: 0.0 - 255.0
- **Default**: 30.0
- **Description**: Sensitivity for scene change detection. Lower values detect more changes.

### ROI (Region of Interest)
- **x, y**: Top-left corner coordinates (absolute pixels)
- **width, height**: Size of analysis region
- **Description**: Only analyze this rectangular area for scene changes

### min_scene_len
- **Range**: Integer > 0
- **Default**: 15
- **Description**: Minimum frames between scene changes (prevents detecting rapid flashes)

### blur_size
- **Range**: Odd integers (1, 3, 5, 7, ...)
- **Default**: 5
- **Description**: Gaussian blur kernel size for noise reduction

## Troubleshooting

### Build Errors

#### "couldn't find clang.dll"
- Install LLVM and set `LIBCLANG_PATH` environment variable
- See [SETUP_OPENCV_WINDOWS.md](./SETUP_OPENCV_WINDOWS.md) for details

#### "opencv2/core.hpp not found"
- Verify `OPENCV_INCLUDE_PATHS` environment variable
- Check OpenCV installation

#### "cannot find -lopencv_world4"
- Verify `OPENCV_LINK_PATHS` environment variable
- Ensure OpenCV library files exist

#### "opencv_world4.dll not found"
- Add OpenCV bin directory to PATH
- Or copy DLL to executable directory

### Runtime Errors

#### "Failed to open video"
- Check video file path
- Verify video codec compatibility (MPEG recommended)
- Ensure video file is not corrupted

#### "ROI exceeds video dimensions"
- Check ROI coordinates and size
- Verify video resolution in metadata.json

## Development

### Project Structure

The project uses a modular architecture:

- **core**: UI-agnostic video processing logic
- **cli**: Command-line interface using clap
- **gui**: Tauri-based graphical interface (optional)

### Key Design Decisions

1. **Separation of Concerns**: Core processing logic is independent of UI
2. **Feature Flags**: GUI support is optional via `--features gui`
3. **Configuration-Driven**: All parameters controlled via YAML
4. **Type Safety**: Rust's strong typing for robust error handling

### Adding New Features

1. Scene detection algorithm improvements → `src/core/processor.rs`
2. New output formats → `src/core/output.rs`
3. UI enhancements → `src/cli/` or `src/gui/`
4. Configuration options → `src/core/config.rs`

## Performance Optimization

- **Release builds only** (LTO enabled in Cargo.toml)
- **ROI analysis** reduces computational load
- **Frame skipping** can be added for faster processing
- **Parallel processing** possible for multiple videos

## License

[Your License Here]

## Contributing

[Contributing Guidelines Here]

## Acknowledgments

- OpenCV project for computer vision tools
- Rust community for excellent crates (opencv, clap, serde, tauri)
