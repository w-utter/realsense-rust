//! Defines the common used enums.

use crate::common::*;

mod frame_metadata;
mod option;
mod timestamp_domain;

pub use frame_metadata::FrameMetadataValue;
pub use option::Rs2Option;
pub use timestamp_domain::TimestampDomain;

/// The enumeration of extensions.
#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extension {
    // sensor
    ColorSensor = sys::rs2_extension_RS2_EXTENSION_COLOR_SENSOR,
    MotionSensor = sys::rs2_extension_RS2_EXTENSION_MOTION_SENSOR,
    FishEyeSensor = sys::rs2_extension_RS2_EXTENSION_FISHEYE_SENSOR,
    DepthSensor = sys::rs2_extension_RS2_EXTENSION_DEPTH_SENSOR,
    DepthStereoSensor = sys::rs2_extension_RS2_EXTENSION_DEPTH_STEREO_SENSOR,
    SoftwareSensor = sys::rs2_extension_RS2_EXTENSION_SOFTWARE_SENSOR,
    PoseSensor = sys::rs2_extension_RS2_EXTENSION_POSE_SENSOR,
    L500DepthSensor = sys::rs2_extension_RS2_EXTENSION_L500_DEPTH_SENSOR,
    Tm2Sensor = sys::rs2_extension_RS2_EXTENSION_TM2_SENSOR,
    // frame
    VideoFrame = sys::rs2_extension_RS2_EXTENSION_VIDEO_FRAME,
    MotionFrame = sys::rs2_extension_RS2_EXTENSION_MOTION_FRAME,
    CompositeFrame = sys::rs2_extension_RS2_EXTENSION_COMPOSITE_FRAME,
    DepthFrame = sys::rs2_extension_RS2_EXTENSION_DEPTH_FRAME,
    DisparityFrame = sys::rs2_extension_RS2_EXTENSION_DISPARITY_FRAME,
    PoseFrame = sys::rs2_extension_RS2_EXTENSION_POSE_FRAME,
    Points = sys::rs2_extension_RS2_EXTENSION_POINTS,
    // filter
    DecimationFilter = sys::rs2_extension_RS2_EXTENSION_DECIMATION_FILTER,
    ThresholdFilter = sys::rs2_extension_RS2_EXTENSION_THRESHOLD_FILTER,
    DisparityFilter = sys::rs2_extension_RS2_EXTENSION_DISPARITY_FILTER,
    SpatialFilter = sys::rs2_extension_RS2_EXTENSION_SPATIAL_FILTER,
    TemporalFilter = sys::rs2_extension_RS2_EXTENSION_TEMPORAL_FILTER,
    HoleFillingFilter = sys::rs2_extension_RS2_EXTENSION_HOLE_FILLING_FILTER,
    ZeroOrderFilter = sys::rs2_extension_RS2_EXTENSION_ZERO_ORDER_FILTER,
    RecommendedFilters = sys::rs2_extension_RS2_EXTENSION_RECOMMENDED_FILTERS,
    // profile
    VideoProfile = sys::rs2_extension_RS2_EXTENSION_VIDEO_PROFILE,
    MotionProfile = sys::rs2_extension_RS2_EXTENSION_MOTION_PROFILE,
    PoseProfile = sys::rs2_extension_RS2_EXTENSION_POSE_PROFILE,
    // device
    SoftwareDevice = sys::rs2_extension_RS2_EXTENSION_SOFTWARE_DEVICE,
    UpdateDevice = sys::rs2_extension_RS2_EXTENSION_UPDATE_DEVICE,
    AutoCalibratedDevice = sys::rs2_extension_RS2_EXTENSION_AUTO_CALIBRATED_DEVICE,
    // misc
    AdvancedMode = sys::rs2_extension_RS2_EXTENSION_ADVANCED_MODE,
    Record = sys::rs2_extension_RS2_EXTENSION_RECORD,
    Playback = sys::rs2_extension_RS2_EXTENSION_PLAYBACK,
    Pose = sys::rs2_extension_RS2_EXTENSION_POSE,
    WheelOdometer = sys::rs2_extension_RS2_EXTENSION_WHEEL_ODOMETER,
    GlobalTimer = sys::rs2_extension_RS2_EXTENSION_GLOBAL_TIMER,
    Updatable = sys::rs2_extension_RS2_EXTENSION_UPDATABLE,
    Count = sys::rs2_extension_RS2_EXTENSION_COUNT,
    Tm2 = sys::rs2_extension_RS2_EXTENSION_TM2,
    Unknown = sys::rs2_extension_RS2_EXTENSION_UNKNOWN,
    Debug = sys::rs2_extension_RS2_EXTENSION_DEBUG,
    Info = sys::rs2_extension_RS2_EXTENSION_INFO,
    Motion = sys::rs2_extension_RS2_EXTENSION_MOTION,
    Options = sys::rs2_extension_RS2_EXTENSION_OPTIONS,
    Video = sys::rs2_extension_RS2_EXTENSION_VIDEO,
    Roi = sys::rs2_extension_RS2_EXTENSION_ROI,
}

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
