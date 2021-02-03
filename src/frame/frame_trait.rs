//! Trait for describing basic frame operations

use crate::common::*;

pub enum ConstructionError {
    CouldNotGetWidth(String),
    CouldNotGetHeight(String),
    CouldNotGetStride(String),
    CouldNotGetBitsPerPixel(String),
    CouldNotGetDataSize(String),
    CouldNotGetData(String),
}

pub trait Frame
where
    Self: Sized,
{
    fn new(frame_ptr: NonNull<sys::rs2_frame>) -> std::result::Result<Self, ConstructionError>;
}
