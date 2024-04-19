use super::{inactive::InactivePipeline, profile::PipelineProfile};
use crate::{check_rs2_error, frame::CompositeFrame, kind::Rs2Exception};
use anyhow::Result;
use realsense_sys as sys;
use std::{ptr::NonNull, task::Poll, time::Duration};
use thiserror::Error;
use std::os::raw::c_void;
use crate::frame::FrameCategory;
use super::inactive::IntoFrame;

pub(crate) unsafe extern "C" fn trampoline<F>(frame: *mut sys::rs2_frame, data: *mut c_void) 
where
    F: FnMut(&impl IntoFrame) + Send + 'static,
{
    let panic = std::panic::catch_unwind(|| {
        if frame.is_null() {
            panic!("null frame");
        }

        let frame = core::mem::ManuallyDrop::new(NonNull::new_unchecked(frame));

        if data.is_null() {
            panic!("empty data");
        }

        let f = &mut *(data as *mut F);

        f(&frame);
    });

    if panic.is_err() {
        eprintln!("Callback function panicked");
        std::process::abort();
    }
}

pub struct StreamingPipeline {
    /// A pointer to the callback function for the pipeline.
    callback: *mut dyn FnMut(&impl IntoFrame),
    /// A (non-null) pointer to the pipeline.
    pipeline_ptr: NonNull<sys::rs2_pipeline>,
    /// The pipeline's profile, which contains the device the pipeline is configured for alongside
    /// the stream profiles for streams in the pipeline.
    profile: PipelineProfile,
}

impl StreamingPipeline {
    /// Constructs a new streaming pipeline from the constituent components
    ///
    /// This is only to be used / called from the [`InactivePipeline`] type.
    pub(crate) fn new<F>(pipeline_ptr: NonNull<sys::rs2_pipeline>, profile: PipelineProfile, callback: F) -> Self 
        where
            F: FnMut(&impl IntoFrame) + Send + 'static
    {
        Self {
            pipeline_ptr,
            callback,
            profile,
        }
    }

    /// Gets the active profile of pipeline.
    pub fn profile(&self) -> &PipelineProfile {
        &self.profile
    }

    /// Stop the pipeline.
    ///
    /// This method consumes the pipeline instance and returns pipeline markered inactive.
    pub fn stop(self) -> InactivePipeline {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            // The only "error" that can occur here is if the pipeline pointer is null.
            //
            // We know it is not (state is managed so that this isn't a possibility, and we use
            // `NonNull` to try and guarantee that even beyond our state management), so there
            // dealing with the error (and thus returning a result type) is superfluous here.
            sys::rs2_pipeline_stop(self.pipeline_ptr.as_ptr(), &mut err);

            let inactive = InactivePipeline::new(self.pipeline_ptr);

            std::mem::forget(self);
            inactive
        }
    }

}

impl Drop for StreamingPipeline {
    fn drop(&mut self) {
        let boxed = unsafe { Box::from_raw(self.pipeline_ptr.as_ptr()) };
        unsafe { sys::rs2_delete_pipeline(self.pipeline_ptr.as_ptr()) };
    }
}
