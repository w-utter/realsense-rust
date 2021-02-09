//! The enumeration of all categories of stream.

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2StreamKind {
    Any = sys::rs2_stream_RS2_STREAM_ANY,
    Depth = sys::rs2_stream_RS2_STREAM_DEPTH,
    Color = sys::rs2_stream_RS2_STREAM_COLOR,
    Infrared = sys::rs2_stream_RS2_STREAM_INFRARED,
    Fisheye = sys::rs2_stream_RS2_STREAM_FISHEYE,
    Gyro = sys::rs2_stream_RS2_STREAM_GYRO,
    Accel = sys::rs2_stream_RS2_STREAM_ACCEL,
    Gpio = sys::rs2_stream_RS2_STREAM_GPIO,
    Pose = sys::rs2_stream_RS2_STREAM_POSE,
    Confidence = sys::rs2_stream_RS2_STREAM_CONFIDENCE,
    // Not included since this just tells us the total number stream types
    //
    // Count = sys::rs2_stream_RS2_STREAM_COUNT,
}
