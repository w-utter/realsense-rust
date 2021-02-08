//! Type for representing a video frame taken from a color or IR camera.

use super::frame_traits::{
    FrameConstructionError, VideoFrameEx, VideoFrameUnsafeEx, BITS_PER_BYTE,
};
use super::{
    iter::ImageIter,
    kind::Kind,
    pixel::{get_pixel, PixelKind},
};
use crate::{check_rs2_error, common::*, stream};
use std::result::Result;

pub struct VideoFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    width: usize,
    height: usize,
    stride: usize,
    bits_per_pixel: usize,
    frame_stream_profile: stream::Profile,
    data_size_in_bytes: usize,
    data: &'a std::os::raw::c_void,
}

impl<'a> VideoFrame<'a> {
    pub fn iter(&'a self) -> ImageIter<'a, VideoFrame<'a>> {
        ImageIter {
            frame: self,
            column: 0,
            row: 0,
        }
    }
}

impl<'a> Drop for VideoFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_release_frame(self.frame_ptr.as_ptr());
        }
    }
}

impl<'a> Kind for VideoFrame<'a> {
    fn extension() -> sys::rs2_extension {
        sys::rs2_extension_RS2_EXTENSION_VIDEO_FRAME
    }
}

impl<'a> std::convert::TryFrom<NonNull<sys::rs2_frame>> for VideoFrame<'a> {
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

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = stream::Profile::new(nonnull_profile_ptr)?;

            let size = sys::rs2_get_frame_data_size(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetDataSize)?;

            debug_assert_eq!(size, width * height * bits_per_pixel / BITS_PER_BYTE);

            let ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            Ok(VideoFrame {
                frame_ptr,
                width: width as usize,
                height: height as usize,
                stride: stride as usize,
                bits_per_pixel: bits_per_pixel as usize,
                frame_stream_profile: profile,
                data_size_in_bytes: size as usize,
                data: ptr.as_ref().unwrap(),
            })
        }
    }
}

impl<'a> VideoFrameUnsafeEx<'a> for VideoFrame<'a> {
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

impl<'a> VideoFrameEx<'a> for VideoFrame<'a> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn profile(&'a self) -> &'a stream::Profile {
        &self.frame_stream_profile
    }

    fn get(&'a self, col: usize, row: usize) -> Option<PixelKind<'a>> {
        if col >= self.width || row >= self.height {
            None
        } else {
            Some(self.get_unchecked(col, row))
        }
    }
}

impl<'a> IntoIterator for &'a VideoFrame<'a> {
    type Item = <ImageIter<'a, VideoFrame<'a>> as Iterator>::Item;
    type IntoIter = ImageIter<'a, VideoFrame<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
