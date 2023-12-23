use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser, Debug)]
#[clap(about = "This program requires mediainfo, ffmpeg, and ffprobe to be installed to PATH")]
pub struct MkvAudurArgs {
    #[command(flatten)]
    pub verbose: Verbosity<WarnLevel>,

    #[clap(short, long, default_value_t = 0.0)]
    /// Minimum duration difference, default = 0.0
    pub treshold: f64,

    #[clap(short, long)]
    /// Only select tracks with this language code
    pub language: Option<String>,

    #[clap(short, long)]
    /// Set a custom output directory
    pub output: Option<OsString>,

    #[clap(short, long)]
    /// Display/Export all audio tracks regardless of treshold (tracks not meeting treshold will not be cut)
    pub all: bool,

    #[clap(short, long)]
    /// Path to use the video track duration of other mkv file(s)
    pub reference: Option<PathBuf>,

    #[command(subcommand)]
    /// Modes: "display" to list track durations or "export" to trim and export tracks
    pub mode: OperationMode,

    /// Filepath, either a MKV file or directory with MKV files in it
    pub filepath: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum OperationMode {
    /// List all duration discrepancies
    Display,

    /// Export (trimmed) tracks
    Export,
}
