pub struct EqMock {
	ident: i32,
	eq: Box<dyn Fn(&Self) -> bool>,
	ne: Option<Box<dyn Fn(&Self) -> bool>>,
}

impl PartialEq for EqMock {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}

	#[allow(clippy::partialeq_ne_impl)]
	fn ne(&self, other: &Self) -> bool {
		if let Some(ne) = &self.ne {
			ne(other)
		} else {
			!self.eq(other)
		}
	}
}

impl EqMock {
	#[allow(clippy::type_complexity)]
	pub fn new(
		ident: i32,
		eq: Box<dyn Fn(&EqMock) -> bool>,
		ne: Option<Box<dyn Fn(&EqMock) -> bool>>,
	) -> Self {
		Self { ident, eq, ne }
	}

	pub fn ident(&self) -> i32 {
		self.ident
	}
}
