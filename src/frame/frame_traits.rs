//! Trait for describing basic frame operations

use crate::common::*;
use std::result::Result;

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

pub trait VideoFrameUnsafeEx {
    type Output: Sized;

    fn at_no_bounds_check(&self, col: usize, row: usize) -> Self::Output;
}

pub trait VideoFrameEx: VideoFrameUnsafeEx {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn at(&self, col: usize, row: usize) -> Result<Self::Output, PixelIndexOutOfBoundsError>;
}
