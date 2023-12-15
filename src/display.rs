use std::path::Path;

use serde_json::Value;

pub fn display(
    mkv_mediainfo: &Value,
    video_duration: f64,
    treshold: f64,
    language: &Option<String>,
    process_all: bool,
) {
    println!(
        "{} | Video Duration: {}",
        Path::new(mkv_mediainfo["media"]["@ref"].as_str().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        video_duration
    );
    let tracks = mkv_mediainfo["media"]["track"].as_array().unwrap();
    for track in tracks {
        if track["@type"] != "Audio" {
            continue;
        }
        let duration_difference: f64 =
            track["Duration"].as_str().unwrap().parse::<f64>().unwrap() - video_duration;

        let track_language = track["Language"].as_str();

        if ((language.is_none()
            || (track_language.is_some() && track_language.unwrap() == language.as_ref().unwrap()))
            && f64::abs(duration_difference) > treshold)
            || process_all
        {
            println!(
                "Track {} ({}): Duration: {} Difference: {}",
                track["ID"].as_str().unwrap().parse::<i64>().unwrap(),
                track_language.unwrap_or("und"),
                track["Duration"].as_str().unwrap().parse::<f64>().unwrap(),
                duration_difference
            );
        }
    }
    println!()
}
