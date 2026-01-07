//! WARNING: If you're using the latest github version of gdext, use this crate's latest github version too,
//! it won't compile otherwise.

#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
#![feature(unboxed_closures)]
#![cfg_attr(feature = "async", feature(async_fn_traits))]
#![allow(clippy::needless_return)]
#![allow(clippy::useless_conversion)]
#![allow(unused_doc_comments)]
#![allow(private_bounds)]
#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use godot::builtin::{Callable, Variant};

mod builder;
mod coroutine;
mod pinky_promise;
mod start_coroutine;
mod yielding;

#[cfg(feature = "async")]
mod start_async_task;

pub(crate) enum OnFinishCall {
	Closure(Box<dyn FnOnce(Variant)>),
	Callable(Callable),
}

pub mod prelude {
	pub use crate::coroutine::{
		IsFinished, IsPaused, IsRunning, PollMode, SpireCoroutine, SIGNAL_FINISHED,
	};

	pub use crate::yielding::{
		frames, seconds, wait_until, wait_while, KeepWaiting, SpireYield as Yield,
		WaitUntilFinished,
	};

	pub use crate::builder::CoroutineBuilder;
	pub use crate::start_coroutine::StartCoroutine;

	#[cfg(feature = "async")]
	pub use crate::start_async_task::StartAsyncTask;
}
