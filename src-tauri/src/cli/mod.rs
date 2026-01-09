//! CLI interface module using clap

use clap::Parser;
use std::path::PathBuf;

/// Scene Detector - Detect scene changes in video files using OpenCV
#[derive(Parser, Debug)]
#[command(name = "video_toolset")]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the video file to process
    #[arg(short, long)]
    pub video: PathBuf,

    /// Path to the YAML configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Output directory (default: ./Output)
    #[arg(short, long, default_value = "./Output")]
    pub output: PathBuf,

    /// Show verbose progress output
    #[arg(long, default_value = "false")]
    pub verbose: bool,
}

impl CliArgs {
    /// Parse CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Check if CLI arguments were provided (for mode detection)
    pub fn has_args() -> bool {
        std::env::args().len() > 1
    }
}

/// Run the CLI application
pub fn run_cli(args: CliArgs) -> anyhow::Result<()> {
    use crate::core::{Config, OutputManager, SceneDetector};

    println!("Scene Detector v{}", env!("CARGO_PKG_VERSION"));
    println!("================================");

    // Load configuration
    println!("Loading config from: {:?}", args.config);
    let config = Config::from_file(&args.config)?;
    println!("Profile: {}", config.profile_name);
    println!("Threshold: {}", config.threshold);
    println!(
        "ROI: {}x{} at ({}, {})",
        config.roi.width, config.roi.height, config.roi.x, config.roi.y
    );

    // Create scene detector
    let video_path = args.video.to_string_lossy().to_string();
    let detector = SceneDetector::new(config.clone(), video_path.clone());

    // Get video info
    println!("\nAnalyzing video: {:?}", args.video);
    let video_info = detector.get_video_info()?;
    println!(
        "Video: {}x{} @ {:.2} fps, {} frames ({:.2}s)",
        video_info.width,
        video_info.height,
        video_info.fps,
        video_info.frame_count,
        video_info.duration_sec()
    );

    // Process video
    println!("\nDetecting scenes...");
    let progress_callback = if args.verbose {
        Some(Box::new(move |current: u64, total: u64| {
            let percent = (current as f64 / total as f64) * 100.0;
            print!("\rProgress: {:.1}% ({}/{})", percent, current, total);
        }) as Box<dyn Fn(u64, u64) + Send>)
    } else {
        None
    };

    let scenes = detector.detect_scenes(progress_callback)?;

    if args.verbose {
        println!(); // New line after progress
    }

    println!("\nFound {} scene changes:", scenes.len());
    for (i, scene) in scenes.iter().enumerate() {
        let hours = (scene.timestamp_sec / 3600.0) as u32;
        let minutes = ((scene.timestamp_sec % 3600.0) / 60.0) as u32;
        let seconds = scene.timestamp_sec % 60.0;
        println!(
            "  {}. Frame {} - {:02}:{:02}:{:05.2} (diff: {:.2})",
            i + 1,
            scene.frame_number,
            hours,
            minutes,
            seconds,
            scene.diff_value
        );
    }

    // Write output
    let output_manager = OutputManager::new(&args.output);
    let output_path = output_manager.write_metadata(&video_path, &video_info, &config, &scenes)?;
    println!("\nMetadata written to: {:?}", output_path);

    Ok(())
}
