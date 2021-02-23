//! The crate provides high level API to librealsense2.
//!
//! It features asynchronous API and integration with [image](https://crates.io/crates/image) and [nalgebra](https://crates.io/crates/nalgebra).
//!
//! # Cargo Features
//!
//! The crate enables **with-nalgebra** and **with-image** features by default.
//!
//! - **with-nalgebra** (default): Enable [nalgebra](https://github.com/rustsim/nalgebra) support.
//! - **with-image** (default): Enable [image](https://github.com/image-rs/image) support.
//! - **buildtime-bindgen**: Generate Rust bindings during build time.
//! - **device-test**: Enable tests that requires connections to RealSense devices.
//!
//! # Get Started
//!
//! You can start by [Pipeline](Pipeline). This is the minimal example to capture color and depth images.
//!
//! ```no_run
//! use anyhow::Result;
//! use realsense_rust::{Config, Rs2Format, Pipeline, Rs2StreamKind};
//!
//! fn main() -> anyhow::Result<()> {
//!     let pipeline = Pipeline::new()?;
//!     let config = Config::new()?
//!         .enable_stream(Rs2StreamKind::Depth, 0, 640, 0, Rs2Format::Z16, 30)?
//!         .enable_stream(Rs2StreamKind::Color, 0, 640, 0, Rs2Format::Rgb8, 30)?;
//!     let mut pipeline = pipeline.start(&config)?;
//!
//!     let frames = pipeline.wait(None)?.unwrap();
//!     let color_frame = frames.color_frame()?.unwrap();
//!     let depth_frame = frames.depth_frame()?.unwrap();
//!
//!     Ok(())
//! }
//! ```
//!
//! # Architecture & Guiding Principles
//!
//! At a high-level, the library tries to map the fundamental abstractions in the librealsense2
//! C-API, but to do so in a type-safe, Rust-native way. What this means is that we try to avoid
//! unsafe methods where possible, and attempt to wrap some of the low-level abstractions from
//! realsense-sys in types that can encapsulate the underlying unsafe pointers and values.
//!
//! Where possible, we try to follow a few guiding principles with regards to the library design.
//! These are:
//!
//! 1. Prefer Rust-native types to types used through the FFI
//! 2. Make lifetimes obvious for all data through the system.
//! 3. Make invalid states unrepresentable (where possible).
//! 4. Make error cases explicit and differentiable.
//!
//! We attempt to explain each of these below.
//!
//! ## Prefer Rust-native types to types used through the FFI
//!
//! In general, many of the people writing Rust code are doing so to maintain safety and
//! efficiency. It is unnatural to have to think about types and semantics from other languages
//! while writing code in Rust. The foreign function interface (FFI) unfortunately is one situation
//! in which we must take care to do so.
//!
//! Realsense-rust aims to provide safe abstractions so that you can access functionality from
//! librealsense2, but write your code in idiomatic Rust. What this means is that we avoid using or
//! returning abstractions directly from the `bindgen` output generated by realsense-sys. There are
//! several strategies that we take to try and avoid exposing lower level abstractions.
//!
//! ### Transform `std::os::raw` types to Rust
//!
//! The first of which is that we wrap all C-style enums in Rust enums. This is done using the
//! [`num_derive`](https://crates.io/crates/num-derive) and
//! [`num_traits`](https://crates.io/crates/num-traits) crates. Since all of the C-style enums in
//! librealsense2 are bound as `u32` values with `bindgen`, we do a fairly small transformation
//! back and forth between the two representations. C-style "strings" (read: `const char*`) are
//! likewise converted into the Rust native
//! [`std::ffi::CStr`](std::ffi::CStr).
//!
//! ### Provide concrete types rather than "extensible" pointers
//!
//! The second way in which we try to express idiomatic Rust requires some understanding of the
//! underlying librealsense2 library. In librealsense2, many types in the C-API are represented as
//! pointers to opaque structs. However, many of the "types" are obscured, as these pointers have
//! the ability to be "extended" to support certain interfaces. This is masking some of the
//! inheritance based structure from the underlying C++ code that is the basis of librealsense2. In
//! any case, extensions to frames, sensors, filters, etc. are all made possible through the
//! `rs2_extension` enumeration (See [Rs2Extension] for how we handle this on the Rust
//! side). What's unfortunate here is that all these extensions are contained within a single
//! enumeration, as opposed to having a separate enumeration for frames, sensors, etc. This is
//! awkward to use when actually programming, as the natural way to know exactly what type you have
//! is not only to know what pointer type you received, but to ask the API if you can extend that
//! pointer to a (growing) list of different extensions, many of which make no sense (e.g. you can
//! never extend a frame to `rs2_extension_RS2_EXTENSION_ZERO_ORDER_FILTER`).
//!
//! Instead, we try to preemptively understand what concrete "type" of data you have from the
//! pointers upfront, and make that clear in the Rust API by providing a concrete type back. This
//! is why there are multiple structs for `VideoFrame`, `PoseFrame`, `MotionFrame`, etc. even
//! though they all store a `*mut rs2_frame` to interface with the FFI's frame functionality
//! underneath the hood. So in short: we do some preemptive checking of the types where necessary
//! and produce concrete Rust types to represent them, even if in librealsense2 these types would
//! be represented by the same pointer or opaque struct.
//!
//! ### Use vectors or native Rust containers over librealsense2 abstractions
//!
//! The third way in which types are kept Rusty is that for many types that would be expressed by a
//! "list" (e.g. `rs2_device_list`, `rs2_stream_profile_list`) in the librealsense2 C-API are
//! provided back as standard Rust vectors (and we take care of ownership / memory safety under the
//! hood). This allows the use of all the things that vectors provide, rather than making our own
//! managed list types.
//!
//! ## Errors
//!
//! The last thing we do with regards to keeping types Rusty concerns how we handle error types.
//! More on this is written below.
//!
//! ### Make lifetimes obvious for all data through the system
//!
//! The librealsense2 C-API does not always do the best job at explaining object lifetimes. It is
//! important to understand that librealsense2 is first and foremost implemented in C++, and the
//! Rust wrapper we provide here is build on a C-wrapper around that C++ API. While the underlying
//! C++ library takes advantage of C++11 abstractions such as `shared_ptr` or `unique_ptr` to
//! declare ownership semantics, the C wrapper around it cannot express these types. So instead it
//! uses raw pointers and attempts to use documentation to help close the gap between what pointers
//! are managed vs. which are not.
//!
//! For users of realsense-rust, you should not have to think about this. The documentation for the
//! C-API does not always describe what the exact lifetimes of the underlying types are. For this,
//! the authors of this crate had to on occasion read the librealsense2 source to understand some
//! of the ownership semantics. Where possible, we try to guarantee the lifetimes of data
//! throughout the system, either by implementing the Drop trait directly, managing creation and
//! deletion of pointers explicitly, or limiting the number of ways in which objects can be
//! constructed (so as to prevent scenarios where lifetimes are sometimes managed or sometimes
//! not).
//!
//! In most scenarios we aim to avoid making lifetimes explicit, but there are instances where that
//! is not possible. However, one should expect that anything obtained from the high-level API is
//! safe to retain unless otherwise noted. Please [submit a bug
//! report](https://gitlab.com/tangram-vision-oss/realsense-rust/-/issues) if you've found some
//! scenario in which an object you retained is holding onto invalid or otherwise deleted pointers.
//!
//! ### Make invalid states unrepresentable
//!
//! The main way we aim to do this is by understanding the lifetimes of the low-level pointers that
//! the realsense-sys library returns (described above). However, many of the types in the system
//! (especially frame types) will preemptively cache some data available through other interfaces
//! ahead of time. A key example might be the `ImageFrame` struct, which caches width, height,
//! stride, and the stream profile associated with the frame on construction. The reason this is
//! done is because error handling with the C-API is awkward.
//!
//! Why is it awkward? Well, recall that librealsense2 is actually implemented in terms of C++. The
//! library utilizes C++ exceptions to signal errors. The C-API cannot do that as C has no way of
//! expressing an exception (language doesn't support it). There are relatively few guarantees you
//! can make about code that can signal exceptions, and so the vast majority of the C-API gets
//! around this by capturing exceptions at the levels they can occur, and then wrapping the
//! exception information in a pointer to an opaque type (`*mut rs2_error`). Of course, since
//! exceptions can occur in so many places, almost every C-API function takes a `*mut *mut
//! rs2_error`. Most of these checks are null-checks on the input pointer type, but not all.
//!
//! On the Rust side, we catch / check these `*mut rs2_error` types internally, and then signal
//! this back to the user by returning a `Result` value of some kind. We cache some of the
//! metadata or small fields in our Rust structs so that we can reduce the amount of `Result`
//! checks that need to be done by the user, and likewise to keep relevant data cached as long as
//! possible.
//!
//! One example where we need to keep "relevant data cached as long as possible" is in the frame
//! types, specifically image frames. In order to be able to interpret the underlying data, we need
//! to get the stream format, which is obtained by the stream profile. If we use the natural API to
//! get this (i.e. `rs2_get_frame_stream_profile`), we can get the stream profile. However, this
//! stream profile is managed by the device, so if the device is disconnected before the frame is
//! processed, this pointer is no longer valid. We cache the stream format when constructing our
//! own `StreamProfile` type on the Rust side so that even if the device is disconnected mid-way
//! through streaming, you can still interpret the format and pixel data of your frame type.
//!
//! Since the frame stream profile is not owned by the frame (shared ownership managed through a
//! device), we wouldn't otherwise be able to guarantee that the stream profile is available if the
//! device the frame was streamed from was disconnected. This is a failure of the C-API. However,
//! we manage to make invalid states unrepresentable since we do own the frame, and we can still
//! access that data since we cache some of the small metadata (in this case, the format) on the
//! Rust side.
//!
//! We do not typically copy / cache the underlying data from frames, as for e.g. a 720p image that
//! involves a lot of copying and allocation, which is expensive and detrimental to users who want
//! to build applications on top of realsense-rust while not sacrificing the speed or efficiency of
//! the C or C++ librealsense2 APIs.
//!
//! ### Make error cases explicit and differentiable
//!
//! In cases where you might get an error from the low-level API you'll find that the high-level
//! Rust wrapper provided by realsense-rust will return a `Result` of some kind. We do not shy away
//! from making new types to express different classes of errors. However, given that the error
//! information provided by the librealsense2 API is somewhat limited, you'll find most of our
//! error types are of the form:
//!
//! ```no_run
//! pub enum SomeError {
//!     CouldNotXXX(String),
//! }
//! ```
//!
//! The enum field names should inform you what specific part of the function failed (if there are
//! multiple parts), and the internal `String` is the exception message from librealsense2. If you
//! find yourself hitting the same message often, this is a bug, and we would love if you [submitted
//! a bug report](https://gitlab.com/tangram-vision-oss/realsense-rust/-/issues).
//!

pub mod base;
mod common;
pub mod config;
pub mod context;
pub mod device;
pub mod device_hub;
pub mod error;
pub mod frame;
pub mod kind;
pub mod pipeline;
pub mod sensor;
pub mod stream_profile;

// pub mod frame_queue;
// pub mod processing_block;
// pub mod processing_block_kind;
// pub mod processing_block_list;

/// The module collects common used traits from this crate.
pub mod prelude {
    pub use crate::frame::{DepthFrameEx, DisparityFrameEx, FrameEx, VideoFrameEx};
    pub use crate::kind::Extension;
}

// pub use frame_queue::FrameQueue;
// pub use processing_block::{
//     Align, AnyProcessingBlock, Colorizer, DecimationFilter, DisparityFilter, HoleFillingFilter,
//     HuffmanDepthDecompress, PointCloud, ProcessingBlock, RatesPrinter, SpatialFilter, Syncer,
//     TemporalFilter, ThresholdFilter, UnitsTransform, YuyDecoder, ZeroOrderFilter,
// };
// pub use processing_block_list::{ProcessingBlockList, ProcessingBlockListIntoIter};
