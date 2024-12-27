use super::partial_eq::*;

pub fn reflexive_test<T, U>(x: (T, U))
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	partial_eq_test(&x.0, &x.0);
	partial_eq_test(&x.1, &x.1);

	partial_eq_test(&x.0, &x.1);
	partial_eq_test(&x.1, &x.0);
}

pub fn transitive_test<T, U>(x: (T, U), y: (T, U), z: (T, U))
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	partial_eq_test(&x.0, &y.0);
	partial_eq_test(&y.0, &z.0);
	partial_eq_test(&x.0, &z.0);

	partial_eq_test(&x.1, &y.1);
	partial_eq_test(&y.1, &z.1);
	partial_eq_test(&x.1, &z.1);

	partial_eq_test(&x.1, &y.0);
	partial_eq_test(&y.0, &z.0);
	partial_eq_test(&x.1, &z.0);

	partial_eq_test(&x.0, &y.1);
	partial_eq_test(&y.1, &z.1);
	partial_eq_test(&x.0, &z.1);

	partial_eq_test(&x.0, &y.1);
	partial_eq_test(&y.1, &z.0);
	partial_eq_test(&x.0, &z.0);

	partial_eq_test(&x.1, &y.0);
	partial_eq_test(&y.0, &z.1);
	partial_eq_test(&x.1, &z.1);

	partial_eq_test(&x.0, &y.0);
	partial_eq_test(&y.0, &z.1);
	partial_eq_test(&x.0, &z.1);

	partial_eq_test(&x.1, &y.1);
	partial_eq_test(&y.1, &z.0);
	partial_eq_test(&x.1, &z.0);

	partial_eq_test(&x.1, &y.1);
	partial_eq_test(&y.1, &z.0);
	partial_eq_test(&x.1, &z.0);

	partial_eq_test(&x.0, &y.0);
	partial_eq_test(&y.0, &z.1);
	partial_eq_test(&x.0, &z.1);
}

pub fn symmetric_test<T, U>(x: (T, U), y: (T, U))
where
	T: PartialEq<U>,
	T: PartialEq<T>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	partial_eq_test(&x.0, &y.0);
	partial_eq_test(&y.0, &x.0);

	partial_eq_test(&x.1, &y.1);
	partial_eq_test(&y.1, &x.1);

	partial_eq_test(&x.0, &y.1);
	partial_eq_test(&y.1, &x.0);

	partial_eq_test(&x.1, &y.0);
	partial_eq_test(&y.0, &x.1);
}

pub fn not_eq_test<T, U>(x: (T, U), y: (T, U))
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	partial_ne_test(&x.0, &y.0);
	partial_ne_test(&y.0, &x.0);

	partial_ne_test(&x.1, &y.1);
	partial_ne_test(&y.1, &x.1);

	partial_ne_test(&x.0, &y.1);
	partial_ne_test(&y.1, &x.0);

	partial_ne_test(&x.1, &y.0);
	partial_ne_test(&y.0, &x.1);
}
