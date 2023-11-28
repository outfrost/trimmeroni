use clap::Parser;
use std::{
	fs::OpenOptions,
	io::Write,
	path::Path,
	process::{self, Command},
};

use clip::InputClip;

mod clip;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	output_name: String,

	#[arg(short, long = "input-clip")]
	input_clip_specs: Vec<String>,

	#[arg(short, long, help = "Print additional information while working")]
	verbose: bool,
}

fn main() {
	let args = Args::parse();

	let clips: Vec<InputClip> = args
		.input_clip_specs
		.iter()
		.map(|s| {
			InputClip::from_spec(s).unwrap_or_else(|err| {
				eprintln!("trimmeroni: error: {}", err.desc);
				process::exit(1)
			})
		})
		.collect();

	if args.verbose {
		eprintln!("trimmeroni: creating temporary directory");
	}
	let tmp_dir = tempfile::tempdir().unwrap_or_else(|err| {
		eprintln!(
			"trimmeroni: error: couldn't create temporary directory: {}",
			err
		);
		process::exit(2)
	});

	if args.verbose {
		eprintln!(
			"trimmeroni: putting temporary files in {}",
			tmp_dir.path().display()
		);
	}

	let concat_list_path = tmp_dir.path().join(format!(
		"{}.trimmeroni_concat.txt",
		Path::new(&args.output_name)
			.file_name()
			.unwrap_or_else(|| {
				eprintln!(
					"trimmeroni: error: output path does not seem to include a file name: {}",
					args.output_name
				);
				process::exit(1)
			})
			.to_string_lossy()
	));

	// trim all clip segments into temporary video files
	{
		if args.verbose {
			eprintln!(
				"trimmeroni: creating temporary list file: {}",
				concat_list_path.display()
			);
		}
		let mut concat_list_file = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&concat_list_path)
			.unwrap_or_else(|err| {
				eprintln!(
					"trimmeroni: error: couldn't create temporary list file {}: {}",
					concat_list_path.display(),
					err
				);
				process::exit(2)
			});

		let mut trim_successful = true;

		for (clip_idx, clip) in clips.iter().enumerate() {
			for (seg_idx, segment) in clip.segments.iter().enumerate() {
				eprintln!(
					"trimmeroni: copying segment {}/{} from {}",
					seg_idx + 1,
					clip.segments.len(),
					&clip.filename
				);

				let source_filename = match Path::new(&clip.filename).file_name() {
					Some(name) => name.to_string_lossy(),
					_ => {
						eprintln!("trimmeroni: error: clip source path does not seem to include a file name");
						trim_successful = false;
						break;
					},
				};
				let temp_filename =
					format!("{}.{}.tmp.{}.mp4", clip_idx, &source_filename, seg_idx);
				let temp_path = tmp_dir.path().join(&temp_filename);

				let mut command = Command::new("ffmpeg");
				command.arg("-hide_banner");
				command.args(["-loglevel", if args.verbose { "info" } else { "warning" }]);
				command.args(["-i", &clip.filename]);

				if let Some(timecode) = &segment.start_timecode {
					command.args(["-ss", &timecode]);
				}

				if let Some(timecode) = &segment.end_timecode {
					command.args(["-to", &timecode]);
				}

				command.args(["-c", "copy"]);
				command.arg(&temp_path);

				let mut command_display =
					format!("\"{}\"", &command.get_program().to_string_lossy());
				for arg in command.get_args() {
					command_display += " \"";
					command_display += &arg.to_string_lossy();
					command_display += "\"";
				}
				if args.verbose {
					eprintln!("trimmeroni: command: {}", command_display);
				}

				let success = match command.status() {
					Ok(status) => status.success(),
					Err(e) => {
						eprintln!("trimmeroni: error: failed to execute ffmpeg: {}", e);
						false
					},
				};

				eprintln!();

				if success {
					if let Err(e) = write!(concat_list_file, "file '{}'\n", temp_path.display()) {
						eprintln!(
							"trimmeroni: error: cannot write to temporary list file: {}",
							e
						);
						trim_successful = false;
						break;
					}
				} else {
					trim_successful = false;
					break;
				}
			}
			if !trim_successful {
				break;
			}
		}

		if !trim_successful {
			eprintln!("trimmeroni: error: trimming clips failed, stopping");
			process::exit(3);
		}
	}

	// concatenate the clips
	eprintln!(
		"trimmeroni: concatenating segments into {}",
		&args.output_name
	);
	eprintln!();

	let exit_status = Command::new("ffmpeg")
		.arg("-hide_banner")
		.args(["-loglevel", if args.verbose { "info" } else { "warning" }])
		.args(["-f", "concat"])
		.args(["-safe", "0"])
		.args(["-i", &concat_list_path.to_string_lossy()])
		.args(["-c", "copy"])
		.arg(&args.output_name)
		.status();

	eprintln!();

	let success = match exit_status {
		Ok(status) => status.success(),
		Err(e) => {
			eprintln!("trimmeroni: error: failed to execute ffmpeg: {}", e);
			false
		},
	};

	if !success {
		eprintln!("trimmeroni: error: concatenating clips failed");
		process::exit(3);
	}
}
