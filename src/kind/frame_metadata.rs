//! The enumeration of metadata kinds of a frame.

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2FrameMetadata {
    FrameCounter = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_COUNTER,
    FrameTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_TIMESTAMP,
    SensorTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SENSOR_TIMESTAMP,
    ActualExposure = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_EXPOSURE,
    GainLevel = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAIN_LEVEL,
    AutoExposure = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_EXPOSURE,
    WhiteBalance = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_WHITE_BALANCE,
    TimeOfArrival = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL,
    Temperature = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_TEMPERATURE,
    BackendTimestamp = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKEND_TIMESTAMP,
    ActualFps = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_FPS,
    FrameLaserPower = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER,
    FrameLaserPowerMode = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER_MODE,
    ExposurePriority = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_PRIORITY,
    ExposureRoiLeft = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_LEFT,
    ExposureRoiRight = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_RIGHT,
    ExposureRoiTop = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_TOP,
    ExposureRoiBottom = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_BOTTOM,
    Brightness = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BRIGHTNESS,
    Contrast = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_CONTRAST,
    Saturation = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SATURATION,
    Sharpness = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_SHARPNESS,
    AutoWhiteBalanceTemperature =
        sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_WHITE_BALANCE_TEMPERATURE,
    BacklightCompensation = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKLIGHT_COMPENSATION,
    Hue = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_HUE,
    Gamma = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAMMA,
    ManualWhiteBalance = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_MANUAL_WHITE_BALANCE,
    PowerLineFrequency = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_POWER_LINE_FREQUENCY,
    LowLightCompensation = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_LOW_LIGHT_COMPENSATION,
    FrameEmitterMode = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_EMITTER_MODE,
    FrameLedPower = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LED_POWER,
    // Not included since this just tells us the total number of metadata fields
    //
    // Count = sys::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT,
}
