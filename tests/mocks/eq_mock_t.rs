use super::eq_mock_u::EqMockU;

#[allow(clippy::type_complexity)]
pub struct EqMockT {
	ident: i32,
	eq_t: Box<dyn Fn(&EqMockT) -> bool>,
	eq_u: Box<dyn Fn(&EqMockU) -> bool>,
	ne_t: Option<Box<dyn Fn(&EqMockT) -> bool>>,
	ne_u: Option<Box<dyn Fn(&EqMockU) -> bool>>,
}

impl EqMockT {
	#[allow(clippy::type_complexity)]
	pub fn new(
		ident: i32,
		eq_t: Box<dyn Fn(&EqMockT) -> bool>,
		eq_u: Box<dyn Fn(&EqMockU) -> bool>,
		ne_t: Option<Box<dyn Fn(&EqMockT) -> bool>>,
		ne_u: Option<Box<dyn Fn(&EqMockU) -> bool>>,
	) -> Self {
		Self {
			ident,
			eq_t,
			eq_u,
			ne_t,
			ne_u,
		}
	}

	pub fn ident(&self) -> i32 {
		self.ident
	}
}

impl PartialEq for EqMockT {
	fn eq(&self, other: &Self) -> bool {
		let bind = &self.eq_t;
		bind(other)
	}

	#[allow(clippy::partialeq_ne_impl)]
	fn ne(&self, other: &Self) -> bool {
		if let Some(ne) = &self.ne_t {
			ne(other)
		} else {
			!(self.eq_t)(other)
		}
	}
}

impl PartialEq<EqMockU> for EqMockT {
	fn eq(&self, other: &EqMockU) -> bool {
		let bind = &self.eq_u;
		bind(other)
	}
}
