pub fn reflexive_test<T>(x: &T)
where
	T: PartialEq<T>,
{
	todo!()
}

pub fn transitive_test<T>(x: &T, y: &T, z: &T)
where
	T: PartialEq<T>,
{
	todo!()
}

pub fn symmetric_test<T, U>(x: &T, y: &U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	todo!()
}

pub fn not_eq_test<T, U>(x: &T, y: &U)
where
	T: PartialEq<U>,
	U: PartialEq<T>,
{
	todo!()
}
