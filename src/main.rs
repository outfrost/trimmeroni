use clap::{ArgAction, Parser};
use regex::Regex;
use std::{fmt, fs::File, path::Path, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	output_name: String,

	#[arg(short, long = "input-clip")]
	input_clip_specs: Vec<String>,
}

#[derive(Debug)]
struct InputClip {
	filename: String,
	segments: Vec<ClipSegment>,
}

#[derive(Debug)]
struct ClipSegment {
	start_timecode: Option<String>,
	end_timecode: Option<String>,
}

impl ClipSegment {
	fn new() -> Self {
		Self {
			start_timecode: None,
			end_timecode: None,
		}
	}

	fn from_parts(start: &str, end: &str) -> Result<Self, ParseError> {
		let re = Regex::new(r"^((\d+:)?\d{2}:\d{2}(\.\d+)?)?$").unwrap();
		if !re.is_match(start) {
			return Err(ParseError {
				desc: String::from("invalid timecode: ") + start,
			});
		}
		if !re.is_match(end) {
			return Err(ParseError {
				desc: String::from("invalid timecode: ") + end,
			});
		}

		Ok(Self {
			start_timecode: none_if_empty(start.to_owned()),
			end_timecode: none_if_empty(end.to_owned()),
		})
	}
}

#[derive(Debug)]
enum ParseState {
	Filename,
	SegmentStart,
	SegmentEnd,
}

#[derive(Debug)]
struct ParseError {
	desc: String,
}

impl InputClip {
	fn from_spec(spec: &str) -> Result<Self, ParseError> {
		let mut state = ParseState::Filename;

		let mut filename = String::new();
		let mut segments: Vec<ClipSegment> = vec![];
		let mut seg_start = String::new();
		let mut seg_end = String::new();
		let mut escape = false;

		for c in spec.chars() {
			state = match state {
				ParseState::Filename => {
					if escape {
						filename.push(c);
						escape = false;
						ParseState::Filename
					} else if c == '\\' {
						escape = true;
						ParseState::Filename
					} else if c == '@' {
						ParseState::SegmentStart
					} else {
						filename.push(c);
						ParseState::Filename
					}
				},
				ParseState::SegmentStart => {
					if c.is_whitespace() {
						ParseState::SegmentStart
					} else if c == '-' {
						ParseState::SegmentEnd
					} else {
						seg_start.push(c);
						ParseState::SegmentStart
					}
				},
				ParseState::SegmentEnd => {
					if c.is_whitespace() {
						ParseState::SegmentEnd
					} else if c == ',' {
						segments.push(ClipSegment::from_parts(&seg_start, &seg_end)?);
						seg_start.clear();
						seg_end.clear();
						ParseState::SegmentStart
					} else {
						seg_end.push(c);
						ParseState::SegmentEnd
					}
				},
			};
		}

		if !seg_start.is_empty() || !seg_end.is_empty() {
			segments.push(ClipSegment::from_parts(&seg_start, &seg_end)?);
		}

		if segments.is_empty() {
			segments.push(ClipSegment::new());
		}

		Ok(Self {
			filename: filename.trim_end().into(),
			segments,
		})
	}
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

	let mut concat_list_file = File::create(tmp_dir.path().join(format!("{}.trimmeroni_concat.txt", args.output_name))).unwrap_or_else(|err| {
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

fn none_if_empty(s: String) -> Option<String> {
	if s.is_empty() {
		None
	} else {
		Some(s)
	}
}
