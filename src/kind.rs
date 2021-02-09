//! Defines the common used enums.

use crate::common::*;

mod extension;
mod frame_metadata;
mod option;
mod timestamp_domain;

pub use extension::Rs2Extension;
pub use frame_metadata::Rs2FrameMetadata;
pub use option::Rs2Option;
pub use timestamp_domain::TimestampDomain;

/// The enumeration of sensor information.
#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraInfo {
    Name = sys::rs2_camera_info_RS2_CAMERA_INFO_NAME,
    SerialNumber = sys::rs2_camera_info_RS2_CAMERA_INFO_SERIAL_NUMBER,
    FirmwareVersion = sys::rs2_camera_info_RS2_CAMERA_INFO_FIRMWARE_VERSION,
    RecommendedFirmwareVersion = sys::rs2_camera_info_RS2_CAMERA_INFO_RECOMMENDED_FIRMWARE_VERSION,
    PhysicalPort = sys::rs2_camera_info_RS2_CAMERA_INFO_PHYSICAL_PORT,
    DebugOpCode = sys::rs2_camera_info_RS2_CAMERA_INFO_DEBUG_OP_CODE,
    AdvancedMode = sys::rs2_camera_info_RS2_CAMERA_INFO_ADVANCED_MODE,
    ProductId = sys::rs2_camera_info_RS2_CAMERA_INFO_PRODUCT_ID,
    CameraLocked = sys::rs2_camera_info_RS2_CAMERA_INFO_CAMERA_LOCKED,
    UsbTypeDescriptor = sys::rs2_camera_info_RS2_CAMERA_INFO_USB_TYPE_DESCRIPTOR,
    ProductLine = sys::rs2_camera_info_RS2_CAMERA_INFO_PRODUCT_LINE,
    AsicSerialNumber = sys::rs2_camera_info_RS2_CAMERA_INFO_ASIC_SERIAL_NUMBER,
    FirmwareUpdateId = sys::rs2_camera_info_RS2_CAMERA_INFO_FIRMWARE_UPDATE_ID,
    Count = sys::rs2_camera_info_RS2_CAMERA_INFO_COUNT,
}

/// The enumeration of all categories of stream.
#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamKind {
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
    Count = sys::rs2_stream_RS2_STREAM_COUNT,
}

/// The enumeration of frame data format.
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
    Count = sys::rs2_format_RS2_FORMAT_COUNT,
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
}

/// The enumeration of color schemes.
#[repr(usize)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    Jet = 0,
    Classic = 1,
    WhiteToBlack = 2,
    BlackToWhite = 3,
    Bio = 4,
    Cold = 5,
    Warm = 6,
    Quantized = 7,
    Pattern = 8,
    Hue = 9,
}

/// The enumeration of persistence controls.
#[repr(usize)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersistenceControl {
    Disabled = 0,
    Valid8OutOf8 = 1,
    Valid2OutOf3 = 2,
    Valid2OutOf4 = 3,
    Valid2OutOf8 = 4,
    Valid1OutOf2 = 5,
    Valid1OutOf5 = 6,
    Valid1OutOf8 = 7,
    Indefinitely = 8,
}

/// The enumeration of persistence controls.
#[repr(usize)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HoleFillingMode {
    FillFromLeft = 0,
    FarestFromAround = 1,
    NearestFromAround = 2,
}
