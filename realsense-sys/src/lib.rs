// Allow all warnings here -- Bindgen generates this file, we really don't care about individual
// warnings since we can't really do much about them, we'd have to fix bindgen upstream or
// librealsense2 itself.
#![allow(warnings)]
#![allow(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

include!("../bindings/bindings.rs");
