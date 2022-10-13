use crate::util;
use regex::Regex;

#[derive(Debug)]
enum ParseState {
	Filename,
	SegmentStart,
	SegmentEnd,
}

#[derive(Debug)]
pub struct ParseError {
	pub desc: String,
}

#[derive(Debug)]
pub struct ClipSegment {
	pub start_timecode: Option<String>,
	pub end_timecode: Option<String>,
}

impl ClipSegment {
	pub fn new() -> Self {
		Self {
			start_timecode: None,
			end_timecode: None,
		}
	}

	pub fn from_parts(start: &str, end: &str) -> Result<Self, ParseError> {
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
			start_timecode: util::none_if_empty(start.to_owned()),
			end_timecode: util::none_if_empty(end.to_owned()),
		})
	}
}

#[derive(Debug)]
pub struct InputClip {
	pub filename: String,
	pub segments: Vec<ClipSegment>,
}

impl InputClip {
	pub fn from_spec(spec: &str) -> Result<Self, ParseError> {
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
