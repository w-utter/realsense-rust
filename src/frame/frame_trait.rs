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

pub trait VideoFrameEx {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;
}
