#![no_std]
#![allow(bad_style)]
#![allow(unused_macros)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::let_and_return)]

//! Global GLES loader and bindings for GLES 3.0.
//!
//! * Unlike with the [`gl`](https://docs.rs/gl) crate, all GLES functions and
//!   constants provided here are given under their official names, without an
//!   intended module prefix.
//!     * In other words, for the function `glClearColor` the `gl` crate exposes
//!       it as `ClearColor` so that you call it with `gl::ClearColor`. This
//!       crate just uses the real name of `glClearColor`.
//!     * Similarly, for a constant such as `GL_BLEND`, the `gl` crate exposes
//!       it as `BLEND` and then you use it as `gl::BLEND`. This crate just
//!       gives you `GL_BLEND`.
//! * Loading of all functions can be performed with a single call to
//!   [`load_gles_with`]. You provide a function pointer lookup function and it
//!   loads all the individual functions. If a function can't be loaded under
//!   the main name, the loader will attempt to use fallback names when
//!   possible.
//! * At your option, you can load _individual_ functions. Each function named
//!   `glFoo` actually has a module `Foo` which contains `pub fn is_loaded() ->
//!   bool`, allowing you to check the loaded status of that function, and `pub
//!   unsafe fn load_with<F>(mut load_fn: F)` which works just like
//!   `load_gles_with` except for only that one function.
//!     * These modules are all hidden from the normal documentation because you
//!       shouldn't ever need to use them and it clogs up the docs.
//! * On all functions with documentation, I have attempted to give a link to
//!   the correct khronos.org documentation page.
//!     * Doc links started as auto-generated links to the page with the same
//!       function name, but Kronos sorts the "overloaded" functions into a
//!       single page with all the variants. I've tried to correct this by hand.
//!       If you find any remaining bad links [please file a
//!       bug](https://github.com/Lokathor/gles30/issues) about it.
//!
//! ```ignore
//! use gles30::*;
//!
//! load_gles_with(|ptr| SDL_GL_GetProcAddress(ptr));
//!
//! glClearColor(0.5, 0.5, 0.5, 1.0);
//! ```
//!
//! This crate _only_ loads GLES 3.0, not any other version of GLES.
//!
//! ## Features
//!
//! There's some features you can turn on for debugging assistance.
//!
//! * `debug_trace_messages`: If enabled, immediately _before_ a call to a GL
//!   function it'll print the name of the function about to be called. I expect
//!   that you'll keep this off most of the time, but if you're getting
//!   segfaults it makes it a snap to tell what you called before the segfault
//!   happened.
//! * `debug_error_checks`: If enabled, immediately _after_ all calls to GL
//!   there's an additional call to `glGetError`. If the error value is
//!   something other than `GL_NO_ERROR` you'll get an error message printed
//!   showing the name of the function you called, the arguments you passed, and
//!   the error code.
//!
//! Unfortunately, the `glDebugMessageCallback` function didn't become part of
//! GLES until 3.2, so we got some silly macros.
//!
//! ## `no_std` Support
//!
//! This library is `no_std` friendly. It's just bindings and loader callbacks
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
