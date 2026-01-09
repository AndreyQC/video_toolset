//! Core module - UI-agnostic video processing logic
//!
//! This module contains all the business logic for scene detection
//! and is independent of any user interface (CLI or GUI).

pub mod config;
pub mod output;
pub mod processor;

pub use config::Config;
pub use output::OutputManager;
pub use processor::SceneDetector;
