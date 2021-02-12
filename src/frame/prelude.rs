//! Trait for describing frame operations.

use super::pixel::PixelKind;
use crate::{
    common::*,
    kind::{Rs2FrameMetadata, Rs2TimestampDomain},
    sensor::Sensor,
    stream::StreamProfile,
};
use anyhow::Result;
use realsense_sys as sys;
use thiserror::Error;

pub const BITS_PER_BYTE: i32 = 8;

#[derive(Error, Debug)]
pub enum FrameConstructionError {
    #[error("Could not get frame width. Reason: {0}")]
    CouldNotGetWidth(String),
    #[error("Could not get frame height. Reason: {0}")]
    CouldNotGetHeight(String),
    #[error("Could not get stride. Reason: {0}")]
    CouldNotGetStride(String),
    #[error("Could not get bits-per-pixel. Reason: {0}")]
    CouldNotGetBitsPerPixel(String),
    #[error("Could not get timestamp. Reason: {0}")]
    CouldNotGetTimestamp(String),
    #[error("Could not get timestamp domain. Reason: {0}")]
    CouldNotGetTimestampDomain(String),
    #[error("Could not get frame stream profile. Reason: {0}")]
    CouldNotGetFrameStreamProfile(String),
    #[error("Could not get data size (in bytes). Reason: {0}")]
    CouldNotGetDataSize(String),
    #[error("Could not get pointer to frame data. Reason: {0}")]
    CouldNotGetData(String),
    #[error("Could not get number of points: {0}")]
    CouldNotGetPointCount(String),
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

    fn timestamp(&self) -> f64;

    fn timestamp_domain(&self) -> Rs2TimestampDomain;

    fn metadata(&self, metadata_kind: Rs2FrameMetadata) -> Option<std::os::raw::c_longlong>;

    fn supports_metadata(&self, metadata_kind: Rs2FrameMetadata) -> bool;

    unsafe fn get_owned_frame_ptr(self) -> NonNull<sys::rs2_frame>;
}

pub trait DepthFrameEx {
    fn distance(&self, col: usize, row: usize) -> Result<f32, DepthError>;

    fn depth_units(&self) -> Result<f32>;
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

pub trait PointsFrameEx<'a> {
    /// Gets vertices of point cloud.
    fn vertices(&'a self) -> &'a [sys::rs2_vertex];
    /// Gets texture coordinates of each point of point cloud.
    fn texture_coordinates(&'a self) -> &'a [[f32; 2]];
    /// Gets number of points in frame.
    fn points_count(&self) -> usize;
}
