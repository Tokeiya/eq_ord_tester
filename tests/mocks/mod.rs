pub mod eq_mock;
pub mod eq_mock_t;
pub mod eq_mock_u;

pub mod prelude {
	use std::panic::catch_unwind;

	pub fn assume_panic(proc: impl FnOnce() + std::panic::UnwindSafe) {
		let result = catch_unwind(proc);
		assert!(result.is_err())
	}

	pub use super::eq_mock::EqMock;
	pub use super::eq_mock_t::EqMockT;
	pub use super::eq_mock_u::EqMockU;
}
