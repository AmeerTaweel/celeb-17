use crate::log;
use std::process;

pub fn with_error(msg: &str) -> ! {
	log::error(msg);
	process::exit(1);
}

pub fn with_warn(msg: &str) -> ! {
	log::warn(msg);
	process::exit(1);
}

pub fn with_success(msg: &str) -> ! {
	log::success(msg);
	process::exit(0);
}

pub trait UnwrapOrExit<T> {
	fn unwrap_or_exit_with_error(self, msg: &str) -> T;
}

impl<T, E> UnwrapOrExit<T> for Result<T, E> {
	fn unwrap_or_exit_with_error(self, msg: &str) -> T {
		match self {
			Ok(v) => v,
			Err(_) => with_error(msg)
		}
	}
}

impl<T> UnwrapOrExit<T> for Option<T> {
	fn unwrap_or_exit_with_error(self, msg: &str) -> T {
		match self {
			Some(v) => v,
			None => with_error(msg)
		}
	}
}
