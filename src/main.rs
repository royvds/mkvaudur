use clap::Parser;

use mkvaudur::args::AudiotrimmerArgs;
use mkvaudur::mediainfo::get_mediainfo;
use mkvaudur::{get_files, process_mkv_file};

fn main() {
    let args = AudiotrimmerArgs::parse();
    let mkv_files = get_files(&args.filepath);
    let reference_files = match &args.reference {
        Some(path) => Some(get_files(path)),
        None => None,
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
                args.treshold,
                &args.language,
                args.all,
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
                args.treshold,
                &args.language,
                args.all,
                &args.output,
            );
        }
    }
}
