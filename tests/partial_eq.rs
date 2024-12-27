extern crate eq_ord_tester;
mod mocks;
use eq_ord_tester::partial_eq::{partial_eq_test, partial_ne_test};
use mocks::prelude::*;
use std::panic::catch_unwind;

fn assume_panic(proc: impl FnOnce() + std::panic::UnwindSafe) {
	let result = catch_unwind(proc);
	assert!(result.is_err())
}

#[test]
fn partial_eq() {
	partial_eq_test(&10, &10);
	assume_panic(|| partial_eq_test(10, 20));
}

#[test]
fn complex_invalid_test() {
	todo!()
}
