//! Type for representing a video frame taken from a color or IR camera.

use super::prelude::{
    CouldNotGetFrameSensorError, DepthError, DepthFrameEx, DisparityError, DisparityFrameEx,
    FrameConstructionError, FrameEx, VideoFrameEx, VideoFrameUnsafeEx, BITS_PER_BYTE,
};
use super::{
    iter::ImageIter,
    pixel::{get_pixel, PixelKind},
};
use crate::{
    check_rs2_error,
    common::*,
    kind::{Extension, Rs2Extension, Rs2FrameMetadata, Rs2Option, Rs2TimestampDomain},
    sensor::Sensor,
    stream::StreamProfile,
};
use anyhow::Result;
use num_traits::ToPrimitive;
use std::convert::TryFrom;

pub struct Depth;
pub struct Disparity;
pub struct Video;

pub struct ImageFrame<'a, Kind> {
    frame_ptr: NonNull<sys::rs2_frame>,
    width: usize,
    height: usize,
    stride: usize,
    bits_per_pixel: usize,
    timestamp: f64,
    timestamp_domain: Rs2TimestampDomain,
    frame_stream_profile: StreamProfile<'a>,
    data_size_in_bytes: usize,
    data: &'a std::os::raw::c_void,
    should_drop: bool,
    _phantom: PhantomData<Kind>,
}

pub type DepthFrame<'a> = ImageFrame<'a, Depth>;
pub type DisparityFrame<'a> = ImageFrame<'a, Disparity>;
pub type VideoFrame<'a> = ImageFrame<'a, Video>;

impl<'a, K> ImageFrame<'a, K> {
    pub fn iter(&'a self) -> ImageIter<'a, ImageFrame<'a, K>> {
        ImageIter {
            frame: self,
            column: 0,
            row: 0,
        }
    }
}

impl<'a, K> Drop for ImageFrame<'a, K> {
    fn drop(&mut self) {
        unsafe {
            if self.should_drop {
                sys::rs2_release_frame(self.frame_ptr.as_ptr());
            }
        }
    }
}

unsafe impl<'a, K> Send for ImageFrame<'a, K> {}

impl<'a, K> TryFrom<NonNull<sys::rs2_frame>> for ImageFrame<'a, K> {
    type Error = anyhow::Error;

    fn try_from(frame_ptr: NonNull<sys::rs2_frame>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let width = sys::rs2_get_frame_width(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetWidth)?;

            let height = sys::rs2_get_frame_height(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetHeight)?;

            let bits_per_pixel = sys::rs2_get_frame_bits_per_pixel(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetBitsPerPixel)?;

            let stride = sys::rs2_get_frame_stride_in_bytes(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetStride)?;

            let timestamp = sys::rs2_get_frame_timestamp(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetTimestamp)?;

            let timestamp_domain =
                sys::rs2_get_frame_timestamp_domain(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetTimestampDomain)?;

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = StreamProfile::try_from(nonnull_profile_ptr)?;

            let size = sys::rs2_get_frame_data_size(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetDataSize)?;

            debug_assert_eq!(size, width * height * bits_per_pixel / BITS_PER_BYTE);

            let ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            Ok(ImageFrame {
                frame_ptr,
                width: width as usize,
                height: height as usize,
                stride: stride as usize,
                bits_per_pixel: bits_per_pixel as usize,
                timestamp,
                timestamp_domain: Rs2TimestampDomain::from_u32(timestamp_domain).unwrap(),
                frame_stream_profile: profile,
                data_size_in_bytes: size as usize,
                data: ptr.as_ref().unwrap(),
                should_drop: true,
                _phantom: PhantomData::<K> {},
            })
        }
    }
}

impl<'a> Extension for DepthFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::DepthFrame
    }
}

impl<'a> Extension for DisparityFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::DisparityFrame
    }
}

impl<'a> Extension for VideoFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::VideoFrame
    }
}

impl<'a, T> FrameEx<'a> for ImageFrame<'a, T> {
    fn profile(&'a self) -> &'a StreamProfile<'a> {
        &self.frame_stream_profile
    }

    fn sensor(&self) -> Result<Sensor> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let sensor_ptr = sys::rs2_get_frame_sensor(self.frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotGetFrameSensorError)?;

            Ok(Sensor::try_from(NonNull::new(sensor_ptr).unwrap())?)
        }
    }

    fn timestamp(&self) -> f64 {
        self.timestamp
    }

    fn timestamp_domain(&self) -> Rs2TimestampDomain {
        self.timestamp_domain
    }

    fn metadata(&self, metadata_kind: Rs2FrameMetadata) -> Option<std::os::raw::c_longlong> {
        unimplemented!();
    }

    fn supports_metadata(&self, metadata_kind: Rs2FrameMetadata) -> bool {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let supports_metadata = sys::rs2_supports_frame_metadata(
                self.frame_ptr.as_ptr(),
                metadata_kind.to_u32().unwrap(),
                &mut err,
            );

            err.as_ref().is_none() && supports_metadata != 0
        }
    }

    unsafe fn get_owned_frame_ptr(mut self) -> NonNull<sys::rs2_frame> {
        self.should_drop = false;

        self.frame_ptr
    }
}

impl<'a> DepthFrameEx for DepthFrame<'a> {
    fn distance(&self, col: usize, row: usize) -> Result<f32, DepthError> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let distance = sys::rs2_depth_frame_get_distance(
                self.frame_ptr.as_ptr(),
                col as c_int,
                row as c_int,
                &mut err,
            );
            check_rs2_error!(err, DepthError::CouldNotGetDistance)?;
            Ok(distance)
        }
    }

    fn depth_units(&self) -> Result<f32> {
        let sensor = self.sensor()?;
        let depth_units = sensor.get_option(Rs2Option::DepthUnits)?;
        Ok(depth_units)
    }
}

impl<'a> DepthFrameEx for DisparityFrame<'a> {
    fn distance(&self, col: usize, row: usize) -> Result<f32, DepthError> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let distance = sys::rs2_depth_frame_get_distance(
                self.frame_ptr.as_ptr(),
                col as c_int,
                row as c_int,
                &mut err,
            );
            check_rs2_error!(err, DepthError::CouldNotGetDistance)?;
            Ok(distance)
        }
    }

    fn depth_units(&self) -> Result<f32> {
        let sensor = self.sensor()?;
        let depth_units = sensor.get_option(Rs2Option::DepthUnits)?;
        Ok(depth_units)
    }
}

impl<'a> DisparityFrameEx for DisparityFrame<'a> {
    fn baseline(&self) -> Result<f32, DisparityError> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let baseline =
                sys::rs2_depth_stereo_frame_get_baseline(self.frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, DisparityError)?;
            Ok(baseline)
        }
    }
}

impl<'a, K> VideoFrameUnsafeEx<'a> for ImageFrame<'a, K> {
    fn get_unchecked(&'a self, col: usize, row: usize) -> PixelKind<'a> {
        unsafe {
            get_pixel(
                self.frame_stream_profile.format(),
                self.data_size_in_bytes,
                self.data,
                self.stride,
                col,
                row,
            )
        }
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn bits_per_pixel(&self) -> usize {
        self.bits_per_pixel
    }

    fn get_raw_size(&self) -> usize {
        self.data_size_in_bytes
    }

    fn get_raw(&'a self) -> &'a std::os::raw::c_void {
        self.data
    }
}

impl<'a, K> VideoFrameEx<'a> for ImageFrame<'a, K> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&'a self, col: usize, row: usize) -> Option<PixelKind<'a>> {
        if col >= self.width || row >= self.height {
            None
        } else {
            Some(self.get_unchecked(col, row))
        }
    }
}

impl<'a, K> IntoIterator for &'a ImageFrame<'a, K> {
    type Item = <ImageIter<'a, ImageFrame<'a, K>> as Iterator>::Item;
    type IntoIter = ImageIter<'a, ImageFrame<'a, K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
