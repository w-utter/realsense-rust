use super::{active::ActivePipeline, profile::PipelineProfile};
use crate::{check_rs2_error, config::Config, context::Context, kind::Rs2Exception};
use anyhow::Result;
use realsense_sys as sys;
use std::{convert::TryFrom, ptr::NonNull};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineConstructionError {
    #[error("Could not create the pipeline from the provided context. Type: {0}; Reason {1}")]
    CouldNotCreatePipelineFromContext(Rs2Exception, String),
}

#[derive(Error, Debug)]
#[error("Could not successfully start the pipeline. Type: {0}; Reason: {1}")]
pub struct CouldNotStartPipelineError(pub Rs2Exception, pub String);

pub struct InactivePipeline<'a> {
    pipeline_ptr: NonNull<sys::rs2_pipeline>,
    context: &'a Context,
}

impl<'a> Drop for InactivePipeline<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_pipeline(self.pipeline_ptr.as_ptr());
        }
    }
}

unsafe impl<'a> Send for InactivePipeline<'a> {}

impl<'a> TryFrom<&'a Context> for InactivePipeline<'a> {
    type Error = anyhow::Error;

    fn try_from(context: &'a Context) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let context_ptr = context.get_raw();

            let pipeline_ptr = sys::rs2_create_pipeline(context_ptr.as_ptr(), &mut err);
            check_rs2_error!(
                err,
                PipelineConstructionError::CouldNotCreatePipelineFromContext
            )?;

            Ok(Self {
                pipeline_ptr: NonNull::new(pipeline_ptr).unwrap(),
                context,
            })
        }
    }
}

impl<'a> InactivePipeline<'a> {
    /// Constructs a new inactive pipeline from the constituent components
    ///
    /// This is only to be used / called from the [`ActivePipeline`] type.
    pub(crate) fn new(pipeline_ptr: NonNull<sys::rs2_pipeline>, context: &'a Context) -> Self {
        Self {
            pipeline_ptr,
            context,
        }
    }

    /// Start the pipeline with an optional config.
    ///
    /// The method consumes inactive pipeline itself, and returns the started pipeine.
    pub fn start(self, config: Option<&'a Config>) -> Result<ActivePipeline<'a>> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let profile_ptr = if let Some(conf) = config {
                sys::rs2_pipeline_start_with_config(
                    self.pipeline_ptr.as_ptr(),
                    conf.get_raw().as_ptr(),
                    &mut err,
                )
            } else {
                sys::rs2_pipeline_start(self.pipeline_ptr.as_ptr(), &mut err)
            };
            check_rs2_error!(err, CouldNotStartPipelineError)?;

            let profile = PipelineProfile::try_from(NonNull::new(profile_ptr).unwrap())?;
            let active = ActivePipeline::new(self.pipeline_ptr, profile, self.context);

            std::mem::forget(self);
            Ok(active)
        }
    }

    pub fn resolve(&self, config: &'a Config) -> Option<PipelineProfile> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let profile_ptr = sys::rs2_config_resolve(
                config.get_raw().as_ptr(),
                self.pipeline_ptr.as_ptr(),
                &mut err,
            );

            if let Some(nonnull_profile) = NonNull::new(profile_ptr) {
                if let Ok(profile) = PipelineProfile::try_from(nonnull_profile) {
                    return Some(profile);
                }
            }

            // If we get here, then we've failed some important checks.
            None
        }
    }

    pub fn can_resolve(&self, config: &'a Config) -> bool {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let can_resolve = sys::rs2_config_can_resolve(
                config.get_raw().as_ptr(),
                self.pipeline_ptr.as_ptr(),
                &mut err,
            );
            can_resolve != 0
        }
    }
}
