use serde_json::Value;

use std::{ffi::OsString, path::PathBuf, process::Command};

use super::output::{create_track_filepath, get_codec_args, get_map_args};

pub fn trim_silence(
    input_file: &PathBuf,
    track: &Value,
    new_track_duration: f64,
    output_dir: &Option<OsString>,
) {
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-t")
        .arg(new_track_duration.to_string())
        .arg("-i")
        .arg(input_file)
        .args(get_map_args(track))
        .args(vec!["-map_chapters", "-1"])
        .args(get_codec_args(track))
        .arg(create_track_filepath(input_file, track, output_dir))
        .output()
        .expect(&format!(
            "Failed trimming audio of file: {}",
            input_file.display()
        ));
}
