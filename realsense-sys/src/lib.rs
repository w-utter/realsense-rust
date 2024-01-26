#![doc = include_str!("../README.md")]
// Allow all warnings here -- Bindgen generates this file, we really don't care about individual
// warnings since we can't really do much about them, we'd have to fix bindgen upstream or
// librealsense2 itself.
#![allow(warnings)]
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
include!("../bindings/bindings.rs");
