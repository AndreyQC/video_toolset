//! Video processor module for scene detection using OpenCV
//!
//! Algorithm: ROI extraction -> Grayscale -> GaussianBlur -> absdiff -> mean calculation

use crate::core::config::{Config, ConfigError, Roi};
use opencv::{
    core::{self, Mat, Rect, Scalar, Size},
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureTraitConst},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("OpenCV error: {0}")]
    OpenCvError(#[from] opencv::Error),

    #[error("Failed to open video file: {0}")]
    VideoOpenError(String),

    #[error("Failed to read frame at position {0}")]
    FrameReadError(u64),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Video has no frames")]
    EmptyVideo,
}

/// Detected scene change information
#[derive(Debug, Clone)]
pub struct SceneChange {
    /// Frame number where scene change was detected
    pub frame_number: u64,
    /// Timestamp in seconds
    pub timestamp_sec: f64,
    /// Difference value that triggered detection
    pub diff_value: f64,
}

/// Progress callback type for reporting processing status
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send>;

/// Scene detector using OpenCV for video analysis
pub struct SceneDetector {
    config: Config,
    video_path: String,
}

impl SceneDetector {
    /// Create a new SceneDetector with the given configuration
    pub fn new(config: Config, video_path: String) -> Self {
        Self { config, video_path }
    }

    /// Get video metadata (width, height, fps, frame count)
    pub fn get_video_info(&self) -> Result<VideoInfo, ProcessorError> {
        let cap = VideoCapture::from_file(&self.video_path, videoio::CAP_ANY)?;

        if !cap.is_opened()? {
            return Err(ProcessorError::VideoOpenError(self.video_path.clone()));
        }

        let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as u32;
        let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as u32;
        let fps = cap.get(videoio::CAP_PROP_FPS)?;
        let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT)? as u64;

        Ok(VideoInfo {
            width,
            height,
            fps,
            frame_count,
        })
    }

    /// Process video and detect scene changes
    pub fn detect_scenes(
        &self,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<Vec<SceneChange>, ProcessorError> {
        let mut cap = VideoCapture::from_file(&self.video_path, videoio::CAP_ANY)?;

        if !cap.is_opened()? {
            return Err(ProcessorError::VideoOpenError(self.video_path.clone()));
        }

        let video_info = self.get_video_info()?;

        // Validate ROI against video dimensions
        self.config
            .roi
            .validate(video_info.width, video_info.height)?;

        if video_info.frame_count == 0 {
            return Err(ProcessorError::EmptyVideo);
        }

        let mut scenes: Vec<SceneChange> = Vec::new();
        let mut prev_gray: Option<Mat> = None;
        let mut frame_number: u64 = 0;
        let mut last_scene_frame: u64 = 0;

        let roi_rect = self.roi_to_rect(&self.config.roi);
        let blur_size = Size::new(
            self.config.blur_size as i32,
            self.config.blur_size as i32,
        );

        loop {
            let mut frame = Mat::default();

            if !cap.read(&mut frame)? || frame.empty() {
                break;
            }

            // Extract ROI
            let roi_frame = Mat::roi(&frame, roi_rect)?;

            // Convert to grayscale
            let mut gray = Mat::default();
            imgproc::cvt_color(&roi_frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

            // Apply Gaussian blur
            let mut blurred = Mat::default();
            imgproc::gaussian_blur(
                &gray,
                &mut blurred,
                blur_size,
                0.0,
                0.0,
                core::BORDER_DEFAULT,
            )?;

            // Compare with previous frame
            if let Some(ref prev) = prev_gray {
                let mut diff = Mat::default();
                core::absdiff(&blurred, prev, &mut diff)?;

                // Calculate mean difference
                let mean = core::mean(&diff, &core::no_array())?;
                let diff_value = mean[0];

                // Check if scene change detected
                let frames_since_last = frame_number - last_scene_frame;
                if diff_value > self.config.threshold
                    && frames_since_last >= self.config.min_scene_len as u64
                {
                    let timestamp_sec = frame_number as f64 / video_info.fps;
                    scenes.push(SceneChange {
                        frame_number,
                        timestamp_sec,
                        diff_value,
                    });
                    last_scene_frame = frame_number;
                }
            }

            prev_gray = Some(blurred);
            frame_number += 1;

            // Report progress
            if let Some(ref callback) = progress_callback {
                callback(frame_number, video_info.frame_count);
            }
        }

        Ok(scenes)
    }

    /// Convert ROI config to OpenCV Rect
    fn roi_to_rect(&self, roi: &Roi) -> Rect {
        Rect::new(
            roi.x as i32,
            roi.y as i32,
            roi.width as i32,
            roi.height as i32,
        )
    }
}

/// Video metadata information
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub frame_count: u64,
}

impl VideoInfo {
    /// Get total duration in seconds
    pub fn duration_sec(&self) -> f64 {
        self.frame_count as f64 / self.fps
    }
}
