//! Module for specifying safe abstractions
//!
//! # What is safe vs. unsafe?
//!
//! The distinction between "safe" and "unsafe" abstractions is more or less a distinction of
//! which side of the FFI data lives on. This isn't exactly a clear distinction, and without some
//! knowledge of the underlying `librealsense2` library and the abstractions made by both
//! `realsense-sys` and `realsense-rust`, so a brief description is provided.
//!
//! You may notice that many of the return types for methods in the crate are some form of `Result`
//! type. This is defined in the `error` sub-module. The possible errors this can return are
//! somewhat limited, but generally flow from the underlying `rs2_error` type defined in the
//! `librealsense2` library.
//!
//! `librealsense2` is first and foremost a C++ library, not a C library. Intel makes an effort to
//! encapsulate this in portable C. However, being a C++ library, we have to understand how
//! `librealsense2` actually does error handling natively. Namely, it uses C++ exceptions.
//!
//! Of course, to generate portable C, you can't actually use exceptions. The runtime doesn't have
//! any mechanism for it. So most of the exception information is wrapped into a C type:
//! `rs2_error`. This type bundles the function name, arguments, and error message (i.e.
//! `exception.what()`) into a single opaque struct type. The C library returns this only as a
//! pointer. If you look at `<rs_types.h>` in the original `librealsense2` include directory, you
//! can see the general API for this error type.
//!
//! Unfortunately, just because the error is bundled as a C type, doesn't mean that the C++
//! semantics of exceptions go away. In particular, for most of the system an exception can occur
//! at any time by any function call. Regrettably, this means that an error can occur at almost any
//! entry point to the library. In fact, if you look at most of the function declarations in
//! `<rs2.h>`, you'll actually find that the C wrapper around the C++ code takes in an
//! `rs2_error**` parameter for every function. This is more or less a secondary output parameter,
//! that you need to check for every call to the C wrapper.
//!
//! ## What is `realsense-sys` doing?
//!
//! `realsense-sys` is more or less a standard
//! [bindgen](https://docs.rs/bindgen/0.57.0/bindgen/index.html) wrapper around the C headers. For
//! brevity, we won't go into how `bindgen` works, but rest assured that it's not doing anything
//! fancy here: merely making it possible to call these bare C wrapper functions from
//! `librealsense2`.
//!
//! Because of the way the C API is structured, we have to check these `rs2_error` types before we
//! can guarantee that any pointer, type, or structs we touch across the FFI boundary are safe (in
//! the Rust sense).
//!
//! ## What is `realsense-rust` doing?
//!
//! Error handling more or less works by checking that the returned pointer that we get from any of
//! the C APIs is null (i.e. no error). If it is not null, there are a few categories of errors we
//! can transform that into, which is more or less the distinction we see between the different
//! types of errors that can be returned by the `Result<T>` in most of the `realsense-rust` API.
//!
//! Side note: these error "types" or categories are mostly guessed at by parsing the error strings
//! that we can get from `rs2_error`. This is somewhat fragile. Alas, not the biggest concern or
//! immediate priority.
//!
//! # Why is returning `Result<T>` everywhere so bad?
//!
//! Well, it's more a matter of performance. Consider frame types, such as `Frame<Depth>`. The
//! signature for the `distance` method is:
//!
//! ```text
//! fn distance(&self, x: usize, y: usize) -> Result<f32>;
//! ```
//!
//! In a normal pipeline using this data, we might decide to get the distance or depth at every
//! pixel, and do something with that depth. This could be running depth through a shader
//! (rendering to screen), rectifying the depth to convert it to a point cloud, and then tracking
//! based off that point cloud in some SLAM system. Rendering and rectification might both read
//! into the same frame, but at no point do we need to worry about modifying the underlying data
//! (the API doesn't allow for this, regardless, so nothing lost there).
//!
//! So for both rendering and rectification, we need to loop through every pixel in the image and
//! get the `distance` at that pixel location. Cool, except remember the return type:
//! `Result<f32>`. Notice that means that for every pixel, we now have two additional operations:
//!
//! 1. The inner type is `f32`, not `&f32`. So we aren't reading into some pointer location, the
//!    `Result` type is made by calling `Ok(some_f32)`, which will typically result in a cheap copy
//!    of the underlying float. By extension, every time you read through every pixel in order you
//!    end up doing a full copy of the whole frame.
//! 2. We have to `unwrap()` or deal with the `Result` type somehow.
//!
//! Both of these mean that we can't use the inner frame data (in this case, the distance / depth)
//! in tight loops. We also eat the cost of a full copy every time we loop over it. Even ignoring
//! Rust, the C API forces a pretty bad model that brings some tricky performance behaviour to the
//! table.
//!
//! The same problem is more or less true for every other type as well. In some cases, it's less of
//! an issue because those parts of the API may not be performance critical (e.g. `Config` is
//! typically not a concern here). However, this is a problem that needs to be addressed at the
//! frame level.
//!
//! # So what is the "safe" module, anyways?
//!
//! This module is more or less here to provide types that represent a full copy of the underlying
//! data, expressed in purely standard Rust types (i.e. no underlying C / FFI pointers). There is
//! not a terrible amount we can do about the C API, however by doing a complete copy of the
//! underlying data across the FFI boundary, we can provide types that have sane semantics with
//! regards to performance and how consumers of the data expect to use these types.
//!
