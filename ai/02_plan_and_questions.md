# Plan & Questions: Rust Scene Detector

## Project Analysis Summary

Based on `01_initial_requirement.md`, this is a scene detection application for MPEG video files using OpenCV with dual interface support (GUI via Tauri, CLI via clap).

---

## Implementation Plan

### Phase 1: Project Structure Setup

1. **Create Cargo workspace or modular structure**
   - `src/core/` - video processing logic (UI-agnostic)
   - `src/cli/` - CLI interface with clap
   - `src/gui/` - Tauri integration
   - `src/main.rs` - entry point with mode switching

2. **Setup `Cargo.toml`** with dependencies:
   - `opencv` (4.x bindings)
   - `tauri`
   - `clap` (derive)
   - `serde` + `serde_yaml`
   - `anyhow` + `thiserror`

3. **Windows OpenCV configuration**
   - Document vcpkg setup or static paths
   - Configure `build.rs` if needed
   - Set environment variables (`OPENCV_LINK_PATHS`, `OPENCV_INCLUDE_PATHS`)

### Phase 2: Core Module Implementation

4. **`src/core/config.rs`**
   - YAML profile structure: `threshold`, `roi`, `min_scene_len`
   - Serde deserialization
   - Config validation

5. **`src/core/processor.rs`**
   - `VideoCapture` for frame reading
   - ROI extraction
   - Algorithm: Gray → GaussianBlur → absdiff → mean calculation
   - Scene change detection with threshold comparison

6. **`src/core/output.rs`**
   - Create `./Output/` directory
   - Generate `metadata.json` with timestamps and profile metadata

### Phase 3: Interface Implementation

7. **CLI (`src/cli/mod.rs`)**
   - Accept video path and config path arguments
   - Progress reporting to stdout

8. **GUI (`src/gui/mod.rs`)**
   - Tauri backend commands
   - Visual ROI editor
   - Parameter adjustment UI

### Phase 4: Integration & Testing

9. **`src/main.rs`** - Mode switching logic
10. **Testing with sample MPEG files**
11. **Windows build verification**

---

## Open Questions

### Architecture Questions

1. **Workspace vs Single Crate?**
   - Should we use Cargo workspace (separate crates for core, cli, gui)?
   - Or single crate with feature flags (`--features gui`, `--features cli`)?

2. **Frontend Technology for Tauri?**
   - Requirements say "Frontend на выбор" - which one?
   - Options: Vanilla JS, React, Vue, Svelte, Solid?

3. **Mode Selection Mechanism?**
   - CLI flag (`--gui` / `--cli`)?
   - Separate binaries?
   - Auto-detect (GUI if no args, CLI if args present)?

### Technical Questions

4. **OpenCV Installation Method for Windows?**
   - vcpkg (recommended)?
   - Pre-built binaries with manual paths?
   - Chocolatey?

5. **Async Processing?**
   - Should video processing be async?
   - How to handle progress reporting to GUI?

6. **ROI Configuration Format?**
   - Absolute pixels or relative (percentage)?
   - What if ROI exceeds video dimensions?

### Functional Questions

7. **Scene Detection Output?**
   - Only timestamps?
   - Frame numbers as well?
   - Thumbnail generation for each scene?

8. **Multiple Profiles Support?**
   - Single profile per run?
   - Or batch processing with multiple profiles?

9. **Video Format Support?**
   - MPEG only (as stated)?
   - Or any format OpenCV supports?

10. **Error Recovery?**
    - Skip corrupted frames?
    - Abort on first error?

---

## Recommended Decisions (Pending Confirmation)

| Question | Recommended Choice | Rationale |
|----------|-------------------|-----------|
| Structure | Single crate + features | Simpler for this project size |
| Frontend | Svelte | Lightweight, Tauri-friendly |
| Mode selection | CLI args presence | Intuitive UX |
| OpenCV setup | vcpkg | Best Windows support |
| ROI format | Absolute pixels | Predictable behavior |
| Output | Timestamps + frame numbers | More useful for editing |

---

## Next Steps

1. Confirm answers to open questions
2. Setup development environment (Rust, OpenCV, Tauri CLI)
3. Begin Phase 1 implementation

---

## Dependencies Version Targets

```toml
# Preliminary - to be finalized
opencv = "0.92"          # OpenCV 4.x bindings
tauri = "2.0"            # Latest Tauri
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
anyhow = "1.0"
thiserror = "2.0"
```
