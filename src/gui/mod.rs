//! GUI Module - Tauri integration for frontend
//!
//! This module provides the backend API that the Svelte frontend uses
//! to interact with the video processing core.

use crate::core::config::Config;
use crate::core::processor::SceneProcessor;
use crate::core::output::OutputManager;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

/// Global state for the GUI application
pub type GuiState = Mutex<GuiAppState>;

/// Application state shared between frontend and backend
#[derive(Debug, Clone)]
pub struct GuiAppState {
    pub video_path: Option<String>,
    pub config: Config,
    pub scenes_detected: Vec<SceneInfo>,
}

/// Scene information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneInfo {
    pub scene_id: u32,
    pub timestamp: String,
    pub frame_number: u64,
}

/// ROI data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoiData {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Initialize the GUI application state
pub fn init_gui_state() -> GuiState {
    Mutex::new(GuiAppState {
        video_path: None,
        config: Config::default(),
        scenes_detected: Vec::new(),
    })
}

/// Load configuration from YAML file
#[tauri::command]
pub async fn load_config(
    config_path: String,
    state: State<'_, GuiState>,
) -> Result<Config, String> {
    let config = Config::load_from_file(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config = config.clone();

    Ok(config)
}

/// Save configuration to YAML file
#[tauri::command]
pub async fn save_config(
    config_path: String,
    config: Config,
    state: State<'_, GuiState>,
) -> Result<(), String> {
    config.save_to_file(&config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config = config;

    Ok(())
}

/// Get current configuration
#[tauri::command]
pub async fn get_config(state: State<'_, GuiState>) -> Result<Config, String> {
    let state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(state_guard.config.clone())
}

/// Update ROI configuration
#[tauri::command]
pub async fn update_roi(
    roi: RoiData,
    state: State<'_, GuiState>,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config.roi.x = roi.x as i32;
    state_guard.config.roi.y = roi.y as i32;
    state_guard.config.roi.width = roi.width as u32;
    state_guard.config.roi.height = roi.height as u32;
    Ok(())
}

/// Get ROI data
#[tauri::command]
pub async fn get_roi(state: State<'_, GuiState>) -> Result<RoiData, String> {
    let state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(RoiData {
        x: state_guard.config.roi.x as u32,
        y: state_guard.config.roi.y as u32,
        width: state_guard.config.roi.width,
        height: state_guard.config.roi.height,
    })
}

/// Update threshold
#[tauri::command]
pub async fn update_threshold(
    threshold: f64,
    state: State<'_, GuiState>,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config.threshold = threshold;
    Ok(())
}

/// Update minimum scene length
#[tauri::command]
pub async fn update_min_scene_len(
    min_scene_len: u32,
    state: State<'_, GuiState>,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config.min_scene_len = min_scene_len;
    Ok(())
}

/// Update blur size
#[tauri::command]
pub async fn update_blur_size(
    blur_size: u32,
    state: State<'_, GuiState>,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.config.blur_size = blur_size as i32;
    Ok(())
}

/// Run scene detection
#[tauri::command]
pub async fn detect_scenes(
    video_path: String,
    output_dir: Option<String>,
    state: State<'_, GuiState>,
) -> Result<Vec<SceneInfo>, String> {
    let state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    let config = state_guard.config.clone();
    drop(state_guard);

    let output_dir = output_dir.unwrap_or_else(|| "./Output".to_string());

    // Run scene detection
    let processor = SceneProcessor::new(config.clone());
    let scenes = processor
        .detect_scenes(&video_path)
        .map_err(|e| format!("Detection failed: {}", e))?;

    let output_manager = OutputManager::new(&output_dir);
    output_manager
        .save_metadata(&video_path, &scenes, &config)
        .map_err(|e| format!("Failed to save metadata: {}", e))?;

    // Update state with results
    let mut state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    state_guard.video_path = Some(video_path);
    state_guard.scenes_detected = scenes
        .iter()
        .enumerate()
        .map(|(i, scene)| SceneInfo {
            scene_id: i as u32 + 1,
            timestamp: format_time(scene.timestamp),
            frame_number: scene.frame_number,
        })
        .collect();

    Ok(state_guard.scenes_detected.clone())
}

/// Get detected scenes
#[tauri::command]
pub async fn get_scenes(state: State<'_, GuiState>) -> Result<Vec<SceneInfo>, String> {
    let state_guard = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(state_guard.scenes_detected.clone())
}

/// Get video metadata
#[tauri::command]
pub async fn get_video_metadata(video_path: String) -> Result<VideoMetadata, String> {
    use opencv::videoio::{VideoCapture, CAP_ANY};

    let mut cap = VideoCapture::from_file(&video_path, CAP_ANY)
        .map_err(|e| format!("Failed to open video: {}", e))?;

    let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)
        .map_err(|e| format!("Failed to get width: {}", e))?;
    let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)
        .map_err(|e| format!("Failed to get height: {}", e))?;
    let fps = cap.get(opencv::videoio::CAP_PROP_FPS)
        .map_err(|e| format!("Failed to get FPS: {}", e))?;
    let frame_count = cap.get(opencv::videoio::CAP_PROP_FRAME_COUNT)
        .map_err(|e| format!("Failed to get frame count: {}", e))?;

    Ok(VideoMetadata {
        width: width as u32,
        height: height as u32,
        fps: fps as f64,
        frame_count: frame_count as u64,
        duration_seconds: if fps > 0.0 {
            frame_count as f64 / fps
        } else {
            0.0
        },
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub frame_count: u64,
    pub duration_seconds: f64,
}

/// Format timestamp as HH:MM:SS.mmm
fn format_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as u64;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u64;
    let secs = (seconds % 60.0).floor() as u64;
    let millis = ((seconds % 1.0) * 1000.0).floor() as u64;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
}

/// Register all GUI commands with Tauri
pub fn register_commands(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage(init_gui_state());

    // Register commands
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_config,
            save_config,
            get_config,
            update_roi,
            get_roi,
            update_threshold,
            update_min_scene_len,
            update_blur_size,
            detect_scenes,
            get_scenes,
            get_video_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
