use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use std::str;

// This program uses mediainfo because ffprobe can only retrieve the duration
// of an audio track when a DURATION tag has been set in the MKV file. It can
// not calculate the duration nor can it provide us with enough data to
// calculate it (stream size is missing)

pub fn get_mediainfo(input_file: &PathBuf) -> Value {
    match Command::new("mediainfo")
        .arg("--Output=JSON")
        .arg(input_file)
        .output()
    {
        Ok(output) => {
            let data = str::from_utf8(&output.stdout).unwrap();
            return serde_json::from_str(data).unwrap();
        }
        Err(_) => {
            panic!("Could not retrieve mediainfo of file, is mediainfo installed to path?");
        }
    };
}

pub fn get_audio_ext(track: &Value) -> String {
    let audio_format = track["Format"]
        .as_str()
        .unwrap()
        .parse::<String>()
        .unwrap()
        .to_lowercase();
    if audio_format == "pcm" {
        return ".wav".to_string();
    }
    return ".".to_string() + audio_format.as_str();
}
