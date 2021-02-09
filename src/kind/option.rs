//! Enumeration of RS2 sensor options.

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;
use std::ffi::CStr;

/// The enumeration of options.
#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2Option {
    BacklightCompensation = sys::rs2_option_RS2_OPTION_BACKLIGHT_COMPENSATION,
    Brightness = sys::rs2_option_RS2_OPTION_BRIGHTNESS,
    Contrast = sys::rs2_option_RS2_OPTION_CONTRAST,
    Exposure = sys::rs2_option_RS2_OPTION_EXPOSURE,
    Gain = sys::rs2_option_RS2_OPTION_GAIN,
    Gamma = sys::rs2_option_RS2_OPTION_GAMMA,
    Hue = sys::rs2_option_RS2_OPTION_HUE,
    Saturation = sys::rs2_option_RS2_OPTION_SATURATION,
    Sharpness = sys::rs2_option_RS2_OPTION_SHARPNESS,
    WhiteBalance = sys::rs2_option_RS2_OPTION_WHITE_BALANCE,
    EnableAutoExposure = sys::rs2_option_RS2_OPTION_ENABLE_AUTO_EXPOSURE,
    EnableAutoWhiteBalance = sys::rs2_option_RS2_OPTION_ENABLE_AUTO_WHITE_BALANCE,
    VisualPreset = sys::rs2_option_RS2_OPTION_VISUAL_PRESET,
    LaserPower = sys::rs2_option_RS2_OPTION_LASER_POWER,
    Accuracy = sys::rs2_option_RS2_OPTION_ACCURACY,
    MotionRange = sys::rs2_option_RS2_OPTION_MOTION_RANGE,
    FilterOption = sys::rs2_option_RS2_OPTION_FILTER_OPTION,
    ConfidenceThreshold = sys::rs2_option_RS2_OPTION_CONFIDENCE_THRESHOLD,
    EmitterEnabled = sys::rs2_option_RS2_OPTION_EMITTER_ENABLED,
    FramesQueueSize = sys::rs2_option_RS2_OPTION_FRAMES_QUEUE_SIZE,
    TotalFrameDrops = sys::rs2_option_RS2_OPTION_TOTAL_FRAME_DROPS,
    AutoExposureMode = sys::rs2_option_RS2_OPTION_AUTO_EXPOSURE_MODE,
    PowerLineFrequency = sys::rs2_option_RS2_OPTION_POWER_LINE_FREQUENCY,
    AsicTemperature = sys::rs2_option_RS2_OPTION_ASIC_TEMPERATURE,
    ErrorPollingEnabled = sys::rs2_option_RS2_OPTION_ERROR_POLLING_ENABLED,
    ProjectorTemperature = sys::rs2_option_RS2_OPTION_PROJECTOR_TEMPERATURE,
    OutputTriggerEnabled = sys::rs2_option_RS2_OPTION_OUTPUT_TRIGGER_ENABLED,
    MotionModuleTemperature = sys::rs2_option_RS2_OPTION_MOTION_MODULE_TEMPERATURE,
    DepthUnits = sys::rs2_option_RS2_OPTION_DEPTH_UNITS,
    EnableMotionCorrection = sys::rs2_option_RS2_OPTION_ENABLE_MOTION_CORRECTION,
    AutoExposurePriority = sys::rs2_option_RS2_OPTION_AUTO_EXPOSURE_PRIORITY,
    ColorScheme = sys::rs2_option_RS2_OPTION_COLOR_SCHEME,
    HistogramEqualizationEnabled = sys::rs2_option_RS2_OPTION_HISTOGRAM_EQUALIZATION_ENABLED,
    MinDistance = sys::rs2_option_RS2_OPTION_MIN_DISTANCE,
    MaxDistance = sys::rs2_option_RS2_OPTION_MAX_DISTANCE,
    TextureSource = sys::rs2_option_RS2_OPTION_TEXTURE_SOURCE,
    FilterMagnitude = sys::rs2_option_RS2_OPTION_FILTER_MAGNITUDE,
    FilterSmoothAlpha = sys::rs2_option_RS2_OPTION_FILTER_SMOOTH_ALPHA,
    FilterSmoothDelta = sys::rs2_option_RS2_OPTION_FILTER_SMOOTH_DELTA,
    HolesFill = sys::rs2_option_RS2_OPTION_HOLES_FILL,
    StereoBaseline = sys::rs2_option_RS2_OPTION_STEREO_BASELINE,
    AutoExposureConvergeStep = sys::rs2_option_RS2_OPTION_AUTO_EXPOSURE_CONVERGE_STEP,
    InterCamSyncMode = sys::rs2_option_RS2_OPTION_INTER_CAM_SYNC_MODE,
    StreamFilter = sys::rs2_option_RS2_OPTION_STREAM_FILTER,
    StreamFormatFilter = sys::rs2_option_RS2_OPTION_STREAM_FORMAT_FILTER,
    StreamIndexFilter = sys::rs2_option_RS2_OPTION_STREAM_INDEX_FILTER,
    EmitterOnOff = sys::rs2_option_RS2_OPTION_EMITTER_ON_OFF,
    ZeroOrderPointX = sys::rs2_option_RS2_OPTION_ZERO_ORDER_POINT_X,
    ZeroOrderPointY = sys::rs2_option_RS2_OPTION_ZERO_ORDER_POINT_Y,
    LldTemperature = sys::rs2_option_RS2_OPTION_LLD_TEMPERATURE,
    McTemperature = sys::rs2_option_RS2_OPTION_MC_TEMPERATURE,
    MaTemperature = sys::rs2_option_RS2_OPTION_MA_TEMPERATURE,
    HardwarePreset = sys::rs2_option_RS2_OPTION_HARDWARE_PRESET,
    GlobalTimeEnabled = sys::rs2_option_RS2_OPTION_GLOBAL_TIME_ENABLED,
    ApdTemperature = sys::rs2_option_RS2_OPTION_APD_TEMPERATURE,
    EnableMapping = sys::rs2_option_RS2_OPTION_ENABLE_MAPPING,
    EnableRelocalization = sys::rs2_option_RS2_OPTION_ENABLE_RELOCALIZATION,
    EnablePoseJumping = sys::rs2_option_RS2_OPTION_ENABLE_POSE_JUMPING,
    EnableDynamicCalibration = sys::rs2_option_RS2_OPTION_ENABLE_DYNAMIC_CALIBRATION,
    DepthOffset = sys::rs2_option_RS2_OPTION_DEPTH_OFFSET,
    LedPower = sys::rs2_option_RS2_OPTION_LED_POWER,
    ZeroOrderEnabled = sys::rs2_option_RS2_OPTION_ZERO_ORDER_ENABLED,
    EnableMapPreservation = sys::rs2_option_RS2_OPTION_ENABLE_MAP_PRESERVATION,
    // Not included since this just tells us the total number of options.
    //
    // Count = sys::rs2_option_RS2_OPTION_COUNT,
}

impl Rs2Option {
    pub fn to_cstr(&self) -> &'static CStr {
        unsafe {
            let ptr = sys::rs2_option_to_string(*self as sys::rs2_option);
            CStr::from_ptr(ptr)
        }
    }

    pub fn to_str(&self) -> &'static str {
        self.to_cstr().to_str().unwrap()
    }
}

impl ToString for Rs2Option {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}
