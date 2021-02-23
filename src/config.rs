//! Configuration type for [Pipeline](crate::pipeline::Pipeline).

use crate::{
    base::from_path,
    check_rs2_error,
    kind::{Rs2Exception, Rs2Format, Rs2StreamKind},
};
use anyhow::Result;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{ffi::CStr, path::Path, ptr::NonNull};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Could not enable stream. Type: {0}; Reason: {1}")]
    CouldNotEnableStream(Rs2Exception, String),
    #[error("Could not enable all streams. Type: {0}; Reason: {1}")]
    CouldNotEnableAllStreams(Rs2Exception, String),
    #[error("Could not disable stream. Type: {0}; Reason: {1}")]
    CouldNotDisableStream(Rs2Exception, String),
    #[error("Could not disable all streams. Type: {0}; Reason: {1}")]
    CouldNotDisableAllStreams(Rs2Exception, String),
    #[error("Could not enable requested device. Type: {0}; Reason: {1}")]
    CouldNotEnableDevice(Rs2Exception, String),
    #[error("Could not enable recording to file from device. Type: {0}; Reason: {1}")]
    CouldNotEnableRecordingToFile(Rs2Exception, String),
}

/// The pipeline configuration that will be consumed by [Pipeline::start()](crate::pipeline::Pipeline::start).
#[derive(Debug)]
pub struct Config {
    config_ptr: NonNull<sys::rs2_config>,
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_config(self.config_ptr.as_ptr());
        }
    }
}

unsafe impl Send for Config {}

impl Config {
    /// Create an instance.
    pub fn new() -> Self {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let ptr = sys::rs2_create_config(&mut err);

            // Literally the only way this can fail seems to be through C++ make_shared.
            //
            // This would imply that OOM errors are the only possible exceptions here, so I'm
            // inclined to believe that checking the error is a fool's errand: If this call errors
            // there isn't going to be a whole lot you can do.
            //
            // We're better off letting Rust panic due to NonNull::new(ptr).unwrap() than to add
            // any extra logic on top.

            Self {
                config_ptr: NonNull::new(ptr).unwrap(),
            }
        }
    }

    /// Enable data stream with attributes.
    pub fn enable_stream(
        &mut self,
        stream: Rs2StreamKind,
        index: usize,
        width: usize,
        height: usize,
        format: Rs2Format,
        framerate: usize,
    ) -> Result<&mut Self, ConfigurationError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_enable_stream(
                self.config_ptr.as_ptr(),
                stream.to_u32().unwrap(),
                index as i32,
                width as i32,
                height as i32,
                format.to_u32().unwrap(),
                framerate as i32,
                &mut err,
            );
            check_rs2_error!(err, ConfigurationError::CouldNotEnableStream)?;
        };
        Ok(self)
    }

    /// Enable all device streams explicitly.
    pub fn enable_all_streams(&mut self) -> Result<&mut Self, ConfigurationError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_enable_all_stream(self.config_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, ConfigurationError::CouldNotEnableAllStreams)?;
        }
        Ok(self)
    }

    /// Enable device from a serial number.
    pub fn enable_device_from_serial(
        &mut self,
        serial: &CStr,
    ) -> Result<&mut Self, ConfigurationError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_enable_device(self.config_ptr.as_ptr(), serial.as_ptr(), &mut err);
            check_rs2_error!(err, ConfigurationError::CouldNotEnableDevice)?;
        }
        Ok(self)
    }

    /// Enable device from a file path.
    pub fn enable_device_from_file<P>(&mut self, file: P, loop_playback: bool) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = from_path(file)?;
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_enable_device_from_file_repeat_option(
                self.config_ptr.as_ptr(),
                path.as_ptr(),
                loop_playback as i32,
                &mut err,
            );
            check_rs2_error!(err, ConfigurationError::CouldNotEnableDevice)?;
        }
        Ok(self)
    }

    /// Enable recording data streams to file.
    pub fn enable_record_to_file<P>(&mut self, file: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = from_path(file)?;
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_enable_record_to_file(
                self.config_ptr.as_ptr(),
                path.as_ptr(),
                &mut err,
            );
            check_rs2_error!(err, ConfigurationError::CouldNotEnableRecordingToFile)?;
        }
        Ok(self)
    }

    /// Disable data stream by stream index.
    pub fn disable_stream_at_index(
        &mut self,
        stream: Rs2StreamKind,
        index: usize,
    ) -> Result<&mut Self, ConfigurationError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_disable_indexed_stream(
                self.config_ptr.as_ptr(),
                stream.to_u32().unwrap(),
                index as i32,
                &mut err,
            );
            check_rs2_error!(err, ConfigurationError::CouldNotDisableStream)?;
        }
        Ok(self)
    }

    /// Disable data stream by stream kind.
    pub fn disable_stream(
        &mut self,
        stream: Rs2StreamKind,
    ) -> Result<&mut Self, ConfigurationError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_disable_stream(
                self.config_ptr.as_ptr(),
                stream.to_u32().unwrap(),
                &mut err,
            );
            check_rs2_error!(err, ConfigurationError::CouldNotDisableStream)?;
        }
        Ok(self)
    }

    /// Disable all device streams explicitly.
    pub fn disable_all_streams(&mut self) -> Result<&mut Self> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_config_disable_all_streams(self.config_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, ConfigurationError::CouldNotDisableAllStreams)?;
        }
        Ok(self)
    }

    // /// Enable all device streams explicitly.
    // pub fn resolve<S>(&self, pipeline: &Pipeline<S>) -> Result<PipelineProfile>
    // where
    //     S: PipelineState,
    // {
    //     let profile = unsafe {
    //         let mut checker = ErrorChecker::new();
    //         let ptr = sys::rs2_config_resolve(
    //             self.ptr.as_ptr(),
    //             pipeline.ptr.as_ptr(),
    //             checker.inner_mut_ptr(),
    //         );
    //         checker.check()?;
    //         PipelineProfile::from_raw(ptr)
    //     };
    //     Ok(profile)
    // }

    pub unsafe fn get_raw(&self) -> NonNull<sys::rs2_config> {
        self.config_ptr
    }
}
