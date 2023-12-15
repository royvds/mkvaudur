use std::{ffi::OsString, fs::read_dir, path::PathBuf};

use args::OperationMode;
use serde_json::Value;

pub mod args;
pub mod display;
pub mod export;
pub mod mediainfo;

pub fn get_files(filepath: &PathBuf) -> Vec<PathBuf> {
    let input_files: Vec<PathBuf>;

    if filepath.is_file() && filepath.extension().unwrap() == "mkv" {
        input_files = vec![filepath.clone()]
    } else if filepath.is_dir() {
        let paths = read_dir(filepath).unwrap();
        input_files = paths
            .filter(|path| {
                path.is_ok()
                    && path.as_ref().unwrap().path().is_file()
                    && match path.as_ref().unwrap().path().extension() {
                        Some(v) => v.to_str().unwrap() == "mkv",
                        None => false,
                    }
            })
            .map(|path| path.unwrap().path())
            .collect();
    } else {
        panic!("Provided input is not a valid mkv file or directory");
    }

    if input_files.is_empty() {
        panic!("Provided directory does not contain any mkv files");
    }

    input_files
}

pub fn process_mkv_file(
    mkv_file: &PathBuf,
    mkv_mediainfo: &Value,
    ref_mediainfo: &Value,
    operation_mode: &OperationMode,
    treshold: f64,
    language: &Option<String>,
    process_all: bool,
    output_dir: &Option<OsString>,
) {
    let video_track: &Value = ref_mediainfo["media"]["track"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|t| t["@type"] == "Video")
        .collect::<Vec<&Value>>()[0];
    let video_track_duration = video_track["Duration"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    match operation_mode {
        OperationMode::Display => display::display(
            mkv_mediainfo,
            video_track_duration,
            treshold,
            language,
            process_all,
        ),
        OperationMode::Export => export::export(
            mkv_file,
            mkv_mediainfo,
            video_track_duration,
            treshold,
            language,
            process_all,
            output_dir,
        ),
    }
}
