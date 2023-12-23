use serde_json::Value;

use std::{ffi::OsString, path::PathBuf, process::Command};

use super::output::{create_track_filepath, get_codec_args, get_map_args};

pub fn trim_silence(
    input_file: &PathBuf,
    track: &Value,
    new_track_duration: f64,
    output_dir: &Option<OsString>,
) {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-t")
        .arg(new_track_duration.to_string())
        .arg("-i")
        .arg(input_file)
        .args(get_map_args(track))
        .args(vec!["-map_chapters", "-1"])
        .args(get_codec_args(track))
        .arg(create_track_filepath(input_file, track, output_dir));

    log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                log::error!(
                    "Failed to trim track {} of file {}",
                    get_map_args(track)[1],
                    input_file.display()
                );
                log::trace!(
                    "FFMPEG error log: {}",
                    String::from_utf8(output.stderr).unwrap()
                );
            }
        }
        Err(e) => {
            log::debug!("{}", e);
            panic!("Error trimming track, is FFMPEG installed to path?");
        }
    }
}
