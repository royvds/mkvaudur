use std::{ffi::OsString, fs::File, io::Write, path::PathBuf, process::Command, vec};

use serde_json::Value;
use tempfile::{tempdir, TempDir};

use super::output::{
    create_track_filename, create_track_filepath, create_track_filestem, get_codec_args,
    get_map_args,
};
use crate::mediainfo::get_audio_ext;

/// Returns: (sample rate, channel layout)
fn get_track_samplerate_channellayout(input_file: &PathBuf, track: &Value) -> (i64, String) {
    let mut cmd = Command::new("ffprobe");
    cmd.args(vec!["-v", "error", "-select_streams"])
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
            "stream=sample_rate,channel_layout",
            "-of",
            "csv=p=0",
        ])
        .arg(input_file);

    log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                log::warn!(
                    "Failed to detect sample rate and channel layout of track {} of file {}\nDefaulting to 48Khz stereo; \u{001b}[31mif this is incorrect, appending silence will fail or the resulting audio file will be malformed!\u{001b}[00m",
                    get_map_args(track)[1],
                    input_file.display()
                );
                log::trace!(
                    "FFPROBE error log: {}",
                    String::from_utf8(output.stderr).unwrap()
                );
                (48000, "stereo".to_owned())
            } else {
                let output_string: String =
                    String::from_utf8(output.stdout).unwrap().trim().to_owned();
                let output_values: Vec<&str> = output_string.split(',').collect();
                (
                    output_values[0].parse().unwrap(),
                    output_values[1].to_owned(),
                )
            }
        }
        Err(e) => {
            log::debug!("{}", e);
            panic!("Error retrieving track channel layout, is FFPROBE installed to path?");
        }
    }
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

    let (sample_rate, channel_layout) = get_track_samplerate_channellayout(input_file, track);

    let mut cmd = Command::new("ffmpeg");
    cmd.args(vec!["-f", "lavfi", "-i"])
        .arg(format!(
            "anullsrc=sample_rate={}:channel_layout={}",
            sample_rate, channel_layout
        ))
        .arg("-t")
        .arg(silence_duration.to_string())
        .arg(&silence_file);

    log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                log::error!(
                    "Failed generate silence file for track {} of file {}",
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
            panic!("Error generating silence, is FFMPEG installed to path?");
        }
    }

    silence_file.into()
}

fn concat_files(files: Vec<&PathBuf>, tmp_dir: &TempDir, output_file: OsString) {
    let concat_file_path = tmp_dir.path().join("concat.txt");
    let mut concat_file = File::create(&concat_file_path).expect("Unable to create concat file");

    for f in &files {
        writeln!(
            concat_file,
            "file '{}'",
            f.canonicalize().unwrap().to_str().unwrap()
        )
        .expect("Failed to write content to concat file");
    }

    let mut cmd = Command::new("ffmpeg");
    cmd.args(vec!["-y", "-f", "concat", "-safe", "0", "-i"])
        .arg(concat_file_path)
        .args(vec!["-c", "copy"])
        .arg(output_file);

    log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                log::error!("Failed to concat files: {:?}", &files);
                log::trace!(
                    "FFMPEG error log: {}",
                    String::from_utf8(output.stderr).unwrap()
                );
            }
        }
        Err(e) => {
            log::debug!("{}", e);
            panic!("Error concatenating files, is FFMPEG installed to path?");
        }
    }
}

fn tmp_export_track(input_file: &PathBuf, track: &Value, tmp_dir: &TempDir) -> PathBuf {
    let output_filepath = tmp_dir
        .path()
        .join(create_track_filename(input_file, track));

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-i")
        .arg(input_file)
        .args(get_map_args(track))
        .args(vec!["-map_chapters", "-1"])
        .args(get_codec_args(track))
        .arg(&output_filepath);

    log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                log::error!(
                    "Failed to export track to temporary file: track {} from file {}",
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
            panic!("Failed to export track to temporary file, is FFMPEG installed to path?");
        }
    }

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
            let mut cmd = Command::new("ffmpeg");
            cmd.arg("-y")
                .arg("-i")
                .arg(input_file)
                .args(get_map_args(track))
                .args(vec!["-map_chapters", "-1"])
                .arg("-af")
                .arg(format!("apad=pad_dur={}", silence_duration))
                .args(get_codec_args(track))
                .arg(create_track_filepath(input_file, track, output_dir));

            log::info!("Executing: {:?}", format!("{:?}", cmd).replace('\"', ""));
            match cmd.output() {
                Ok(output) => {
                    if !output.status.success() {
                        log::error!(
                            "Failed to append silence to track {} of file {}",
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
                    panic!("Error appending silence to track, is FFMPEG installed to path?");
                }
            }
        }
    }
}
