//! Trait for describing frame operations.

use super::pixel::PixelKind;
use crate::{common::*, sensor::Sensor, stream::StreamProfile};
use anyhow::Result;
use realsense_sys as sys;
use thiserror::Error;

pub const BITS_PER_BYTE: i32 = 8;

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

#[derive(Error, Debug)]
pub enum DepthError {
    #[error("Could not get distance. Reason: {0}")]
    CouldNotGetDistance(String),
    #[error("Could not get depth units. Reason: {0}")]
    CouldNotGetDepthUnits(String),
}

#[derive(Error, Debug)]
#[error("Could not get baseline. Reason: {0}")]
pub struct DisparityError(pub String);

#[derive(Error, Debug)]
#[error("Could not get frame sensor. Reason: {0}")]
pub struct CouldNotGetFrameSensorError(pub String);

pub trait FrameEx<'a> {
    fn profile(&'a self) -> &'a StreamProfile<'a>;

    fn sensor(&self) -> Result<Sensor>;

    unsafe fn get_owned_frame_ptr(self) -> NonNull<sys::rs2_frame>;
}

pub trait DepthFrameEx {
    fn distance(&self, col: usize, row: usize) -> Result<f32, DepthError>;

    fn depth_units(&self) -> Result<f32, DepthError>;
}

pub trait DisparityFrameEx {
    fn baseline(&self) -> Result<f32, DisparityError>;
}

pub trait VideoFrameUnsafeEx<'a> {
    fn get_unchecked(&'a self, col: usize, row: usize) -> PixelKind<'a>;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn get_raw_size(&self) -> usize;

    fn get_raw(&'a self) -> &'a std::os::raw::c_void;
}

pub trait VideoFrameEx<'a> {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get(&'a self, col: usize, row: usize) -> Option<PixelKind<'a>>;
}

pub trait MotionFrameEx<'a> {
    fn motion(&self) -> &[f32; 3];
}
