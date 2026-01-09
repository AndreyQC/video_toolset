//! Output module for generating results and metadata

use crate::core::config::Config;
use crate::core::processor::{SceneChange, VideoInfo};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OutputError {
    #[error("Failed to create output directory: {0}")]
    DirectoryCreateError(#[from] std::io::Error),

    #[error("Failed to serialize JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Output metadata structure for JSON export
#[derive(Debug, Serialize)]
pub struct OutputMetadata {
    /// Video file path that was processed
    pub video_path: String,

    /// Video information
    pub video_info: VideoInfoOutput,

    /// Configuration profile used
    pub profile: ProfileOutput,

    /// Detected scene changes
    pub scenes: Vec<SceneOutput>,

    /// Processing timestamp
    pub processed_at: String,
}

#[derive(Debug, Serialize)]
pub struct VideoInfoOutput {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub frame_count: u64,
    pub duration_sec: f64,
}

#[derive(Debug, Serialize)]
pub struct ProfileOutput {
    pub name: String,
    pub threshold: f64,
    pub roi: RoiOutput,
    pub min_scene_len: u32,
    pub blur_size: u32,
}

#[derive(Debug, Serialize)]
pub struct RoiOutput {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
pub struct SceneOutput {
    pub frame_number: u64,
    pub timestamp_sec: f64,
    pub timestamp_formatted: String,
    pub diff_value: f64,
}

/// Manager for output file generation
pub struct OutputManager {
    output_dir: PathBuf,
}

impl OutputManager {
    /// Create a new OutputManager with the specified output directory
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    /// Create with default ./Output directory
    pub fn default_output() -> Self {
        Self::new("./Output")
    }

    /// Ensure output directory exists
    pub fn ensure_directory(&self) -> Result<(), OutputError> {
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)?;
        }
        Ok(())
    }

    /// Generate metadata.json file
    pub fn write_metadata(
        &self,
        video_path: &str,
        video_info: &VideoInfo,
        config: &Config,
        scenes: &[SceneChange],
    ) -> Result<PathBuf, OutputError> {
        self.ensure_directory()?;

        let metadata = OutputMetadata {
            video_path: video_path.to_string(),
            video_info: VideoInfoOutput {
                width: video_info.width,
                height: video_info.height,
                fps: video_info.fps,
                frame_count: video_info.frame_count,
                duration_sec: video_info.duration_sec(),
            },
            profile: ProfileOutput {
                name: config.profile_name.clone(),
                threshold: config.threshold,
                roi: RoiOutput {
                    x: config.roi.x,
                    y: config.roi.y,
                    width: config.roi.width,
                    height: config.roi.height,
                },
                min_scene_len: config.min_scene_len,
                blur_size: config.blur_size,
            },
            scenes: scenes
                .iter()
                .map(|s| SceneOutput {
                    frame_number: s.frame_number,
                    timestamp_sec: s.timestamp_sec,
                    timestamp_formatted: format_timestamp(s.timestamp_sec),
                    diff_value: s.diff_value,
                })
                .collect(),
            processed_at: chrono_now(),
        };

        let json = serde_json::to_string_pretty(&metadata)?;
        let output_path = self.output_dir.join("metadata.json");
        fs::write(&output_path, json)?;

        Ok(output_path)
    }

    /// Get the output directory path
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
}

/// Format seconds to HH:MM:SS.mmm
fn format_timestamp(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    let millis = ((seconds - total_seconds as f64) * 1000.0) as u32;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
}

/// Get current timestamp in ISO format (simple implementation without chrono crate)
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();
    // Simple UTC timestamp approximation
    format!("{}", secs)
}
