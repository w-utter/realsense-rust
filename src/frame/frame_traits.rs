//! Trait for describing basic frame operations

use super::pixel::PixelKind;
use crate::{common::*, stream};
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

pub trait VideoFrameUnsafeEx<'a> {
    fn get_unchecked(&self, col: usize, row: usize) -> PixelKind<'a>;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn get_raw_size(&self) -> usize;

    fn get_raw(&'a self) -> &'a std::os::raw::c_void;
}

pub trait VideoFrameEx<'a> {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn profile(&'a self) -> &'a stream::Profile;

    fn get(&self, col: usize, row: usize) -> Option<PixelKind<'a>>;
}
