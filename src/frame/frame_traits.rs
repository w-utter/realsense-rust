//! Trait for describing basic frame operations

use crate::common::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrameConstructionError {
    #[error("Could not get frame width: {0}")]
    CouldNotGetWidth(String),
    #[error("Could not get frame height: {0}")]
    CouldNotGetHeight(String),
    #[error("Could not get stride: {0}")]
    CouldNotGetStride(String),
    #[error("Could not get bits-per-pixel: {0}")]
    CouldNotGetBitsPerPixel(String),
    #[error("Could not get frame stream profile: {0}")]
    CouldNotGetFrameStreamProfile(String),
    #[error("Could not get data size (in bytes): {0}")]
    CouldNotGetDataSize(String),
    #[error("Could not get pointer to frame data: {0}")]
    CouldNotGetData(String),
}

pub const BITS_PER_BYTE: i32 = 8;

pub trait VideoFrameUnsafeEx {
    type Output: Sized;

    fn get_unchecked(&self, col: usize, row: usize) -> Self::Output;
}

pub trait VideoFrameEx: VideoFrameUnsafeEx {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn get(&self, col: usize, row: usize) -> Option<Self::Output>;
}
