pub struct EqMock<T> {
	ident: i32,
	pred: T,
}

impl<T: PartialEq> PartialEq for EqMock<T> {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl<T: PartialEq> EqMock<T> {
	pub fn new(ident: i32, pred: T) -> Self {
		Self { ident, pred }
	}

	pub fn ident(&self) -> i32 {
		self.ident
	}
}
