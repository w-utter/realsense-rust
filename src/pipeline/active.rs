//! Defines the pipeline type.

use super::{inactive::InactivePipeline, profile::PipelineProfile};
use crate::{
    base::DEFAULT_TIMEOUT, check_rs2_error, config::Config, context::Context,
    frame::CompositeFrame, kind::Rs2Exception,
};
use anyhow::Result;
use realsense_sys as sys;
use std::{convert::TryFrom, ptr::NonNull};
use thiserror::Error;

pub struct ActivePipeline<'a> {
    pipeline_ptr: NonNull<sys::rs2_pipeline>,
    profile: PipelineProfile<'a>,
    context: &'a Context,
}

impl<'a> Drop for ActivePipeline<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_pipeline(self.pipeline_ptr.as_ptr());
        }
    }
}

impl<'a> ActivePipeline<'a> {
    /// Constructs a new active pipeline from the constituent components
    ///
    /// This is only to be used / called from the [`InactivePipeline`] type.
    pub(crate) fn new(
        pipeline_ptr: NonNull<sys::rs2_pipeline>,
        profile: PipelineProfile<'a>,
        context: &'a Context,
    ) -> Self {
        Self {
            pipeline_ptr,
            profile,
            context,
        }
    }

    /// Gets the active profile of pipeline.
    pub fn profile(&'a self) -> &'a PipelineProfile<'a> {
        &self.profile
    }

    /// Stop the pipeline.
    ///
    /// This method consumes the pipeline instance and returns pipeline markered inactive.
    pub fn stop(self) -> InactivePipeline<'a> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            // The only "error" that can occur here is if the pipeline pointer is null.
            //
            // We know it is not (state is managed so that this isn't a possibility, and we use
            // `NonNull` to try and guarantee that even beyond our state management), so there
            // dealing with the error (and thus returning a result type) is superfluous here.
            sys::rs2_pipeline_stop(self.pipeline_ptr.as_ptr(), &mut err);

            let inactive = InactivePipeline::new(self.pipeline_ptr, self.context);

            std::mem::forget(self);
            inactive
        }
    }

    // /// Block until the next frame is available.
    // ///
    // /// When the timeout is set, it returns `Ok(Some(frame))` if the frame is available,
    // /// or returns `Ok(None)` when timeout occurs.
    // ///
    // /// If the timeout is `None`, it waits indefinitely before the next frame.
    // pub fn wait(&mut self, timeout: impl Into<Option<Duration>>) -> Result<CompositeFrame> {
    //     let timeout = timeout.into();
    //     let timeout_ms = timeout.unwrap_or(DEFAULT_TIMEOUT).as_millis() as c_uint;

    //     let frame = loop {
    //         let mut checker = ErrorChecker::new();
    //         let ptr = unsafe {
    //             sys::rs2_pipeline_wait_for_frames(
    //                 self.ptr.as_ptr(),
    //                 timeout_ms,
    //                 checker.inner_mut_ptr(),
    //             )
    //         };

    //         match (timeout, checker.check()) {
    //             (None, Err(RsError::Timeout(_))) => continue,
    //             (Some(_), Err(RsError::Timeout(_))) => {
    //                 return Ok(None);
    //             }
    //             (_, result) => result?,
    //         }

    //         let frame = unsafe { Frame::from_raw(ptr) };
    //         break frame;
    //     };

    //     Ok(Some(frame))
    // }

    /// Poll if next frame is immediately available.
    ///
    /// Unlike [Pipeline::start], the method does not block and returns None
    /// if next from is not available.
    pub fn poll(&mut self) -> Result<Option<CompositeFrame>> {
        unimplemented!();
        // unsafe {
        //     let mut checker = ErrorChecker::new();
        //     let mut ptr: *mut sys::rs2_frame = ptr::null_mut();
        //     let ret = sys::rs2_pipeline_poll_for_frames(
        //         self.ptr.as_ptr(),
        //         &mut ptr as *mut _,
        //         checker.inner_mut_ptr(),
        //     );

        //     if let Err(err) = checker.check() {
        //         return Err(err);
        //     }

        //     if ret != 0 {
        //         let frame = Frame::from_raw(ptr);
        //         Ok(Some(frame))
        //     } else {
        //         Ok(None)
        //     }
        // }
    }
}
