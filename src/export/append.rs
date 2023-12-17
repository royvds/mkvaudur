use std::{ffi::OsString, fs::File, io::Write, path::PathBuf, process::Command, vec};

use serde_json::Value;
use tempfile::{tempdir, TempDir};

use super::output::{
    create_track_filename, create_track_filepath, create_track_filestem, get_codec_args,
    get_map_args,
};
use crate::mediainfo::get_audio_ext;

fn get_track_samplerate(input_file: &PathBuf, track: &Value) -> i64 {
    let output = Command::new("ffprobe")
        .args(vec!["-v", "error", "-select_streams"])
        .arg(
            "a:".to_owned()
                + &(track["@typeorder"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    - 1)
                .to_string(),
        )
        .args(vec![
            "-show_entries",
            "stream=sample_rate",
            "-of",
            "csv=p=0",
        ])
        .arg(input_file)
        .output()
        .expect("Error: Unable to determine track sample rate");

    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

fn get_track_channel_layout(input_file: &PathBuf, track: &Value) -> String {
    let output = Command::new("ffprobe")
        .args(vec!["-v", "error", "-select_streams"])
        .arg(
            "a:".to_owned()
                + &(track["@typeorder"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    - 1)
                .to_string(),
        )
        .args(vec![
            "-show_entries",
            "stream=channel_layout",
            "-of",
            "csv=p=0",
        ])
        .arg(input_file)
        .output()
        .expect("Error: Unable to determine track sample rate");

    String::from_utf8(output.stdout)
        .unwrap()
        .split('(')
        .take(1)
        .collect::<Vec<_>>()[0]
        .to_owned()
}

fn generate_silence(
    input_file: &PathBuf,
    track: &Value,
    silence_duration: f64,
    tmp_dir: &TempDir,
) -> PathBuf {
    let mut silence_file = tmp_dir
        .path()
        .join(create_track_filestem(input_file, track))
        .as_os_str()
        .to_os_string();
    silence_file.push(".silence");
    silence_file.push(get_audio_ext(track));

    Command::new("ffmpeg")
        .args(vec!["-f", "lavfi", "-i"])
        .arg(format!(
            "anullsrc=channel_layout={}:sample_rate={}",
            get_track_channel_layout(input_file, track),
            get_track_samplerate(input_file, track)
        ))
        .arg("-t")
        .arg(silence_duration.to_string())
        .arg(&silence_file)
        .output()
        .expect("Concatenation of track with silence failed");

    silence_file.into()
}

fn concat_files(files: Vec<&PathBuf>, tmp_dir: &TempDir, output_file: OsString) {
    let concat_file_path = tmp_dir.path().join("concat.txt");
    let mut concat_file = File::create(&concat_file_path).expect("Unable to create concat file");

    for f in files {
        writeln!(
            concat_file,
            "file '{}'",
            f.canonicalize().unwrap().to_str().unwrap()
        )
        .expect("Failed to write content to concat file");
    }

    Command::new("ffmpeg")
        .args(vec!["-y", "-f", "concat", "-safe", "0", "-i"])
        .arg(concat_file_path)
        .args(vec!["-c", "copy"])
        .arg(output_file)
        .output()
        .expect("Failed to concatenate audio file with silence");
}

fn tmp_export_track(input_file: &PathBuf, track: &Value, tmp_dir: &TempDir) -> PathBuf {
    let output_filepath = tmp_dir
        .path()
        .join(create_track_filename(input_file, track));

    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_file)
        .args(get_map_args(track))
        .args(vec!["-map_chapters", "-1"])
        .args(get_codec_args(track))
        .arg(&output_filepath)
        .output()
        .expect("Concatenation of track with silence failed");

    output_filepath
}

pub fn append_silence(
    input_file: &PathBuf,
    track: &Value,
    silence_duration: f64,
    output_dir: &Option<OsString>,
) {
    let tmp_dir = tempdir().expect("Unable to create temporary directory");
    match track["Compression_Mode"].as_str().unwrap() {
        "Lossy" => {
            let concat_file = tmp_export_track(input_file, track, &tmp_dir);
            let silence_file = generate_silence(input_file, track, silence_duration, &tmp_dir);

            concat_files(
                vec![&concat_file, &silence_file],
                &tmp_dir,
                create_track_filepath(input_file, track, output_dir),
            );
        }
        _ => {
            // Could use the concat method here too, but this is
            // faster, more accurate for lossy encoded tracks, and
            // requires less IO usage
            Command::new("ffmpeg")
                .arg("-y")
                .arg("-i")
                .arg(input_file)
                .args(get_map_args(track))
                .args(vec!["-map_chapters", "-1"])
                .arg("-af")
                .arg(format!("apad=pad_dur={}", silence_duration))
                .args(get_codec_args(track))
                .arg(create_track_filepath(input_file, track, output_dir))
                .output()
                .expect("Concatenation of track with silence failed");
        }
    }
}
