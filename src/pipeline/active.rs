//! Defines the pipeline type.

use super::profile::PipelineProfile;
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

    // /// Block until the next frame is available.
    // ///
    // /// When the timeout is set, it returns `Ok(Some(frame))` if the frame is available,
    // /// or returns `Ok(None)` when timeout occurs.
    // ///
    // /// If the timeout is `None`, it waits indefinitely before the next frame.
    // pub fn wait(&mut self, timeout: impl Into<Option<Duration>>) -> Result<Option<CompositeFrame>> {
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

    // /// Poll if next frame is immediately available.
    // ///
    // /// Unlike [Pipeline::start], the method does not block and returns None
    // /// if next from is not available.
    // pub fn try_wait(&mut self) -> Result<Option<CompositeFrame>> {
    //     unsafe {
    //         let mut checker = ErrorChecker::new();
    //         let mut ptr: *mut sys::rs2_frame = ptr::null_mut();
    //         let ret = sys::rs2_pipeline_poll_for_frames(
    //             self.ptr.as_ptr(),
    //             &mut ptr as *mut _,
    //             checker.inner_mut_ptr(),
    //         );

    //         if let Err(err) = checker.check() {
    //             return Err(err);
    //         }

    //         if ret != 0 {
    //             let frame = Frame::from_raw(ptr);
    //             Ok(Some(frame))
    //         } else {
    //             Ok(None)
    //         }
    //     }
    // }

    // /// Wait for the next frame asynchronously.
    // ///
    // /// The method is analogous to [Pipeline::wait].
    // ///
    // /// When the timeout is set, it returns `Ok(Some(frame))` if the frame is available,
    // /// or returns `Ok(None)` when timeout occurs.
    // ///
    // /// If the timeout is `None`, it waits indefinitely before the next frame.
    // pub async fn wait_async(
    //     &mut self,
    //     timeout: impl Into<Option<Duration>>,
    // ) -> Result<Option<CompositeFrame>> {
    //     let timeout = timeout.into();
    //     let timeout_ms = timeout
    //         .map(|duration| duration.as_millis() as c_uint)
    //         .unwrap_or(sys::RS2_DEFAULT_TIMEOUT as c_uint);
    //     let (tx, rx) = futures::channel::oneshot::channel();
    //     let pipeline_ptr = AtomicPtr::new(self.ptr.as_ptr());

    //     thread::spawn(move || {
    //         let result = unsafe {
    //             loop {
    //                 let mut checker = ErrorChecker::new();
    //                 let ptr = sys::rs2_pipeline_wait_for_frames(
    //                     pipeline_ptr.load(Ordering::Relaxed),
    //                     timeout_ms,
    //                     checker.inner_mut_ptr(),
    //                 );
    //                 let result = match (timeout, checker.check()) {
    //                     (None, Err(RsError::Timeout(_))) => continue,
    //                     (Some(_), Err(RsError::Timeout(_))) => Ok(None),
    //                     (_, result) => result.map(|_| Some(Frame::from_raw(ptr))),
    //                 };
    //                 break result;
    //             }
    //         };
    //         let _ = tx.send(result);
    //     });

    //     let frame = rx.await.unwrap()?;
    //     Ok(frame)
    // }

    // /// Stop the pipeline.
    // ///
    // /// This method consumes the pipeline instance and returns pipeline markered inactive.
    // pub fn stop(self) -> Result<InactivePipeline> {
    //     unsafe {
    //         let mut checker = ErrorChecker::new();
    //         sys::rs2_pipeline_stop(self.ptr.as_ptr(), checker.inner_mut_ptr());
    //         checker.check()?;
    //     }

    //     let pipeline = {
    //         let (pipeline_ptr, context_ptr, profile_ptr) = self.into_raw_parts();

    //         mem::drop(unsafe { pipeline_kind::Active::from_raw_parts(profile_ptr) });

    //         Pipeline {
    //             ptr: NonNull::new(pipeline_ptr).unwrap(),
    //             context: unsafe { Context::from_raw(context_ptr) },
    //             state: pipeline_kind::Inactive,
    //         }
    //     };

    //     Ok(pipeline)
    // }

    // /// Unpack the pipeline into raw pointers.
    // ///
    // /// After calling this method, you have to take care of their lifetime manually.
    // pub fn into_raw_parts(
    //     self,
    // ) -> (
    //     *mut sys::rs2_pipeline,
    //     *mut sys::rs2_context,
    //     *mut sys::rs2_pipeline_profile,
    // ) {
    //     // take fields without invoking drop()
    //     let ptr = self.ptr;
    //     let context = unsafe { self.context.unsafe_clone().into_raw() };
    //     let pipeline_profile = unsafe { self.state.unsafe_clone().into_raw_parts() };
    //     mem::forget(self);
    //     (ptr.as_ptr(), context, pipeline_profile)
    // }

    // /// Construct an active pipeline from raw pointers.
    // ///
    // /// It assumes the pipeline pointer is built from the context pointer, and profile pointer
    // /// is the active profile of the pipeline.
    // pub unsafe fn from_raw_parts(
    //     pipeline_ptr: *mut sys::rs2_pipeline,
    //     context_ptr: *mut sys::rs2_context,
    //     profile_ptr: *mut sys::rs2_pipeline_profile,
    // ) -> Self {
    //     let context = Context::from_raw(context_ptr);
    //     let state = pipeline_kind::Active::from_raw_parts(profile_ptr);
    //     Self {
    //         ptr: NonNull::new(pipeline_ptr).unwrap(),
    //         context,
    //         state,
    //     }
    // }
}
