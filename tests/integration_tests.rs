use std::{
    ffi::OsString,
    fs::{read_dir, remove_dir_all},
    path::PathBuf,
};

use mkvaudur::{args::OperationMode, mediainfo::get_mediainfo, process_mkv_file, TrackFilter};

fn get_audio_files(dir: &str) -> Vec<PathBuf> {
    let paths = read_dir(dir).unwrap();
    paths
        .filter(|path| {
            path.is_ok()
                && path.as_ref().unwrap().path().is_file()
                && match path.as_ref().unwrap().path().extension() {
                    Some(v) => v.to_str().unwrap() == "flac" || v.to_str().unwrap() == "opus",
                    None => false,
                }
        })
        .map(|path| path.unwrap().path())
        .collect()
}

fn get_audio_file_duration(audio_file: &PathBuf) -> f64 {
    let audio_mediainfo = get_mediainfo(&audio_file);
    audio_mediainfo["media"]["track"].as_array().unwrap()[0]["Duration"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

static TRACK_FILTER: TrackFilter = TrackFilter {
    treshold: 0.0,
    language: None,
    process_all: false,
};

#[test]
fn trim() {
    let mkv_file = PathBuf::from("./tests/test_video_2s.mkv");
    let mkv_mediainfo = get_mediainfo(&mkv_file);
    process_mkv_file(
        &mkv_file,
        &mkv_mediainfo,
        &mkv_mediainfo,
        &OperationMode::Export,
        &TRACK_FILTER,
        &Some(OsString::from("./tests/trim")),
    );

    let audio_files = get_audio_files("./tests/trim");
    for audio_file in audio_files {
        assert!((1.9..2.1).contains(&get_audio_file_duration(&audio_file)));
    }

    remove_dir_all("./tests/trim").err();
}

#[test]
fn append() {
    let mkv_file = PathBuf::from("./tests/test_video_2s.mkv");
    let ref_file = PathBuf::from("./tests/ref_video_8s.mkv");
    let mkv_mediainfo = get_mediainfo(&mkv_file);
    let ref_mediainfo = get_mediainfo(&ref_file);
    process_mkv_file(
        &mkv_file,
        &mkv_mediainfo,
        &ref_mediainfo,
        &OperationMode::Export,
        &TRACK_FILTER,
        &Some(OsString::from("./tests/append")),
    );

    let audio_files = get_audio_files("./tests");
    for audio_file in audio_files {
        println!(
            "file: {}, duration: {}",
            &audio_file.display(),
            get_audio_file_duration(&audio_file)
        );
        assert!((7.9..8.1).contains(&get_audio_file_duration(&audio_file)));
    }

    remove_dir_all("./tests/append").err();
}
