# Windows OpenCV Setup Guide

This guide explains how to set up OpenCV and LLVM for Rust development on Windows.

⚠️ **IMPORTANT**: The `opencv` crate requires LLVM/libclang to build. Follow these steps in order.

## Prerequisites

### Install LLVM

LLVM is required for building the opencv crate (via clang-sys dependency).

**Option A: Using LLVM Installer (Recommended)**

1. Download LLVM from: https://releases.llvm.org/download.html or https://github.com/llvm/llvm-project/releases
2. Download the latest version for Windows (e.g., `LLVM-17.0.6-win64.exe`)
3. Run the installer with:
   - ✅ Add LLVM to the system PATH for all users
   - ✅ Install for all users
4. Set environment variable:
   ```powershell
   [Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "Machine")
   ```

**Option B: Using vcpkg**

```powershell
.\vcpkg install llvm:x64-windows
[Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\vcpkg\installed\x64-windows\bin", "Machine")
```

## Option 1: vcpkg (Recommended)

### Install vcpkg

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install
```

### Install OpenCV

```powershell
# Install OpenCV 4 (64-bit)
.\vcpkg install opencv4:x64-windows

# Or with additional features
.\vcpkg install opencv4[contrib]:x64-windows
```

### Set Environment Variables

Add these to your system environment variables:

```powershell
# PowerShell (run as Administrator)
[Environment]::SetEnvironmentVariable("VCPKG_ROOT", "C:\vcpkg", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_LIBS", "opencv_world4", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_PATHS", "C:\vcpkg\installed\x64-windows\lib", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_INCLUDE_PATHS", "C:\vcpkg\installed\x64-windows\include", "Machine")
```

Or in Command Prompt:

```batch
setx VCPKG_ROOT "C:\vcpkg"
setx OPENCV_LINK_LIBS "opencv_world4"
setx OPENCV_LINK_PATHS "C:\vcpkg\installed\x64-windows\lib"
setx OPENCV_INCLUDE_PATHS "C:\vcpkg\installed\x64-windows\include"
```

### Add DLL to PATH

```powershell
# Add to PATH
$env:Path += ";C:\vcpkg\installed\x64-windows\bin"

# Or permanently
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\vcpkg\installed\x64-windows\bin", "Machine")
```

---

## Option 2: Pre-built Binaries

### Download OpenCV

1. Go to https://opencv.org/releases/
2. Download the Windows release (e.g., `opencv-4.9.0-windows.exe`)
3. Extract to `C:\opencv`

### Set Environment Variables

```powershell
[Environment]::SetEnvironmentVariable("OPENCV_LINK_LIBS", "opencv_world490", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_PATHS", "C:\opencv\build\x64\vc16\lib", "Machine")
[Environment]::SetEnvironmentVariable("OPENCV_INCLUDE_PATHS", "C:\opencv\build\include", "Machine")
```

### Add DLL to PATH

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\opencv\build\x64\vc16\bin", "Machine")
```

---

## Verify Installation

After setting up, restart your terminal and run:

```powershell
# Check environment variables
echo $env:OPENCV_LINK_PATHS
echo $env:OPENCV_INCLUDE_PATHS

# Build the project
cargo build
```

## Troubleshooting

### LLVM Errors

#### "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll']"
- LLVM is not installed or not in PATH
- Install LLVM using the installer (recommended) or vcpkg
- Set `LIBCLANG_PATH` environment variable:
  ```powershell
  [Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "Machine")
  ```
- Restart your terminal after setting environment variables

#### "could not execute `llvm-config`"
- LLVM is not properly installed
- Check that LLVM is in your PATH: `where clang`
- If not found, reinstall LLVM with "Add to PATH" option enabled

### OpenCV Errors

#### "opencv2/core.hpp not found"
- Verify `OPENCV_INCLUDE_PATHS` points to the correct include directory
- The path should contain `opencv2/core.hpp`

#### "cannot find -lopencv_world4"
- Verify `OPENCV_LINK_PATHS` points to the lib directory
- Check if the lib file exists (e.g., `opencv_world490.lib`)

#### "opencv_world4.dll not found" at runtime
- Add the bin directory to your PATH
- Or copy the DLL to your executable's directory

### MSVC Linker Issues
- Ensure you have Visual Studio Build Tools installed
- Use the MSVC toolchain: `rustup default stable-msvc`

### Build System Issues
- Clean build: `cargo clean && cargo build`
- Check all environment variables are set correctly
- Restart terminal after installing LLVM/OpenCV

## Build Commands

```bash
# Build CLI only (default)
cargo build --release

# Build with GUI support
cargo build --release --features gui

# Build all features
cargo build --release --all-features
```
