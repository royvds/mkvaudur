use std::{
    ffi::OsString,
    fmt,
    fs::read_dir,
    path::{Path, PathBuf},
};

use args::OperationMode;
use serde_json::Value;

pub mod args;
pub mod display;
pub mod export;
pub mod mediainfo;

pub struct TrackFilter {
    pub treshold: f64,
    pub language: Option<String>,
    pub process_all: bool,
}

#[derive(Debug, Clone)]
pub struct NoMkvFound;

impl fmt::Display for NoMkvFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "filepath does not contain any mkv file")
    }
}

pub fn get_files(filepath: &Path) -> Result<Vec<PathBuf>, NoMkvFound> {
    let input_files: Vec<PathBuf>;

    if filepath.is_file() && filepath.extension().unwrap() == "mkv" {
        input_files = vec![filepath.to_owned()]
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
        return Err(NoMkvFound);
    }

    if input_files.is_empty() {
        return Err(NoMkvFound);
    }

    Ok(input_files)
}

pub fn process_mkv_file(
    mkv_file: &PathBuf,
    mkv_mediainfo: &Value,
    ref_mediainfo: &Value,
    operation_mode: &OperationMode,
    track_filter: &TrackFilter,
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
        OperationMode::Display => {
            display::display(mkv_mediainfo, video_track_duration, track_filter)
        }
        OperationMode::Export => export::export(
            mkv_file,
            mkv_mediainfo,
            video_track_duration,
            track_filter,
            output_dir,
        ),
    }
}
