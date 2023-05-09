# trimmeroni

This little tool aims to make it easier to cut and concatenate together any number of video clips,
from any number of input files, into one output file, without reencoding.

It uses the `ffmpeg` program under the hood to copy each requested video segment from the source
files into a temporary directory, and then concatenate them (insert one after another) into
the destination file.

The only requirement is that you have `ffmpeg` installed on your system, accessible from `PATH`.

## Usage

```
Usage: trimmeroni [OPTIONS] <OUTPUT_NAME>

Arguments:
  <OUTPUT_NAME>

Options:
  -i, --input-clip <INPUT_CLIP_SPECS>
  -h, --help                           Print help information
  -V, --version                        Print version information
```

`<INPUT_CLIP_SPECS>` specifies an input file and the timecodes of clips to copy from it. It should
be formatted as follows:

```
INPUT_NAME @ TIMECODE - TIMECODE[, TIMECODE - TIMECODE, ...]
```

where
* `INPUT_NAME` is the input file name; if the file name contains "@", use a backslash to escape it
("\\@"); there is no need to escape spaces;
* each `TIMECODE` can be any seek position supported by `ffmpeg` (e.g. in `h:mm:ss` format),
or empty, meaning either the start or end of the entire video file;
* "`TIMECODE - TIMECODE,`" can be repeated any number of times in order to grab more than one clip
from the input file.

You can use `-i` many times to use clips from many source files.

Remember to quote your `<INPUT_CLIP_SPECS>`, so that `trimmeroni` receives it as a single string.

### Examples

From "cool_gameplay.mkv", grab the segment between 08:34 and 11:02, and save it to "360_noscope.mp4"
```sh
trimmeroni -i 'cool_gameplay.mkv @ 08:34 - 11:02' "360_noscope.mp4"
```

From "camera_footage.mp4", use the segments 04:20 - 04:59, 06:44 - 09:00, and from 14:40 to the end,
concatenate them, and save the result to "check_this_out.mp4"
```sh
trimmeroni -i 'camera_footage.mp4 @ 04:20 - 04:59, 06:44 - 09:00, 14:40 - ' "check_this_out.mp4"
```

Take 38:08 to 45:35, and 1:02:20 to 1:17:52 from "Day1.mp4", and from the beginning to 29:26 from
"Day2.mp4", concatenate, and save the result to "Weekend bike adventure.mp4"
```sh
trimmeroni -i 'Day1.mp4 @ 38:08 - 45:35, 1:02:20 - 1:17:52' -i 'Day2.mp4 @ - 29:26' "Weekend bike adventure.mp4"
```

## Installing

If you have `cargo`, you can install this program from crates.io:
```sh
cargo install trimmeroni
```

## Building

This project uses Rust's stable toolchain, 2021 edition.

To build locally from source:

```sh
git clone https://github.com/outfrost/trimmeroni.git
cd trimmeroni
cargo build
```

You can then run the debug binary like so:

```sh
cargo run -- -i 'file.mkv @ 00:02 - 01:12' "out.mp4"
```

or install it to your `cargo` binaries with:

```sh
cargo install --path .
```

## Contributing

Issues and pull requests welcome.

By opening an issue or pull request for this repository, you acknowledge and agree that
the contributions included in your issue or pull request may be published, used, copied, modified,
repurposed, and reused without limitation, under the terms of the MIT License (included in
[LICENSE](LICENSE)), regardless of whether or not the pull request is ever merged, and whether
or not the issue is ever resolved.
