//! Type for representing the various pixel formats.

use crate::common::*;
use std::os::raw::c_void;

// For detailed pixel format information, see
// https://github.com/IntelRealSense/librealsense/blob/4f37f2ef0874c1716bce223b20e46d00532ffb04/wrappers/nodejs/index.js#L3865
pub enum PixelKind<'a> {
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
    Raw8 {
        val: &'a u8,
    },
    Y8 {
        y: &'a u8,
    },
    Y16 {
        y: &'a u16,
    },
    Z16 {
        depth: &'a u16,
    },
    Distance {
        distance: &'a f32,
    },
    Disparity32 {
        disparity: &'a f32,
    },
    Xyz32f {
        x: &'a f32,
        y: &'a f32,
        z: &'a f32,
    },
}

pub(crate) unsafe fn get_pixel<'a>(
    format: sys::rs2_format,
    data_size_in_bytes: usize,
    data: &'a c_void,
    stride_in_bytes: usize,
    col: usize,
    row: usize,
) -> PixelKind<'a> {
    let data_as_ptr = data as *const std::os::raw::c_void;

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
    match format {
        // YUYV is not uniform since it encapsulates two pixels over 32 bits (four u8
        // values). Instead, we can index YUYV (and UYVY) as follows:
        //
        // offset = (row * stride) + (col / 2) * 4
        //
        // The strange part here is the (col / 2) * 4. This is done because on odd rows we
        // don't want to offset to the next Y value, but rather take the full YUYV and pick
        // the correct Y depending on whether the row is even or odd.
        //
        // NOTE: Order matters because we are taking advantage of integer division here.
        //
        sys::rs2_format_RS2_FORMAT_YUYV => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col / 2) * 4;

            let y = if row % 2 == 0 {
                &slice[offset]
            } else {
                &slice[offset + 2]
            };

            PixelKind::Yuyv {
                y,
                u: &slice[offset + 1],
                v: &slice[offset + 3],
            }
        }
        // UYVY follows from the same exact pattern we use for YUYV, since it's more or less a
        // re-ordering of the underlying data.
        //
        sys::rs2_format_RS2_FORMAT_UYVY => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col / 2) * 4;

            let y = if row % 2 == 0 {
                &slice[offset + 1]
            } else {
                &slice[offset + 3]
            };

            PixelKind::Uyvy {
                y,
                u: &slice[offset],
                v: &slice[offset + 2],
            }
        }
        // For BGR / RGB, we do a similar trick, but since pixels aren't interleaved as they
        // are with YUYV / UYVY, the multipliers for column and row offsets can be uniform.
        //
        sys::rs2_format_RS2_FORMAT_BGR8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col * 3);

            PixelKind::Bgr8 {
                b: &slice[offset],
                g: &slice[offset + 1],
                r: &slice[offset + 2],
            }
        }
        // BGRA8 is more or less the same as BGR8, except we use 4 as a multiplier.
        //
        sys::rs2_format_RS2_FORMAT_BGRA8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col * 4);

            PixelKind::Bgra8 {
                b: &slice[offset],
                g: &slice[offset + 1],
                r: &slice[offset + 2],
                a: &slice[offset + 3],
            }
        }
        // RGB8 is the same as BGR8, the order is just different.
        //
        sys::rs2_format_RS2_FORMAT_RGB8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col * 3);

            PixelKind::Bgr8 {
                r: &slice[offset],
                g: &slice[offset + 1],
                b: &slice[offset + 2],
            }
        }
        // RGBA8 is the same as BGRA8, the order is just different.
        //
        sys::rs2_format_RS2_FORMAT_RGBA8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + (col * 4);

            PixelKind::Bgra8 {
                r: &slice[offset],
                g: &slice[offset + 1],
                b: &slice[offset + 2],
                a: &slice[offset + 3],
            }
        }
        sys::rs2_format_RS2_FORMAT_RAW8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + col;

            PixelKind::Raw8 {
                val: &slice[offset],
            }
        }
        sys::rs2_format_RS2_FORMAT_Y8 => {
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u8>(), data_size_in_bytes);
            let offset = (row * stride_in_bytes) + col;

            PixelKind::Y8 { y: &slice[offset] }
        }
        sys::rs2_format_RS2_FORMAT_Y16 => {
            let size = data_size_in_bytes / std::mem::size_of::<u16>();
            let stride = stride_in_bytes / std::mem::size_of::<u16>();
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u16>(), size);
            let offset = (row * stride) + col;

            PixelKind::Y16 { y: &slice[offset] }
        }
        sys::rs2_format_RS2_FORMAT_Z16 => {
            let size = data_size_in_bytes / std::mem::size_of::<u16>();
            let stride = stride_in_bytes / std::mem::size_of::<u16>();
            let slice = slice::from_raw_parts(data_as_ptr.cast::<u16>(), size);
            let offset = (row * stride) + col;

            PixelKind::Z16 {
                depth: &slice[offset],
            }
        }
        sys::rs2_format_RS2_FORMAT_DISTANCE => {
            let size = data_size_in_bytes / std::mem::size_of::<f32>();
            let stride = stride_in_bytes / std::mem::size_of::<f32>();
            let slice = slice::from_raw_parts(data_as_ptr.cast::<f32>(), size);
            let offset = (row * stride) + col;

            PixelKind::Distance {
                distance: &slice[offset],
            }
        }
        sys::rs2_format_RS2_FORMAT_DISPARITY32 => {
            let size = data_size_in_bytes / std::mem::size_of::<f32>();
            let stride = stride_in_bytes / std::mem::size_of::<f32>();
            let slice = slice::from_raw_parts(data_as_ptr.cast::<f32>(), size);
            let offset = (row * stride) + col;

            PixelKind::Disparity32 {
                disparity: &slice[offset],
            }
        }
        sys::rs2_format_RS2_FORMAT_XYZ32F => {
            let size = data_size_in_bytes / std::mem::size_of::<f32>();
            let stride = stride_in_bytes / std::mem::size_of::<f32>();
            let slice = slice::from_raw_parts(data_as_ptr.cast::<f32>(), size);
            let offset = (row * stride) + col;

            PixelKind::Xyz32f {
                x: &slice[offset],
                y: &slice[offset + 1],
                z: &slice[offset + 2],
            }
        }
        _ => {
            panic!("Unsupported video format.");
        }
    }
}
