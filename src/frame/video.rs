//! Type for representing a video frame taken from a color or IR camera.

use super::frame_traits::{
    FrameConstructionError, VideoFrameEx, VideoFrameUnsafeEx, BITS_PER_BYTE,
};
use super::{iter::ImageIter, kind::Kind};
use crate::{check_rs2_error, common::*, stream};
use std::result::Result;

// For detailed pixel format information, see
// https://github.com/IntelRealSense/librealsense/blob/4f37f2ef0874c1716bce223b20e46d00532ffb04/wrappers/nodejs/index.js#L3865
pub enum PixelFormat<'a> {
    Yuyv {
        y: &'a u8,
        u: &'a u8,
        v: &'a u8,
    },
    Uyvy {
        y: &'a u8,
        u: &'a u8,
        v: &'a u8,
    },
    Bgr8 {
        b: &'a u8,
        g: &'a u8,
        r: &'a u8,
    },
    Bgra8 {
        b: &'a u8,
        g: &'a u8,
        r: &'a u8,
        a: &'a u8,
    },
    Rgb8 {
        r: &'a u8,
        g: &'a u8,
        b: &'a u8,
    },
    Rgba8 {
        r: &'a u8,
        g: &'a u8,
        b: &'a u8,
        a: &'a u8,
    },
}

pub struct VideoFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    width: usize,
    height: usize,
    stride: usize,
    bits_per_pixel: usize,
    frame_stream_profile: stream::Profile,
    data: &'a [u8],
}

impl<'a> VideoFrame<'a> {
    pub fn profile(&'a self) -> &'a stream::Profile {
        &self.frame_stream_profile
    }

    pub fn iter(&'a self) -> ImageIter<'a, VideoFrame<'a>> {
        ImageIter {
            frame: self,
            column: 0,
            row: 0,
        }
    }

    pub fn get_raw(&'a self) -> &'a [u8] {
        self.data
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
    type Error = FrameConstructionError;

    fn try_from(frame_ptr: NonNull<sys::rs2_frame>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let width = sys::rs2_get_frame_width(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetWidth);

            let height = sys::rs2_get_frame_height(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetHeight);

            let bits_per_pixel = sys::rs2_get_frame_bits_per_pixel(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetBitsPerPixel);

            let stride = sys::rs2_get_frame_stride_in_bytes(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetStride);

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile);

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = stream::Profile::new(nonnull_profile_ptr).map_err(|_| {
                FrameConstructionError::CouldNotGetFrameStreamProfile(String::from(
                    "Could not construct stream profile.",
                ))
            })?;

            let size = sys::rs2_get_frame_data_size(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetDataSize);

            debug_assert_eq!(size, width * height * bits_per_pixel / BITS_PER_BYTE);

            let ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData);

            let data = slice::from_raw_parts(ptr.cast::<u8>(), size as usize);

            Ok(VideoFrame {
                frame_ptr,
                width: width as usize,
                height: height as usize,
                stride: stride as usize,
                bits_per_pixel: bits_per_pixel as usize,
                frame_stream_profile: profile,
                data,
            })
        }
    }
}

impl<'a> VideoFrameUnsafeEx for VideoFrame<'a> {
    type Output = PixelFormat<'a>;

    fn get_unchecked(&self, col: usize, row: usize) -> Self::Output {
        // Realsense stores frame data in row-major format. Normally, we would offset into a
        // uniform array in column major format with the following equation:
        //
        // offset = row * width + column
        //
        // The assumption here being that it is a uniform array. See individual comments below for
        // how each offset equation differs.
        //
        // NOTE; You _could_ still represent this same pointer arithmetic in row-major form, but be
        // warned that the equations will look fairly different.
        //
        match self.frame_stream_profile.format() {
            // YUYV is not uniform since it encapsulates two pixels over 32 bits (four u8
            // values). Instead, we can index YUYV (and UYVY) as follows:
            //
            // offset = (row * width * 2) + (col / 2) * 4
            //
            // The strange part here is the (col / 2) * 4. This is done because on odd rows we
            // don't want to offset to the next Y value, but rather take the full YUYV and pick
            // the correct Y depending on whether the row is even or odd.
            //
            // NOTE: Order matters because we are taking advantage of integer division here.
            //
            sys::rs2_format_RS2_FORMAT_YUYV => {
                let offset = (row * self.width * 2) + (col / 2) * 4;

                let y = if row % 2 == 0 {
                    &self.data[offset]
                } else {
                    &self.data[offset + 2]
                };

                PixelFormat::Yuyv {
                    y,
                    u: &self.data[offset + 1],
                    v: &self.data[offset + 3],
                }
            }
            // UYVY follows from the same exact pattern we use for YUYV, since it's more or less a
            // re-ordering of the underlying data.
            //
            sys::rs2_format_RS2_FORMAT_UYVY => {
                let offset = (row * self.width * 2) + (col / 2) * 4;

                let y = if row % 2 == 0 {
                    &self.data[offset + 1]
                } else {
                    &self.data[offset + 3]
                };

                PixelFormat::Uyvy {
                    y,
                    u: &self.data[offset],
                    v: &self.data[offset + 2],
                }
            }
            // For BGR / RGB, we do a similar trick, but since pixels aren't interleaved as they
            // are with YUYV / UYVY, the multipliers for column and row offsets can be uniform.
            //
            // offset = (row * width * 3) + (col * 3)
            //
            sys::rs2_format_RS2_FORMAT_BGR8 => {
                let offset = (col * self.height * 3) + (row * 3);

                PixelFormat::Bgr8 {
                    b: &self.data[offset],
                    g: &self.data[offset + 1],
                    r: &self.data[offset + 2],
                }
            }
            // BGRA8 is more or less the same as BGR8, except we use 4 as a multiplier.
            //
            sys::rs2_format_RS2_FORMAT_BGRA8 => {
                let offset = (col * self.height * 4) + (row * 4);

                PixelFormat::Bgra8 {
                    b: &self.data[offset],
                    g: &self.data[offset + 1],
                    r: &self.data[offset + 2],
                    a: &self.data[offset + 3],
                }
            }
            // RGB8 is the same as BGR8, the order is just different.
            //
            sys::rs2_format_RS2_FORMAT_RGB8 => {
                let offset = (col * self.height * 3) + (row * 3);

                PixelFormat::Bgr8 {
                    r: &self.data[offset],
                    g: &self.data[offset + 1],
                    b: &self.data[offset + 2],
                }
            }
            // RGBA8 is the same as BGRA8, the order is just different.
            //
            sys::rs2_format_RS2_FORMAT_RGBA8 => {
                let offset = (col * self.height * 4) + (row * 4);

                PixelFormat::Bgra8 {
                    r: &self.data[offset],
                    g: &self.data[offset + 1],
                    b: &self.data[offset + 2],
                    a: &self.data[offset + 3],
                }
            }
            _ => {
                panic!("Unsupported video format.");
            }
        }
    }
}

impl<'a> VideoFrameEx for VideoFrame<'a> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn bits_per_pixel(&self) -> usize {
        self.bits_per_pixel
    }

    fn get(&self, col: usize, row: usize) -> Option<Self::Output> {
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
