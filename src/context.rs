//! Defines the sensor context.

use crate::{
    base::from_path,
    check_rs2_error,
    device::Device,
    device_hub::DeviceHub,
    kind::{Rs2Exception, Rs2ProductLine},
};
use anyhow::Result;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{convert::TryFrom, path::Path, ptr::NonNull};
use thiserror::Error;

#[derive(Debug)]
pub struct Context {
    context_ptr: NonNull<sys::rs2_context>,
}

#[derive(Error, Debug)]
#[error("Could not construct the context. Type: {0}; Reason: {1}")]
pub struct ContextConstructionError(pub Rs2Exception, pub String);

#[derive(Error, Debug)]
#[error("Could not get the device hub from the context. Type: {0}; Reason: {1}")]
pub struct CouldNotGetDeviceHubError(pub Rs2Exception, pub String);

#[derive(Error, Debug)]
#[error("Could not add a device from file. Type: {0}; Reason: {1}")]
pub struct CouldNotAddDeviceError(pub Rs2Exception, pub String);

#[derive(Error, Debug)]
#[error("Could not remove device from file. Type: {0}; Reason: {1}")]
pub struct CouldNotRemoveDeviceError(pub Rs2Exception, pub String);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::rs2_delete_context(self.context_ptr.as_ptr()) }
    }
}

unsafe impl Send for Context {}

impl Context {
    /// Create an instance.
    pub fn new() -> Result<Self> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let ptr = sys::rs2_create_context(sys::RS2_API_VERSION as i32, &mut err);
            check_rs2_error!(err, ContextConstructionError)?;

            Ok(Self {
                context_ptr: NonNull::new(ptr).unwrap(),
            })
        }
    }

    /// Create an [DeviceHub](DeviceHub) instance.
    pub fn create_device_hub(&self) -> Result<DeviceHub> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let devicehub_ptr = sys::rs2_create_device_hub(self.context_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotGetDeviceHubError)?;

            Ok(DeviceHub::from(NonNull::new(devicehub_ptr).unwrap()))
        }
    }

    /// Discover available devices.
    pub fn query_devices(&self, product_mask: Vec<Rs2ProductLine>) -> Vec<Device> {
        // TODO/TEST: Make sure that an empty mask (therefore giving no filter) gives
        // us _all_ devices, not _no_ devices.

        let mask = if product_mask.len() > 0 {
            product_mask.iter().fold(0, |k, v| k | v.to_u32().unwrap()) as i32
        } else {
            Rs2ProductLine::Any.to_i32().unwrap()
        };

        let devices = Vec::new();
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let device_list_ptr =
                sys::rs2_query_devices_ex(self.context_ptr.as_ptr(), mask, &mut err);

            if err.as_ref().is_some() {
                return devices;
            }

            let device_list = NonNull::new(device_list_ptr).unwrap();

            let len = sys::rs2_get_device_count(device_list.as_ptr(), &mut err);

            if err.as_ref().is_some() {
                sys::rs2_delete_device_list(device_list.as_ptr());
                return devices;
            }

            for i in 0..len {
                match Device::try_create(&device_list, i) {
                    Ok(d) => {
                        devices.push(d);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }

            sys::rs2_delete_device_list(device_list.as_ptr());
        }
        devices
    }

    /// Add device file to context.
    pub fn add_device<P>(&mut self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = from_path(file)?;
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_context_add_device(self.context_ptr.as_ptr(), path.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotAddDeviceError)?;

            Ok(())
        }
    }

    pub fn remove_device<P>(&mut self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = from_path(file)?;
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_context_remove_device(self.context_ptr.as_ptr(), path.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotRemoveDeviceError)?;

            Ok(())
        }
    }

    pub unsafe fn get_raw(&self) -> NonNull<sys::rs2_context> {
        self.context_ptr
    }
}
