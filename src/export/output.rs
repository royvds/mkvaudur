use std::{
    ffi::OsString,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::mediainfo::get_audio_ext;

/// Includes directory, filename, and extension
pub fn create_track_filepath(
    input_file: &Path,
    track: &Value,
    custom_directory: &Option<OsString>,
) -> OsString {
    match custom_directory.as_ref() {
        Some(custom_dir) => {
            match create_dir_all(custom_dir) {
                Ok(_) => {}
                Err(_) => panic!("Unable to create custom output directory"),
            }
            PathBuf::from(custom_dir)
                .join(create_track_filename(input_file, track))
                .as_os_str()
                .to_owned()
        }
        None => input_file
            .parent()
            .unwrap()
            .join(create_track_filename(input_file, track))
            .as_os_str()
            .to_owned(),
    }
}

/// Includes filename and extension. Excludes directory.
pub fn create_track_filename(input_file: &Path, track: &Value) -> OsString {
    let mut output_filename = create_track_filestem(input_file, track);
    output_filename.push(get_audio_ext(track));
    output_filename
}

/// Includes filename. Excludes directory and extension
pub fn create_track_filestem(input_file: &Path, track: &Value) -> OsString {
    let mut output_filename = input_file.file_stem().unwrap().to_owned();
    output_filename.push("_Audio");
    output_filename.push(format!(
        "{:02}",
        track["@typeorder"]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap(),
    ));
    output_filename.push(".");
    match track["Language"].as_str() {
        Some(lang) => output_filename.push(lang.to_uppercase()),
        None => output_filename.push("UND"),
    }
    output_filename
}

pub fn get_map_args(track: &Value) -> Vec<String> {
    vec![
        "-map".to_owned(),
        format!(
            "0:a:{}",
            track["@typeorder"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap()
                - 1
        ),
    ]
}

pub fn get_codec_args(track: &Value) -> Vec<String> {
    match track["Compression_Mode"].as_str().unwrap() {
        "Lossy" => vec!["-c:a".to_string(), "copy".to_string()],
        "Lossless" => vec![],
        _ => panic!("Track contains invalid Compression Mode"),
    }
}
