//! The enumeration of frame data format.

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;

#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Any = sys::rs2_format_RS2_FORMAT_ANY,
    Yuyv = sys::rs2_format_RS2_FORMAT_YUYV,
    Uyvy = sys::rs2_format_RS2_FORMAT_UYVY,
    MotionRaw = sys::rs2_format_RS2_FORMAT_MOTION_RAW,
    GpioRaw = sys::rs2_format_RS2_FORMAT_GPIO_RAW,
    Distance = sys::rs2_format_RS2_FORMAT_DISTANCE,
    Mjpeg = sys::rs2_format_RS2_FORMAT_MJPEG,
    Inzi = sys::rs2_format_RS2_FORMAT_INZI,
    Invi = sys::rs2_format_RS2_FORMAT_INVI,
    _6Dof = sys::rs2_format_RS2_FORMAT_6DOF,
    Bgr8 = sys::rs2_format_RS2_FORMAT_BGR8,
    Bgra8 = sys::rs2_format_RS2_FORMAT_BGRA8,
    Disparity16 = sys::rs2_format_RS2_FORMAT_DISPARITY16,
    Disparity32 = sys::rs2_format_RS2_FORMAT_DISPARITY32,
    MotionXyz32F = sys::rs2_format_RS2_FORMAT_MOTION_XYZ32F,
    Raw8 = sys::rs2_format_RS2_FORMAT_RAW8,
    Raw10 = sys::rs2_format_RS2_FORMAT_RAW10,
    Raw16 = sys::rs2_format_RS2_FORMAT_RAW16,
    Rgb8 = sys::rs2_format_RS2_FORMAT_RGB8,
    Rgba8 = sys::rs2_format_RS2_FORMAT_RGBA8,
    W10 = sys::rs2_format_RS2_FORMAT_W10,
    Xyz32F = sys::rs2_format_RS2_FORMAT_XYZ32F,
    Y8 = sys::rs2_format_RS2_FORMAT_Y8,
    Y8I = sys::rs2_format_RS2_FORMAT_Y8I,
    Y10Bpack = sys::rs2_format_RS2_FORMAT_Y10BPACK,
    Y12I = sys::rs2_format_RS2_FORMAT_Y12I,
    Y16 = sys::rs2_format_RS2_FORMAT_Y16,
    Z16 = sys::rs2_format_RS2_FORMAT_Z16,
    // Not included since this just tells us the total number of formats
    //
    // Count = sys::rs2_format_RS2_FORMAT_COUNT,
}


