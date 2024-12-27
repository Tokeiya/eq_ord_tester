extern crate eq_ord_tester;
mod mocks;
use eq_ord_tester::partial_eq::{partial_eq_test, partial_ne_test};
use mocks::prelude::*;

#[test]
fn partial_eq() {
	partial_eq_test(&10, &10);
	assume_panic(|| partial_eq_test(10, 20));
}

#[test]
fn complex_eq_invalid_test() {
	assume_panic(|| {
		let a = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| false)));
		let b = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| true)));

		partial_eq_test(a, b)
	});

	assume_panic(|| {
		let a = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| true)));
		let b = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| false)));

		partial_eq_test(a, b)
	})
}

#[test]
fn partial_ne() {
	partial_ne_test(10, 2);
	assume_panic(|| partial_ne_test(1, 1));
}

#[test]
fn complex_ne_invalid_test() {
	assume_panic(|| {
		let a = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| false)));
		let b = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| true)));

		partial_ne_test(a, b)
	});

	assume_panic(|| {
		let a = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| true)));
		let b = EqMock::new(42, Box::new(|_| true), Some(Box::new(|_| false)));

		partial_ne_test(a, b)
	})
}
