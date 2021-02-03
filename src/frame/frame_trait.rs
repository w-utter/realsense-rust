//! Trait for describing basic frame operations

use crate::common::*;

pub enum FrameConstructionError {
    CouldNotGetWidth(String),
    CouldNotGetHeight(String),
    CouldNotGetStride(String),
    CouldNotGetBitsPerPixel(String),
    CouldNotGetFrameStreamProfile(String),
    CouldNotGetDataSize(String),
    CouldNotGetData(String),
}

pub struct PixelIndexOutOfBoundsError();

pub trait Frame
where
    Self: Sized,
{
    fn new(frame_ptr: NonNull<sys::rs2_frame>)
        -> std::result::Result<Self, FrameConstructionError>;
}
