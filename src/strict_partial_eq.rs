pub fn reflexive_test<T, U>(x: (&T, &U))
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	todo!()
}

pub fn transitive_test<T, U>(x: (&T, &U), y: (&T, &U), z: (&T, &U))
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	todo!()
}

pub fn symmetric_test<T, U>(x: (&T, &U), y: (&T, &U))
where
	T: PartialEq<U>,
	T: PartialEq<T>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	todo!()
}

pub fn not_eq_test<T, U>(x: &T, y: &T)
where
	T: PartialEq<T>,
	T: PartialEq<U>,
	U: PartialEq<T>,
	U: PartialEq<U>,
{
	todo!()
}
