//! Defines the device types.

use crate::{check_rs2_error, kind::Rs2CameraInfo, sensor::Sensor};
use anyhow::Result;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{convert::TryFrom, ffi::CStr, ptr::NonNull};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceConstructionError {
    #[error("Could not create device from sensor. Reason: {0}")]
    CouldNotCreateDeviceFromSensor(String),
    #[error("Could not generate sensor list for device. Reason: {0}")]
    CouldNotGenerateSensorList(String),
}

/// Represents a device instance.
#[derive(Debug)]
pub struct Device {
    device_ptr: NonNull<sys::rs2_device>,
    sensor_list_ptr: NonNull<sys::rs2_sensor_list>,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_sensor_list(self.sensor_list_ptr.as_ptr());
            sys::rs2_delete_device(self.device_ptr.as_ptr());
        }
    }
}

unsafe impl Send for Device {}

impl TryFrom<NonNull<sys::rs2_device>> for Device {
    type Error = DeviceConstructionError;

    fn try_from(device_ptr: NonNull<sys::rs2_device>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let sensor_list = sys::rs2_query_sensors(device_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, DeviceConstructionError::CouldNotGenerateSensorList)?;

            Ok(Device {
                device_ptr,
                sensor_list_ptr: NonNull::new(sensor_list).unwrap(),
            })
        }
    }
}

impl Device {
    /// Discover available sensors on device.
    pub fn sensors(&self) -> Vec<Sensor> {
        let mut sensors = Vec::new();
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let len = sys::rs2_get_sensors_count(self.sensor_list_ptr.as_ptr(), &mut err);

            if err.as_ref().is_some() {
                return sensors;
            }

            for i in 0..len {
                match Sensor::try_create(&self.sensor_list_ptr, i) {
                    Ok(s) => {
                        sensors.push(s);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
        sensors
    }

    pub fn hardware_reset(self) {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            // The only failure this can have is if device_ptr is null. This should not be the case
            // since we're storing a `NonNull` type.
            //
            // It's a bit weird, but we don't need to actually check the error. Because if the
            // device is null and this fails: you have an invalid device (so panic?) but if it
            // succeeds, the device is no longer valid and we need to drop it. This is why this
            // interface takes ownership of `self`.
            sys::rs2_hardware_reset(self.device_ptr.as_ptr(), &mut err);
        }
    }

    pub fn info(&self, camera_info: Rs2CameraInfo) -> Option<&CStr> {
        if !self.supports_info(camera_info) {
            return None;
        }

        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let val = sys::rs2_get_device_info(
                self.device_ptr.as_ptr(),
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
            let supports_info = sys::rs2_supports_device_info(
                self.device_ptr.as_ptr(),
                camera_info.to_u32().unwrap(),
                &mut err,
            );

            err.as_ref().is_none() && supports_info != 0
        }
    }
}
