#![no_std]
#![allow(bad_style)]
#![allow(unused_macros)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::let_and_return)]

//! Global GLES loader and bindings for GLES 3.0 Core.
//!
//! It was generated using `gl_generator`, so it works basically like the `gl`
//! crate does.
//!
//! ```ignore
//! use gles30 as gl;
//!
//! gl::load_with(|ptr| SDL_GL_GetProcAddress(ptr));
//!
//! gl::ClearColor(0.5, 0.5, 0.5, 1.0);
//! ```
//!
//! The main difference from the `gl` crate is that this crate _only_ loads
//! GLES 3.0 Core. If you want to _really_ make sure that you're not
//! accidentally relying on extra functions during development that you won't
//! have during deployment you can use this crate.
//!
//! ## Features
//!
//! There's two features you can turn on. Both will print stuff to the console
//! if `debug_assertions` are enabled.
//!
//! * `debug_trace_messages`: If enabled, immediately _before_ a call to a GL
//!   function it'll print the function's name. I expect that you'd keep this
//!   off most of the time, but if you're getting segfaults it might help to get
//!   a message before each GL call and hopefully you can spot the problem.
//! * `debug_error_checks`: If enabled, immediately _after_ all calls to GL
//!   there's an additional call to `glGetError`. If the error value is
//!   something other than `NO_ERROR` then you'll immediately get an error
//!   message printed showing the name of the function, the arguments you
//!   passed, and the error code.
//!
//! ## `no_std` Support
//!
//! This library is `no_std` friendly! It's just bindings and loader callbacks
//! after all.
//!
//! However, having either of the above features for message printing enabled
//! **will** cause the crate to link to the `std` crate whenever
//! `debug_assertions` are on. It's the price we pay for debugging messages.

#[cfg(all(
  debug_assertions,
  any(feature = "debug_error_checks", feature = "trace_messages")
))]
extern crate std;

macro_rules! error {
  ($($tokens:tt)*) => {
    #[cfg(all(debug_assertions, feature = "debug_error_checks"))]
    {
      std::println!($($tokens)*);
    }
  }
}

macro_rules! trace {
  ($($tokens:tt)*) => {
    #[cfg(all(debug_assertions, feature = "trace_messages"))]
    {
      std::println!($($tokens)*);
    }
  }
}

mod global_loader;
pub use global_loader::*;

use core::{
  mem::transmute,
  ptr::{null_mut, NonNull},
  sync::atomic::{AtomicPtr, Ordering},
};
type OptVoidPtr = Option<NonNull<c_void>>;

pub use core::ffi::c_void;

#[cfg(not(windows))]
pub use libc::{
  c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar,
  c_uint, c_ulong, c_ulonglong, c_ushort,
};
#[cfg(windows)]
pub type c_char = i8;
#[cfg(windows)]
pub type c_schar = i8;
#[cfg(windows)]
pub type c_uchar = u8;
#[cfg(windows)]
pub type c_short = i16;
#[cfg(windows)]
pub type c_ushort = u16;
#[cfg(windows)]
pub type c_int = i32;
#[cfg(windows)]
pub type c_uint = u32;
#[cfg(windows)]
pub type c_long = i32;
#[cfg(windows)]
pub type c_ulong = u32;
#[cfg(windows)]
pub type c_longlong = i64;
#[cfg(windows)]
pub type c_ulonglong = u64;
#[cfg(windows)]
pub type c_float = f32;
#[cfg(windows)]
pub type c_double = f64;
