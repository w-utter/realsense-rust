//! Defines the pipeline type.

use super::{inactive::InactivePipeline, profile::PipelineProfile};
use crate::{
    base::DEFAULT_TIMEOUT, check_rs2_error, config::Config, context::Context,
    frame::CompositeFrame, kind::Rs2Exception,
};
use anyhow::Result;
use realsense_sys as sys;
use std::{convert::TryFrom, mem::MaybeUninit, ptr::NonNull, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrameWaitError {
    /// librealsense2 had an internal error occur while waiting for frames.
    #[error("An internal error occurred while waiting for frames. Type: {0}; Reason: {1}")]
    DidErrorDuringFrameWait(Rs2Exception, String),
    /// The associated function timed out while waiting for frames.
    #[error("Timed out while waiting for frame.")]
    DidTimeoutBeforeFrameArrival,
}

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

    /// Waits to get a new composite frame, blocking the calling thread.
    ///
    /// Returns a composite frame from the pipeline, blocking the calling thread until a frame is
    /// available. This method can return an error if an internal exception occurs or if the thread
    /// waits more than the duration provided by `timeout_ms` (in milliseconds).
    ///
    /// # Errors
    ///
    /// Returns [`FrameWaitError::DidErrorDuringFrameWait`] if an internal error occurs while
    /// waiting for next frame(s).
    ///
    /// Returns [`FrameWaitError::DidTimeoutBeforeFrameArrival`] if the thread waits more than
    /// `timeout_ms` (in milliseconds) without returning a frame.
    ///
    pub fn wait(&mut self, timeout_ms: Option<Duration>) -> Result<CompositeFrame> {
        let timeout_ms = match timeout_ms {
            Some(d) => d.as_millis() as u32,
            None => sys::RS2_DEFAULT_TIMEOUT,
        };

        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let mut frame = std::ptr::null_mut::<sys::rs2_frame>();

            let did_get_frame = sys::rs2_pipeline_try_wait_for_frames(
                self.pipeline_ptr.as_ptr(),
                &mut frame,
                timeout_ms,
                &mut err,
            );
            check_rs2_error!(err, FrameWaitError::DidErrorDuringFrameWait)?;

            if !did_get_frame {
                return Err(FrameWaitError::DidTimeoutBeforeFrameArrival);
            }

            Ok(CompositeFrame::from(NonNull::new(frame).unwrap()))
        }
    }

    /// Poll if next frame is immediately available.
    ///
    /// Unlike [Pipeline::start], the method does not block and returns None
    /// if the next frame is not available.
    pub fn poll(&mut self) -> Option<CompositeFrame> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let mut frame_ptr = std::ptr::null_mut::<sys::rs2_frame>();
            let was_stored = sys::rs2_pipeline_poll_for_frames(
                self.pipeline_ptr.as_ptr(),
                &mut frame_ptr,
                &mut err,
            );

            if was_stored == 0 {
                return None;
            }

            if let Some(nonnull_frame) = NonNull::new(frame_ptr) {
                Some(CompositeFrame::from(nonnull_frame))
            } else {
                None
            }
        }
    }
}
