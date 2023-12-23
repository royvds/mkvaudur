use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use clap::Parser;

use mkvaudur::args::MkvAudurArgs;
use mkvaudur::mediainfo::get_mediainfo;
use mkvaudur::{get_files, process_mkv_file, TrackFilter};

fn main() {
    let args = MkvAudurArgs::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mkv_files = iter_get_files(&args.filepath, "input");
    let reference_files = args
        .reference
        .as_ref()
        .map(|p| iter_get_files(p.as_path(), "reference"));

    let track_filter = TrackFilter {
        treshold: args.treshold,
        language: args.language,
        process_all: args.all,
    };

    if reference_files.is_some() {
        for (mkv_file, ref_file) in mkv_files.iter().zip(reference_files.unwrap().iter()) {
            let mkv_mediainfo = get_mediainfo(mkv_file);
            let ref_mediainfo = get_mediainfo(ref_file);
            process_mkv_file(
                mkv_file,
                &mkv_mediainfo,
                &ref_mediainfo,
                &args.mode,
                &track_filter,
                &args.output,
            );
        }
    } else {
        for mkv_file in mkv_files {
            let mkv_mediainfo = get_mediainfo(&mkv_file);
            process_mkv_file(
                &mkv_file,
                &mkv_mediainfo,
                &mkv_mediainfo,
                &args.mode,
                &track_filter,
                &args.output,
            );
        }
    }
}

fn iter_get_files(filepath: &Path, filepath_name: &str) -> Vec<PathBuf> {
    let mut mkv_files_path = filepath.to_owned();
    match get_files(&mkv_files_path) {
        Ok(mkv_files) => mkv_files,
        Err(_) => {
            while get_files(&mkv_files_path).is_err() {
                println!(
                    "The given {} filepath is invalid or does not include any mkv files.",
                    filepath_name
                );
                print!("Please enter a new path: ");
                let _ = stdout().flush();
                let mut s = String::new();
                let _ = stdin().read_line(&mut s);
                mkv_files_path = PathBuf::from(s.trim());
            }
            get_files(&mkv_files_path).unwrap()
        }
    }
}
