use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::TrackFilter;

use self::output::{create_track_filepath, get_codec_args, get_map_args};
use super::export::{append::append_silence, trim::trim_silence};

pub mod append;
mod output;
pub mod trim;

fn export_unchanged(input_file: &PathBuf, track: &Value, output_dir: &Option<OsString>) {
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_file)
        .args(get_map_args(track))
        .args(vec!["-map_chapters", "-1"])
        .args(get_codec_args(track))
        .arg(create_track_filepath(input_file, track, output_dir))
        .output()
        .expect("Failed to export track");
}

pub fn export(
    mkv_file: &PathBuf,
    mkv_mediainfo: &Value,
    video_duration: f64,
    track_filter: &TrackFilter,
    output_dir: &Option<OsString>,
) {
    println!(
        "Processing file: {}",
        Path::new(mkv_mediainfo["media"]["@ref"].as_str().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let tracks = mkv_mediainfo["media"]["track"].as_array().unwrap();
    for track in tracks {
        if track["@type"] != "Audio" {
            continue;
        }
        let duration_difference: f64 =
            track["Duration"].as_str().unwrap().parse::<f64>().unwrap() - video_duration;
        let track_language = track["Language"].as_str();

        if (track_filter.language.is_none()
            || (track_language.is_some()
                && track_language.unwrap() == track_filter.language.as_ref().unwrap()))
            && f64::abs(duration_difference) > track_filter.treshold
        {
            match duration_difference > 0.0 {
                true => trim_silence(mkv_file, track, video_duration, output_dir),
                false => append_silence(mkv_file, track, f64::abs(duration_difference), output_dir),
            }
        } else if track_filter.process_all {
            export_unchanged(mkv_file, track, output_dir);
        }
    }
}
