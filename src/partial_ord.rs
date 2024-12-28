use std::cmp::PartialOrd;

pub fn simple_ord_test<T: PartialOrd>(pivot: T, greater: T, less: T, eq: T) {
	//greater
	assert!(pivot < greater);
	assert!(pivot <= greater);
	assert!(matches!(pivot.partial_cmp(&greater), Some(x) if x == std::cmp::Ordering::Less));

	//less
	assert!(pivot > less);
	assert!(pivot >= less);
	assert!(matches!(pivot.partial_cmp(&less), Some(x) if x == std::cmp::Ordering::Greater));

	//eq
	assert!(pivot <= eq);
	assert!(pivot >= eq);
	assert!(matches!(pivot.partial_cmp(&eq), Some(x) if x == std::cmp::Ordering::Equal));
}

pub fn complex_ord_test<T, U>(pivot: (T, U), greater: (T, U), less: (T, U), eq: (T, U))
where
	T: PartialOrd,
	T: PartialOrd<U>,
	U: PartialOrd,
	U: PartialOrd<T>,
{
	//greater
	assert!(pivot.0 < greater.0);
	assert!(pivot.0 <= greater.0);
	assert!(matches!(pivot.0.partial_cmp(&greater.0), Some(x) if x == std::cmp::Ordering::Less));

	assert!(pivot.1 < greater.1);
	assert!(pivot.1 <= greater.1);
	assert!(matches!(pivot.1.partial_cmp(&greater.1), Some(x) if x == std::cmp::Ordering::Less));

	assert!(pivot.0 < greater.1);
	assert!(pivot.0 <= greater.1);
	assert!(matches!(pivot.0.partial_cmp(&greater.1), Some(x) if x == std::cmp::Ordering::Less));

	assert!(pivot.1 < greater.0);
	assert!(pivot.1 <= greater.0);
	assert!(matches!(pivot.1.partial_cmp(&greater.0), Some(x) if x == std::cmp::Ordering::Less));

	//less
	assert!(pivot.0 > less.0);
	assert!(pivot.0 >= less.0);
	assert!(matches!(pivot.0.partial_cmp(&less.0), Some(x) if x == std::cmp::Ordering::Greater));

	assert!(pivot.1 > less.1);
	assert!(pivot.1 >= less.1);
	assert!(matches!(pivot.1.partial_cmp(&less.1), Some(x) if x == std::cmp::Ordering::Greater));

	assert!(pivot.0 > less.1);
	assert!(pivot.0 >= less.1);
	assert!(matches!(pivot.0.partial_cmp(&less.1), Some(x) if x == std::cmp::Ordering::Greater));

	assert!(pivot.1 > less.0);
	assert!(pivot.1 >= less.0);
	assert!(matches!(pivot.1.partial_cmp(&less.0), Some(x) if x == std::cmp::Ordering::Greater));

	//eq
	assert!(pivot.0 <= eq.0);
	assert!(pivot.0 >= eq.0);
	assert!(matches!(pivot.0.partial_cmp(&eq.0), Some(x) if x == std::cmp::Ordering::Equal));

	assert!(pivot.1 <= eq.1);
	assert!(pivot.1 >= eq.1);
	assert!(matches!(pivot.1.partial_cmp(&eq.1), Some(x) if x == std::cmp::Ordering::Equal));

	assert!(pivot.0 <= eq.1);
	assert!(pivot.0 >= eq.1);
	assert!(matches!(pivot.0.partial_cmp(&eq.1), Some(x) if x == std::cmp::Ordering::Equal));

	assert!(pivot.1 <= eq.0);
	assert!(pivot.1 >= eq.0);
	assert!(matches!(pivot.1.partial_cmp(&eq.0), Some(x) if x == std::cmp::Ordering::Equal));
}
