//! Composite frame type containing all other potential frame types.
//!
//! Each Pipeline produces a synchronized collection of frames for all streams
//! configured for its allocated device. These frames will be presented to the user as
//! a collection via the Composite Frame type.
//!
//! This is typically what is delivered from the pipeline.

use crate::kind::Extension;
use num_traits::ToPrimitive;
use realsense_sys as sys;
use std::{convert::TryFrom, ptr::NonNull};

/// Holds the raw data pointer from an RS2 Composite frame type.
pub struct CompositeFrame {
    /// The raw data pointer from the original rs2 frame
    pub ptr: NonNull<sys::rs2_frame>,
}

impl Drop for CompositeFrame {
    /// Drop the raw pointer stored with this struct whenever it goes out of scope.
    fn drop(&mut self) {
        unsafe {
            sys::rs2_release_frame(self.ptr.as_ptr());
        }
    }
}

impl From<NonNull<sys::rs2_frame>> for CompositeFrame {
    fn from(frame_ptr: NonNull<sys::rs2_frame>) -> Self {
        Self { ptr: frame_ptr }
    }
}

impl CompositeFrame {
    /// Gets the number of individual frames included in the composite frame.
    pub fn count(&self) -> usize {
        unsafe {
            let mut err: *mut sys::rs2_error = std::ptr::null_mut::<sys::rs2_error>();
            let count = sys::rs2_embedded_frames_count(self.ptr.as_ptr(), &mut err);
            if err.as_ref().is_none() {
                count as usize
            } else {
                sys::rs2_free_error(err);
                0
            }
        }
    }

    /// Checks if the Composite frame collection is empty.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Retrieves all frames in the Composite frame collection of a given type.
    ///
    /// # Generic Arguments
    ///
    /// `E` must implement [`Extension`](crate::kind::Extension). Some examples of good types to
    /// use for this are:
    ///
    /// * [`VideoFrame`](crate::frame::VideoFrame)
    /// * [`DepthFrame`](crate::frame::DepthFrame)
    /// * [`DisparityFrame`](crate::frame::DisparityFrame)
    /// * [`PoseFrame`](crate::frame::PoseFrame)
    /// * [`PointsFrame`](crate::frame::PointsFrame)
    ///
    pub fn frames_of_extension<E>(&self) -> Vec<E>
    where
        E: TryFrom<NonNull<sys::rs2_frame>> + Extension,
    {
        let mut frames = Vec::new();
        for i in 0..self.count() {
            unsafe {
                let mut err = std::ptr::null_mut::<sys::rs2_error>();
                let frame_ptr =
                    sys::rs2_extract_frame(self.ptr.as_ptr(), i as std::os::raw::c_int, &mut err);

                if err.as_ref().is_some() {
                    sys::rs2_free_error(err);
                    continue;
                }

                let nonnull_frame_ptr = NonNull::new(frame_ptr).unwrap();

                let is_kind = sys::rs2_is_frame_extendable_to(
                    nonnull_frame_ptr.as_ptr(),
                    E::extension().to_u32().unwrap(),
                    &mut err,
                );

                if err.as_ref().is_none() {
                    if is_kind != 0 {
                        if let Ok(f) = E::try_from(nonnull_frame_ptr) {
                            frames.push(f);
                            // This continue is to skip releasing the frame at the end of the loop.
                            // If the call to try_from above is successful and we can push, then
                            // the frame is owned by the type `E` and we should not release it.
                            continue;
                        }
                    }
                } else {
                    sys::rs2_free_error(err);
                }
                sys::rs2_release_frame(nonnull_frame_ptr.as_ptr());
            }
        }
        frames
    }
}
