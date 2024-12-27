use super::eq_mock_t::EqMockT;

#[allow(clippy::type_complexity)]
pub struct EqMockU {
	ident: i32,
	eq_t: Box<dyn Fn(&EqMockT) -> bool>,
	eq_u: Box<dyn Fn(&EqMockU) -> bool>,
	ne_t: Option<Box<dyn Fn(&EqMockT) -> bool>>,
	ne_u: Option<Box<dyn Fn(&EqMockU) -> bool>>,
}

impl EqMockU {
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

impl PartialEq for EqMockU {
	fn eq(&self, other: &Self) -> bool {
		let bind = &self.eq_u;
		bind(other)
	}

	#[allow(clippy::partialeq_ne_impl)]
	fn ne(&self, other: &Self) -> bool {
		if let Some(ne) = &self.ne_u {
			ne(other)
		} else {
			!(self.eq_u)(other)
		}
	}
}

impl PartialEq<EqMockT> for EqMockU {
	fn eq(&self, other: &EqMockT) -> bool {
		let bind = &self.eq_t;
		bind(other)
	}

	#[allow(clippy::partialeq_ne_impl)]
	fn ne(&self, other: &EqMockT) -> bool {
		if let Some(ne) = &self.ne_t {
			ne(other)
		} else {
			!(self.eq_t)(other)
		}
	}
}
