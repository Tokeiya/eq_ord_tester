use super::eq_mock_t::EqMockT;

pub struct EqMockU<T, U> {
	ident: i32,
	predicate_t: T,
	predicate_u: U,
}

impl<T, U> EqMockU<T, U>
where
	T: Fn(&EqMockT<T, U>) -> bool,
	U: Fn(&EqMockU<T, U>) -> bool,
{
	pub fn generate(ident: i32, predicate_t: T, predicate_u: U) -> Self {
		Self {
			ident,
			predicate_t,
			predicate_u,
		}
	}

	pub fn ident(&self) -> i32 {
		self.ident
	}
}

impl<T, U> PartialEq for EqMockU<T, U>
where
	T: Fn(&EqMockU<T, U>) -> bool,
{
	fn eq(&self, other: &Self) -> bool {
		let bind = &self.predicate_t;
		bind(other)
	}
}

impl<T, U> PartialEq<EqMockT<T, U>> for EqMockU<T, U>
where
	U: Fn(&EqMockT<T, U>) -> bool,
{
	fn eq(&self, other: &EqMockT<T, U>) -> bool {
		let bind = &self.predicate_u;
		bind(other)
	}
}
