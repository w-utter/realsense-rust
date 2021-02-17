//! Contains commonly used enumerations and configuration types used across the crate.
//!
//! The `Kind` module holds a variety of types useful across realsense-rust APIs as transient
//! values. The vast majority of types in this module are enums, which map to C-style enums in the
//! lower-level `realsense-sys` bindings.
//!
//! # Architecture
//!
//! The low-level bindings for librealsense2 in `realsense-sys` represent the C-style enums as
//! `u32` constants. They are wrapped / transformed into fully qualified types here so as to
//! increase type safety across the API.
//!
//! All of these "wrapper" enums in Rust implement the [`ToPrimitive`](num_traits::ToPrimitive) and
//! [`FromPrimitive`](num_traits::FromPrimitive) traits from the `num_traits` crate. If you need to
//! access the original enum value, you can do so with the following code:
//!
//! ```rust
//! use num_traits::ToPrimitive;
//! use crate::kind::Rs2Extension;
//!
//! fn main() {
//!     let ext = Rs2Extension::ColorSensor;
//!     println!("The extension is: {}", ext.to_u32().unwrap());
//! }
//! ```
//!
//! In practice, most of the time you shouldn't need to wrap or unwrap `u32` values, and the API
//! should never spit one out at you.
//!

mod camera_info;
mod color_scheme;
mod extension;
mod format;
mod frame_metadata;
mod hole_filling;
mod option;
mod persistence_control;
mod prelude;
mod stream_kind;
mod timestamp_domain;

pub use camera_info::Rs2CameraInfo;
pub use color_scheme::ColorScheme;
pub use extension::{
    Rs2Extension, DEVICE_EXTENSIONS, FILTER_EXTENSIONS, FRAME_EXTENSIONS, MISC_EXTENSIONS,
    PROFILE_EXTENSIONS, SENSOR_EXTENSIONS,
};
pub use format::Rs2Format;
pub use frame_metadata::Rs2FrameMetadata;
pub use hole_filling::HoleFillingMode;
pub use option::{OptionSetError, Rs2Option, Rs2OptionRange};
pub use persistence_control::PersistenceControl;
pub use prelude::*;
pub use stream_kind::Rs2StreamKind;
pub use timestamp_domain::Rs2TimestampDomain;
