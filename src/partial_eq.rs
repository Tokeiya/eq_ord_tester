pub fn partial_eq_test<T, U>(x: &T, y: &U)
where
	T: PartialEq<U>,
{
	assert!(x.eq(y))
}

pub fn partial_ne_test<T, U>(x: &T, y: &U)
where
	T: PartialEq<U>,
{
	assert!(x.ne(y))
}

pub mod simple {
	pub fn reflexive_test<T: PartialEq>(x: &T) {
		todo!()
	}
}

pub mod strict {}
#[cfg(test)]
mod test {
	use super::*;
}
