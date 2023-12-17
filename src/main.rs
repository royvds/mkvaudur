use clap::Parser;

use mkvaudur::args::MkvAudurArgs;
use mkvaudur::mediainfo::get_mediainfo;
use mkvaudur::{get_files, process_mkv_file, TrackFilter};

fn main() {
    let args = MkvAudurArgs::parse();
    let mkv_files = get_files(&args.filepath);
    let reference_files = args.reference.as_ref().map(get_files);
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
