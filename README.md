[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![min-rust-1.34](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/gles30.svg)](https://crates.io/crates/gles30)
[![docs.rs](https://docs.rs/gles30/badge.svg)](https://docs.rs/gles30/)

![Unsafe-101-Percent](https://img.shields.io/badge/Unsafety-101%25-red.svg)

# gles30
Bindings to OpenGL ES 3.0

## Stability

The `gles30` crate presents OpenGL ES 3.0 bindings for Rust, as described by [gl.xml](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml).

As often as `gl.xml` updates I will also attempt to issue updates for this crate.

Because `gles30` follows the current content of `gl.xml` as closely as possible,
it's *possible* (though highly unlikely) that there could be a `gl.xml` update
that would somehow cause a breaking change. This is most likely to occur if an
argument's type changes between signed and unsigned, which is not a big
difference in C but it would cause a type mismatch in Rust (you'd need to add an
`as _` to make it cast the value). In this case, the break is considered a
"required bugfix", and you just have to update your code. Sorry.
