//! Configuration module for YAML profile handling

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}

/// Region of Interest for scene detection analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roi {
    /// X coordinate (pixels from left)
    pub x: u32,
    /// Y coordinate (pixels from top)
    pub y: u32,
    /// Width of the region (pixels)
    pub width: u32,
    /// Height of the region (pixels)
    pub height: u32,
}

impl Roi {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Validate ROI against video dimensions
    pub fn validate(&self, video_width: u32, video_height: u32) -> Result<(), ConfigError> {
        if self.x + self.width > video_width {
            return Err(ConfigError::ValidationError(format!(
                "ROI exceeds video width: {} + {} > {}",
                self.x, self.width, video_width
            )));
        }
        if self.y + self.height > video_height {
            return Err(ConfigError::ValidationError(format!(
                "ROI exceeds video height: {} + {} > {}",
                self.y, self.height, video_height
            )));
        }
        if self.width == 0 || self.height == 0 {
            return Err(ConfigError::ValidationError(
                "ROI width and height must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Scene detection configuration profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Profile name for identification
    #[serde(default = "default_profile_name")]
    pub profile_name: String,

    /// Threshold for scene change detection (0.0 - 255.0)
    /// Higher values = less sensitive
    pub threshold: f64,

    /// Region of interest for analysis
    pub roi: Roi,

    /// Minimum scene length in frames
    /// Prevents rapid scene changes from being detected
    pub min_scene_len: u32,

    /// Gaussian blur kernel size (must be odd)
    #[serde(default = "default_blur_size")]
    pub blur_size: u32,
}

fn default_profile_name() -> String {
    "default".to_string()
}

fn default_blur_size() -> u32 {
    5
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.threshold < 0.0 || self.threshold > 255.0 {
            return Err(ConfigError::ValidationError(
                "Threshold must be between 0.0 and 255.0".to_string(),
            ));
        }
        if self.min_scene_len == 0 {
            return Err(ConfigError::ValidationError(
                "min_scene_len must be greater than 0".to_string(),
            ));
        }
        if self.blur_size % 2 == 0 {
            return Err(ConfigError::ValidationError(
                "blur_size must be an odd number".to_string(),
            ));
        }
        Ok(())
    }

    /// Create a default configuration
    pub fn default_config() -> Self {
        Self {
            profile_name: "default".to_string(),
            threshold: 30.0,
            roi: Roi::new(0, 0, 1920, 1080),
            min_scene_len: 15,
            blur_size: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = Config::default_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_threshold() {
        let mut config = Config::default_config();
        config.threshold = 300.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_roi_validation() {
        let roi = Roi::new(100, 100, 800, 600);
        assert!(roi.validate(1920, 1080).is_ok());
        assert!(roi.validate(500, 500).is_err());
    }
}
