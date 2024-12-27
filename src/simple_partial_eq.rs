use super::partial_eq::*;

pub fn reflexive_test<T>(x: T)
where
	T: PartialEq<T>,
{
	partial_eq_test(&x, &x)
}

pub fn transitive_test<T>(x: T, y: T, z: T)
where
	T: PartialEq<T>,
{
	partial_eq_test(&x, &y);
	partial_eq_test(&y, &z);
	partial_eq_test(&x, &z);
}

pub fn symmetric_test<T, U>(x: T, y: U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	partial_eq_test(&x, &y);
	partial_eq_test(&y, &x);
}

pub fn not_eq_test<T, U>(x: T, y: U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	partial_ne_test(&x, &y);
	partial_ne_test(&y, &x);
}
