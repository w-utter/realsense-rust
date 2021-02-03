use super::frame_trait::{Frame, FrameConstructionError};
use super::kind::Kind;
use crate::{common::*, stream};
use std::ffi::CStr;

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

struct VideoFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    width: usize,
    height: usize,
    stride: usize,
    bits_per_pixel: usize,
    frame_stream_profile: stream::Profile,
    data: &'a [u8],
}

impl<'a> VideoFrame<'a> {
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

    fn at(&self, col: usize, row: usize) -> PixelFormat<'a> {
        // Realsense stores data in col-major format. Normally, we would offset into a uniform
        // array in column major format with the following equation:
        //
        // offset = column * height + row
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
            // offset = (column * height * 2) + (row / 2) * 4
            //
            // The strange part here is the (row / 2) * 4. This is done because on odd rows we
            // don't want to offset to the next Y value, but rather take the full YUYV and pick
            // the correct Y depending on whether the row is even or odd.
            //
            // NOTE: Order matters because we are taking advantage of integer division here.
            //
            sys::rs2_format_RS2_FORMAT_YUYV => {
                let offset = (col * self.height * 2) + (row / 2) * 4;

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
                let offset = (col * self.height * 2) + (row / 2) * 4;

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
            // offset = (column * height * 3) + (row * 3)
            //
            sys::rs2_format_RS2_FORMAT_BGR8 => {
                let offset = (col * self.height * 3) + (row * 3);

                PixelFormat::Bgr8 {
                    b: &self.data[offset],
                    g: &self.data[offset + 1],
                    r: &self.data[offset + 2],
                }
            }
            // BGR8 is more or less the same, except we use 4 as a multiplier.
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

impl<'a> Frame for VideoFrame<'a>
where
    Self: Sized,
{
    fn new(
        frame_ptr: NonNull<sys::rs2_frame>,
    ) -> std::result::Result<Self, FrameConstructionError> {
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();
            let width = sys::rs2_get_frame_width(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetWidth(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }
            let height = sys::rs2_get_frame_height(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetHeight(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }
            let bits_per_pixel = sys::rs2_get_frame_bits_per_pixel(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetBitsPerPixel(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }

            let stride = sys::rs2_get_frame_stride_in_bytes(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetStride(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetFrameStreamProfile(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }
            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = stream::Profile::new(nonnull_profile_ptr).map_err(|_| {
                FrameConstructionError::CouldNotGetFrameStreamProfile(String::from(
                    "Could not construct stream profile.",
                ))
            })?;

            let size = sys::rs2_get_frame_data_size(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetDataSize(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }
            debug_assert_eq!(size, width * height * bits_per_pixel / 8);

            let ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                return Err(FrameConstructionError::CouldNotGetData(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }
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
