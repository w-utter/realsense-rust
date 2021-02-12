//! Defines the sensor type.

use crate::{
    check_rs2_error,
    device::{Device, DeviceConstructionError},
    kind::{
        extension::SENSOR_EXTENSIONS, OptionNotSupportedError, Rs2CameraInfo, Rs2Extension,
        Rs2Option,
    },
    stream::StreamProfile,
};
use anyhow::Result;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{convert::TryFrom, ffi::CStr, ptr::NonNull};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SensorConstructionError {
    #[error("Could not generate stream profile list for sensor. Reason: {0}")]
    CouldNotGenerateStreamProfileList(String),
    #[error("Could not get correct sensor from sensor list. Reason: {0}")]
    CouldNotGetSensorFromList(String),
}

pub struct Sensor {
    sensor_ptr: NonNull<sys::rs2_sensor>,
    stream_profiles_ptr: NonNull<sys::rs2_stream_profile_list>,
    should_drop: bool,
}

impl Drop for Sensor {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_stream_profiles_list(self.stream_profiles_ptr.as_ptr());
            sys::rs2_delete_sensor(self.sensor_ptr.as_ptr());
        }
    }
}

unsafe impl Send for Sensor {}

impl std::convert::TryFrom<NonNull<sys::rs2_sensor>> for Sensor {
    type Error = SensorConstructionError;

    fn try_from(sensor_ptr: NonNull<sys::rs2_sensor>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let stream_profiles_ptr = sys::rs2_get_stream_profiles(sensor_ptr.as_ptr(), &mut err);
            check_rs2_error!(
                err,
                SensorConstructionError::CouldNotGenerateStreamProfileList
            )?;

            Ok(Sensor {
                sensor_ptr,
                stream_profiles_ptr: NonNull::new(stream_profiles_ptr).unwrap(),
                should_drop: false,
            })
        }
    }
}

impl Sensor {
    /// Create a sensor from a sensor list and an index
    ///
    /// Unlike when you directly acquire a `*mut rs2_sensor` from an API in librealsense2, such as
    /// when calling `rs2_get_frame_sensor`, you have to drop this pointer at the end (because you
    /// now own it). When calling `try_from` we don't want to drop in the default case, since our
    /// `*mut rs2_sensor` may have come from another source.
    ///
    /// The main difference then is that this API defaults to using `rs2_create_sensor` vs. a call
    /// to get a sensor from somewhere else.
    ///
    /// This can fail for similar reasons to `try_from`, and is likewise only valid if `index` is
    /// less than the length of `sensor_list` (see `rs2_get_sensors_count` for how to get that
    /// length).
    ///
    /// Guaranteeing the lifetime / semantics of the sensor is difficult, so this should probably
    /// not be used outside of this crate. See `crate::device::Device` for where this is used.
    pub(crate) fn try_create(
        sensor_list: &NonNull<sys::rs2_sensor_list>,
        index: i32,
    ) -> Result<Self, SensorConstructionError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let sensor_ptr = sys::rs2_create_sensor(sensor_list.as_ptr(), index, &mut err);
            check_rs2_error!(err, SensorConstructionError::CouldNotGetSensorFromList)?;

            let nonnull_ptr = NonNull::new(sensor_ptr).unwrap();
            let mut sensor = Sensor::try_from(nonnull_ptr)?;
            sensor.should_drop = true;
            Ok(sensor)
        }
    }

    pub fn device(&self) -> Result<Device> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let device_ptr = sys::rs2_create_device_from_sensor(self.sensor_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, DeviceConstructionError::CouldNotCreateDeviceFromSensor)?;

            Ok(Device::try_from(NonNull::new(device_ptr).unwrap())?)
        }
    }

    pub fn extensions(&self) -> Vec<Rs2Extension> {
        SENSOR_EXTENSIONS
            .iter()
            .filter_map(|ext| unsafe {
                let mut err = std::ptr::null_mut::<sys::rs2_error>();
                let is_extendable = sys::rs2_is_sensor_extendable_to(
                    self.sensor_ptr.as_ptr(),
                    ext.to_u32().unwrap(),
                    &mut err,
                );

                if let Some(_) = err.as_ref() {
                    None
                } else if is_extendable != 0 {
                    Some(ext.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_option(&self, option: Rs2Option) -> Result<f32, OptionNotSupportedError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let val = sys::rs2_get_option(
                self.sensor_ptr.as_ptr().cast::<sys::rs2_options>(),
                option.to_u32().unwrap(),
                &mut err,
            );

            check_rs2_error!(err, OptionNotSupportedError)?;
            Ok(val)
        }
    }

    pub fn stream_profiles<'a>(&'a self) -> Vec<StreamProfile<'a>> {
        let mut profiles = Vec::new();
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let len =
                sys::rs2_get_stream_profiles_count(self.stream_profiles_ptr.as_ptr(), &mut err);

            if err.as_ref().is_some() {
                return profiles;
            }

            for i in 0..len {
                let profile_ptr =
                    sys::rs2_get_stream_profile(self.stream_profiles_ptr.as_ptr(), i, &mut err);

                if err.as_ref().is_some() {
                    err = std::ptr::null_mut();
                    continue;
                }

                let nonnull_ptr =
                    NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();

                match StreamProfile::try_from(nonnull_ptr) {
                    Ok(s) => {
                        profiles.push(s);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
        profiles
    }

    // fn recommended_processing_blocks(&self) -> Vec<ProcessingBlock>{}

    pub fn info(&self, camera_info: Rs2CameraInfo) -> Option<&CStr> {
        if !self.supports_info(camera_info) {
            return None;
        }

        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let val = sys::rs2_get_sensor_info(
                self.sensor_ptr.as_ptr(),
                camera_info.to_u32().unwrap(),
                &mut err,
            );

            if err.as_ref().is_some() {
                None
            } else {
                Some(CStr::from_ptr(val))
            }
        }
    }

    pub fn supports_info(&self, camera_info: Rs2CameraInfo) -> bool {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let supports_info = sys::rs2_supports_sensor_info(
                self.sensor_ptr.as_ptr(),
                camera_info.to_u32().unwrap(),
                &mut err,
            );

            err.as_ref().is_none() && supports_info != 0
        }
    }
}
