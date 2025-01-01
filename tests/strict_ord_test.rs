extern crate eq_ord_tester;
mod mocks;

use eq_ord_tester::ord::*;
use mocks::prelude::*;
#[test]
fn simple_ord() {
	simple_ord_test(10, 11, 9, 10);
}

#[test]
fn invalid_simple_ord() {
	assume_panic(|| simple_ord_test(10, 9, 11, 9));
	assume_panic(|| simple_ord_test(10, 10, 9, 10));
	assume_panic(|| simple_ord_test(10, 11, 10, 10));

	assume_panic(|| simple_ord_test(10, 11, 9, 11));
}

#[test]
fn complex_ord() {
	complex_ord_test((10, 10), (11, 11), (9, 9), (10, 10));
}

#[test]
fn invalid_complex_ord() {
	assume_panic(|| complex_ord_test((10, 10), (11, 11), (9, 9), (9, 9)));
	assume_panic(|| complex_ord_test((10, 10), (11, 11), (9, 9), (10, 9)));
	assume_panic(|| complex_ord_test((10, 10), (11, 11), (9, 9), (9, 10)));

	assume_panic(|| complex_ord_test((10, 11), (12, 12), (9, 9), (10, 11)));
}
