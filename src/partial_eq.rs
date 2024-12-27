pub fn partial_eq_test<T, U>(x: T, y: U)
where
	T: PartialEq<U>,
{
	assert!(x.eq(&y));
	assert!(!x.ne(&y));
}

pub fn partial_ne_test<T, U>(x: &T, y: &U)
where
	T: PartialEq<U>,
{
	assert!(x.ne(y));
	assert!(!x.eq(y));
}
