pub trait DropEmpty: Sized {
	fn drop_empty(self) -> Option<Self>;
}

impl DropEmpty for String {
	fn drop_empty(self) -> Option<String> {
		if self.is_empty() {
			None
		} else {
			Some(self)
		}
	}
}
