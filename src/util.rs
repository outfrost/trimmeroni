pub fn none_if_empty(s: String) -> Option<String> {
	if s.is_empty() {
		None
	} else {
		Some(s)
	}
}
