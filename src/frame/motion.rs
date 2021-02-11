//! Type for representing a motion frame

use super::prelude::{FrameConstructionError, MotionFrameEx};
use crate::{
    check_rs2_error,
    common::*,
    kind::{Extension, Rs2Extension},
    stream::StreamProfile,
};
use anyhow::Result;

pub struct MotionFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    frame_stream_profile: StreamProfile<'a>,
    data_size_in_bytes: usize,
    motion: [f32; 3],
    // data: &'a std::os::raw::c_void,
}

impl<'a> Extension for MotionFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::MotionFrame
    }
}

impl<'a> Drop for MotionFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_release_frame(self.frame_ptr.as_ptr());
        }
    }
}

unsafe impl<'a> Send for MotionFrame<'a> {}

impl<'a> std::convert::TryFrom<NonNull<sys::rs2_frame>> for MotionFrame<'a> {
    type Error = anyhow::Error;

    fn try_from(frame_ptr: NonNull<sys::rs2_frame>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();
            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = StreamProfile::try_from(nonnull_profile_ptr)?;

            let size = sys::rs2_get_frame_data_size(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetDataSize)?;

            let ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            let data_as_ptr = ptr.as_ref().unwrap() as *const std::os::raw::c_void;
            let motion_raw = std::slice::from_raw_parts(
                data_as_ptr.cast::<f32>(),
                size as usize / std::mem::size_of::<f32>(),
            );

            Ok(MotionFrame {
                frame_ptr,
                frame_stream_profile: profile,
                data_size_in_bytes: size as usize,
                motion: [motion_raw[0], motion_raw[1], motion_raw[2]],
            })
        }
    }
}

impl<'a> MotionFrameEx<'a> for MotionFrame<'a> {
    fn motion(&self) -> &[f32; 3] {
        &self.motion
    }
}
