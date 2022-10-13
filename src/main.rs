use clap::{ArgAction, Parser};
use std::{fmt, fs::File, path::Path, process};

use clip::InputClip;

mod clip;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	output_name: String,

	#[arg(short, long = "input-clip")]
	input_clip_specs: Vec<String>,
}

fn main() {
	let args = Args::parse();

	let mut clips: Vec<InputClip> = args
		.input_clip_specs
		.iter()
		.map(|s| {
			InputClip::from_spec(s).unwrap_or_else(|err| {
				eprintln!("trimmeroni: error: {}", err.desc);
				process::exit(1)
			})
		})
		.collect();

	println!("{:?}", clips);

	let tmp_dir = tempfile::tempdir().unwrap_or_else(|err| {
		eprintln!("trimmeroni: error: {}", err);
		process::exit(2)
	});

	#[rustfmt::skip]
	let mut concat_list_file = File::create(
		tmp_dir
			.path()
			.join(format!("{}.trimmeroni_concat.txt", args.output_name)),
	).unwrap_or_else(|err| {
		eprintln!("trimmeroni: error: {}", err);
		process::exit(2)
	});

	for clip in &clips {
		for (idx, segment) in clip.segments.iter().enumerate() {
			let temp_filename = format!("{}.tmp.{}.mp4", clip.filename.clone(), idx);
			let path = Path::new(&temp_filename);
		}
	}
}
