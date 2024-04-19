//! Type that defines a RealSense context used by the rest of the API

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
use std::{collections::HashSet, convert::From, path::Path, ptr::NonNull};
use thiserror::Error;

/// Type describing a RealSense context, used by the rest of the API.
#[derive(Debug)]
pub struct Context {
    /// A non-null pointer to the underlying librealsense context.
    context_ptr: NonNull<sys::rs2_context>,
}

/// An error type describing failure to construct a context.
#[derive(Error, Debug)]
#[error("Could not construct the context. Type: {0}; Reason: {1}")]
pub struct ContextConstructionError(pub Rs2Exception, pub String);

/// An error type describing failure to get the device hub from a context.
#[derive(Error, Debug)]
#[error("Could not get the device hub from the context. Type: {0}; Reason: {1}")]
pub struct CouldNotGetDeviceHubError(pub Rs2Exception, pub String);

/// An error type describing failure to add a device from a file.
#[derive(Error, Debug)]
#[error("Could not add a device from file. Type: {0}; Reason: {1}")]
pub struct CouldNotAddDeviceError(pub Rs2Exception, pub String);

/// An error type describing failure to remove a device from a file.
#[derive(Error, Debug)]
#[error("Could not remove device from file. Type: {0}; Reason: {1}")]
pub struct CouldNotRemoveDeviceError(pub Rs2Exception, pub String);

/// An error type describing failure to attach a callback for devices
#[derive(Error, Debug)]
#[error("Could not remove device from file. Type: {0}; Reason: {1}")]
pub struct CouldNotSetDeviceCallbackError(pub Rs2Exception, pub String);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::rs2_delete_context(self.context_ptr.as_ptr()) }
    }
}

unsafe impl Send for Context {}

impl Context {
    /// Construct a new context.
    ///
    /// # Errors
    ///
    /// Returns [`ContextConstructionError`] if the context cannot be created.
    ///
    pub fn new() -> Result<Self, ContextConstructionError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let ptr = sys::rs2_create_context(sys::RS2_API_VERSION as i32, &mut err);
            check_rs2_error!(err, ContextConstructionError)?;

            Ok(Self {
                context_ptr: NonNull::new(ptr).unwrap(),
            })
        }
    }

    /// Creates a device hub from the context.
    ///
    /// # Errors
    ///
    /// Returns [`CouldNotGetDeviceHubError`] if the device hub cannot be created.
    ///
    pub fn create_device_hub(&self) -> Result<DeviceHub, CouldNotGetDeviceHubError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let devicehub_ptr = sys::rs2_create_device_hub(self.context_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotGetDeviceHubError)?;

            Ok(DeviceHub::from(NonNull::new(devicehub_ptr).unwrap()))
        }
    }

    /// Get a list of devices that are already connected to the host.
    pub fn query_devices(&self, product_mask: HashSet<Rs2ProductLine>) -> impl Iterator<Item = Device> {
        // TODO/TEST: Make sure that an empty mask (therefore giving no filter) gives
        // us _all_ devices, not _no_ devices.

        let mask = if product_mask.is_empty() {
            Rs2ProductLine::Any.to_i32().unwrap()
        } else {
            product_mask.iter().fold(0, |k, v| k | v.to_u32().unwrap()) as i32
        };

        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let device_list_ptr =
                sys::rs2_query_devices_ex(self.context_ptr.as_ptr(), mask, &mut err);

            if err.as_ref().is_some() {
                sys::rs2_free_error(err);
                return DeviceIter::new_empty();
            }

            let device_list = NonNull::new(device_list_ptr).unwrap();

            let len = sys::rs2_get_device_count(device_list.as_ptr(), &mut err);

            if err.as_ref().is_some() {
                sys::rs2_free_error(err);
                sys::rs2_delete_device_list(device_list.as_ptr());
                return DeviceIter::new_empty();
            }
            DeviceIter::new(device_list, len)
        }
    }

    //callback is (devices_removed, devices_added)
    pub fn on_devices_changed<'a, F>(&'a self, f: F) -> Result<DeviceMonitor<'a>, CouldNotSetDeviceCallbackError> 
        where
            F: FnMut(&dyn Iterator<Item = Device>, &dyn Iterator<Item = Device>) + Send + 'static
    {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let f = Box::into_raw(Box::new(f));
            sys::rs2_set_devices_changed_callback(self.context_ptr.as_ptr(), f, Some(trampoline::<F>), &mut err);

            check_rs2_error!(err, CouldNotSetDeviceCallbackError)?;

            let monitor = DeviceMonitor::new(f);
            Ok(monitor)
        }
        Ok(())
    }

    /// Create a new device and add it to the context.
    ///
    /// This adds a "device" at a particular file on the system to the RealSense context. Returns a
    /// handle to the device, or an error if this call fails.
    ///
    /// # Errors
    ///
    /// Returns [`NulError`](std::ffi::NulError) if the provided file path cannot be cleanly
    /// represented as a [`CString`](std::ffi::CString). This usually only occurs if you have null
    /// characters in the path. Constructing a path using the utilties in Rust's [`std::fs`] are
    /// expected to work.
    ///
    /// Returns [`CouldNotAddDeviceError`] if the device cannot be added.
    ///
    pub fn add_device<P>(&mut self, file: P) -> Result<Device>
    where
        P: AsRef<Path>,
    {
        let path = from_path(file)?;
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let device_ptr =
                sys::rs2_context_add_device(self.context_ptr.as_ptr(), path.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotAddDeviceError)?;

            Ok(Device::from(NonNull::new(device_ptr).unwrap()))
        }
    }

    /// Removes a playback device from the context, if it exists
    ///
    /// This removes a "device" at a particular file on the system from the RealSense context.
    /// Returns nothing (null tuple) or an Error if the device cannot be removed.
    ///
    /// # Errors
    ///
    /// Returns [`CouldNotRemoveDeviceError`] if the device cannot be removed for any reason.
    ///
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

    /// Get the underlying low-level pointer to the context object.
    ///
    /// # Safety
    ///
    /// This method is not intended to be called or used outside of the crate itself. Be warned, it
    /// is _undefined behaviour_ to call [`realsense_sys::rs2_delete_context`] on this pointer. If
    /// you do, you risk a double-free error when the [`Context`] struct itself is dropped.
    ///
    pub(crate) unsafe fn get_raw(&self) -> NonNull<sys::rs2_context> {
        self.context_ptr
    }
}

struct DeviceIter {
    idx: i32,
    len: i32,
    dev_list: NonNull<sys::rs2_device_list>,
}

impl Drop for DeviceIter {
    fn drop(&mut self) {
        unsafe  {
            sys::rs2_delete_device_list(self.dev_list.as_ptr());
        }
    }
}

impl DeviceIter {
    fn new(list: NonNull<sys::rs2_device_list>, len: i32) -> Self {
        Self {
            dev_list: list,
            len,
            idx: 0,
        }
    }

    fn new_empty() -> Self {
        Self {
            dev_list: unsafe { NonNull::new_unchecked(core::ptr::null_mut()) },
            len: 0,
            idx: 0,
        }
    }
}

impl Iterator for DeviceIter {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len {
            return None;
        }

        match Device::try_create(self.dev_list.as_ptr(), self.idx) {
            Ok(dev) => {
                self.idx += 1;
                Some(dev)
            }
            Err(_) => {
                self.idx += 1;
                self.next()
            }
        }
    }
}

use std::os::raw::c_void;

use core::marker::PhantomData;

struct DeviceMonitor<'a> {
    handle: *mut dyn FnMut(&dyn Iterator<Item = Device>, &dyn Iterator<Item = Device>),
    _lt: PhantomData<&'a ()>
}

impl DeviceMonitor<'_> {
    fn new(ptr: *mut dyn FnMut(&dyn Iterator<Item = Device>, &dyn Iterator<Item = Device>)) -> Self {
        Self {
            handle: ptr,
            _lt: PhantomData,
        }
    }
}

impl <'a> Drop for DeviceMonitor<'a> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.handle) };
    }
}

unsafe extern "C" fn trampoline<F>(devices_removed: *mut sys::rs2_device_list, devices_joined: *mut sys::rs2_device_list, data: *mut c_void) 
where
    F: FnMut(&dyn Iterator<Item = Device>, &dyn Iterator<Item = Device>)
{
    let panic = std::panic::catch_unwind(|| {
        if devices_removed.is_null() {
            panic!("empty removed devices");
        }

        if devices_joined.is_null() {
            panic!("empty joined devices");
        }

        let mut err = std::ptr::null_mut::<sys::rs2_error>();

        let removed_len = sys::rs2_get_device_count(devices_removed, &mut err);
        let added_len = sys::rs2_get_device_count(devices_joined, &mut err);

        let removed_devices = core::mem::ManuallyDrop::new(DeviceIter::new(NonNull::new_unchecked(devices_removed), removed_len));
        let added_devices = core::mem::ManuallyDrop::new(DeviceIter::new(NonNull::new_unchecked(devices_joined), added_len));

        if data.is_null() {
            panic!("empty data");
        }

        let f = &mut *(data as *mut F);
        f(&removed_devices, &added_devices);
    });
}
