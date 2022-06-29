pub mod submit_exercise;
pub use submit_exercise::*;
mod state_reader;

macro_rules! unwrap_result_or_return {
	( $e:expr, $f:expr) => {
		match $e {
			Ok(x) => x,
			Err(_) => return $f,
		}
	};
}
pub(crate) use unwrap_result_or_return;

macro_rules! unwrap_option_or_return {
	( $e:expr, $f:expr) => {
		match $e {
			Some(x) => x,
			None => return $f,
		}
	};
}
pub(crate) use unwrap_option_or_return;
