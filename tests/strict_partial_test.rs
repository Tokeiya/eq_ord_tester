extern crate eq_ord_tester;

mod mocks;

use eq_ord_tester::strict_partial_eq::*;
use mocks::prelude::*;

#[test]
fn reflexive() {
	reflexive_test((10, 10));
}

#[test]
#[should_panic]
fn invalid_reflexive() {
	reflexive_test((10, 20));
}

#[test]
fn transitive() {
	transitive_test((10, 10), (10, 10), (10, 10));
}

#[test]
fn invalid_transitive() {
	assume_panic(|| transitive_test((10, 10), (10, 10), (10, 11)));
	assume_panic(|| transitive_test((10, 10), (10, 1), (10, 10)));
	assume_panic(|| transitive_test((10, 1), (10, 10), (10, 10)));
}

#[test]
fn symmetric() {
	symmetric_test((10, 10), (10, 10));
}

#[test]
fn invalid_symmetric() {
	assume_panic(|| symmetric_test((1, 1), (1, 2)));
	assume_panic(|| symmetric_test((1, 2), (1, 1)));
}

#[test]
fn not_eq() {
	not_eq_test((1, 1), (2, 2));
}

#[test]
fn invalid_not_eq() {
	assume_panic(|| not_eq_test((1, 2), (1, 1)));
	assume_panic(|| not_eq_test((1, 1), (1, 2)));
}
