extern crate eq_ord_tester;
mod mocks;

use eq_ord_tester::simple_partial_eq::*;
use mocks::prelude::*;
#[test]
fn reflexive() {
	reflexive_test(10);

	assume_panic(|| {
		let mock = EqMock::new(1, Box::new(|_| false), None);
		reflexive_test(mock)
	});

	assume_panic(|| {
		let mock = EqMock::new(1, Box::new(|_| true), Some(Box::new(|_| true)));
		reflexive_test(mock);
	});
}

#[test]
fn transitive() {
	transitive_test(10, 10, 10);
}

#[test]
#[should_panic]
fn invalid_transitive() {
	let x = EqMock::new(1, Box::new(|x| x.ident() == 10), Some(Box::new(|_| false)));
	let y = EqMock::new(10, Box::new(|x| x.ident() == 10), Some(Box::new(|_| false)));
	let z = EqMock::new(1, Box::new(|x| x.ident() == 10), Some(Box::new(|_| false)));

	transitive_test(x, y, z)
}

#[test]
fn symmetric() {
	symmetric_test(10, 10);
}

#[test]
#[should_panic]
fn invalid_symmetric() {
	symmetric_test(10, 11);
}

#[test]
fn not_eq() {
	not_eq_test(10, 11)
}

#[test]
#[should_panic]
fn invalid_not_eq() {
	not_eq_test(10, 10)
}
