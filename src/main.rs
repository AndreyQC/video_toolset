//! Video Toolset - Scene Detection Application
//!
//! Supports two modes:
//! - CLI: Run with arguments (--video, --config)
//! - GUI: Run without arguments (launches Tauri window)

mod core;

#[cfg(feature = "cli")]
mod cli;

fn main() -> anyhow::Result<()> {
    // Mode detection: CLI if arguments present, GUI otherwise
    #[cfg(feature = "cli")]
    {
        if cli::CliArgs::has_args() {
            let args = cli::CliArgs::parse_args();
            return cli::run_cli(args);
        }
    }

    // GUI mode (when no CLI args or cli feature disabled)
    #[cfg(feature = "gui")]
    {
        run_gui()?;
        return Ok(());
    }

    // Fallback: show usage if no mode available
    #[cfg(not(feature = "gui"))]
    {
        println!("Video Toolset - Scene Detector");
        println!();
        println!("Usage: video_toolset --video <FILE> --config <FILE>");
        println!();
        println!("Options:");
        println!("  -v, --video <FILE>   Path to the video file");
        println!("  -c, --config <FILE>  Path to the YAML config file");
        println!("  -o, --output <DIR>   Output directory (default: ./Output)");
        println!("  --verbose            Show progress output");
        println!("  -h, --help           Print help");
        println!("  -V, --version        Print version");
    }

    Ok(())
}

#[cfg(feature = "gui")]
fn run_gui() -> anyhow::Result<()> {
    // Tauri application setup
    // TODO: Implement Tauri GUI when gui feature is enabled
    println!("GUI mode not yet implemented");
    println!("Please use CLI mode: video_toolset --video <FILE> --config <FILE>");
    Ok(())
}
