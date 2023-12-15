# mkvaudur
> Equalize the duration of each audio track in a MKV container to the duration of its video track

Sometimes the duration of an audio track within a MKV container differs from the duration of the video track. In this case, most video players keep playing until the end of the longest track. That means that if an audio track is too long, you will stare at a blank video for some time. Not all video players will work like this though.

To solve any potential issues caused by a disrepancy in track duration, this program trims the end of lengthy audio tracks and appends a silence to short audio tracks to match the length of the video track. When dealing with lossy encoded audio, it is expected that the end result may be off by a few milliseconds. This should pose no noticable issues.

## Dependencies
- [FFMPEG, FFPROBE](https://ffmpeg.org/)
- [MediaInfo](https://mediaarea.net/en/MediaInfo)

## Usage
```
This program requires mediainfo, ffmpeg, and ffprobe to be installed to PATH

Usage: mkvaudur [OPTIONS] <FILEPATH> <COMMAND>

Commands:
  display  List all duration discrepancies
  export   Export (trimmed) tracks
  help     Print this message or the help of the given subcommand(s)

Arguments:
  <FILEPATH>  Filepath, either a MKV file or directory with MKV files in it

Options:
  -t, --treshold <TRESHOLD>    Minimum duration difference, default = 0.044 [default: 0]
  -l, --language <LANGUAGE>    Only select tracks with this language code
  -o, --output <OUTPUT>        Set a custom output directory
  -a, --all                    Display/Export all audio tracks regardless of treshold (tracks not meeting treshold will not be cut)
  -r, --reference <REFERENCE>  Path to use the video track duration of other mkv file(s)
  -h, --help                   Print help
```
