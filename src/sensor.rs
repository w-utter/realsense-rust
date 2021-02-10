//! Defines the sensor type.

use crate::{
    check_rs2_error,
    device::Device,
    kind::{
        extension::SENSOR_EXTENSIONS, OptionNotSupportedError, Rs2CameraInfo, Rs2Extension,
        Rs2Option,
    },
    stream,
};
use anyhow::Result;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{convert::TryFrom, ffi::CStr, ptr::NonNull};

pub struct Sensor {
    sensor_ptr: NonNull<sys::rs2_sensor>,
}

impl Drop for Sensor {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_sensor(self.sensor_ptr.as_ptr());
        }
    }
}

unsafe impl Send for Sensor {}

impl std::convert::From<NonNull<sys::rs2_sensor>> for Sensor {
    fn from(sensor_ptr: NonNull<sys::rs2_sensor>) -> Self {
        Sensor { sensor_ptr }
    }
}

impl Sensor {
    pub fn device(&self) -> Result<Device> {
        Device::try_from(self)
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

    pub fn stream_profiles(&self) -> Vec<stream::Profile> {
        let mut profiles = Vec::new();
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let stream_profiles_ptr =
                sys::rs2_get_stream_profiles(self.sensor_ptr.as_ptr(), &mut err);

            if err.as_ref().is_some() {
                return profiles;
            }

            let len = sys::rs2_get_stream_profiles_count(stream_profiles_ptr, &mut err);

            if err.as_ref().is_some() {
                return profiles;
            }

            for i in 0..len {
                let profile_ptr = sys::rs2_get_stream_profile(stream_profiles_ptr, i, &mut err);

                if err.as_ref().is_some() {
                    err = std::ptr::null_mut();
                    continue;
                }

                let nonnull_ptr =
                    NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();

                match stream::Profile::try_from(nonnull_ptr) {
                    Ok(s) => {
                        profiles.push(s);
                    }
                    Err(e) => {
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
