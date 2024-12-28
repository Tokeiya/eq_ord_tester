use super::partial_ord::simple_ord_test as partial_simple_ord_test;

pub fn simple_ord_test<T: PartialOrd>(pivot: T, greater: T, less: T, eq: T) {
	partial_simple_ord_test(&pivot, &greater, &less, &eq);

	assert!(pivot < less);
	assert!(pivot <= less);

	assert!(pivot < greater);
	assert!(pivot <= greater);

	assert!(pivot <= eq);
	assert!(pivot >= eq);
}
