use super::eq_mock_u::EqMockU;

pub struct EqMockT<T, U> {
	ident: i32,
	predicate_t: T,
	predicate_u: U,
}

impl<T, U> EqMockT<T, U>
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

impl<T, U> PartialEq for EqMockT<T, U>
where
	T: Fn(&EqMockT<T, U>) -> bool,
{
	fn eq(&self, other: &Self) -> bool {
		let bind = &self.predicate_t;
		bind(other)
	}
}

impl<T, U> PartialEq<EqMockU<T, U>> for EqMockT<T, U>
where
	U: Fn(&EqMockU<T, U>) -> bool,
{
	fn eq(&self, other: &EqMockU<T, U>) -> bool {
		let bind = &self.predicate_u;
		bind(other)
	}
}
