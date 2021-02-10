//! Composite frame type containing all other potential frame types
//!
//! This is what is typically delivered by the pipeline.

use crate::{common::*, kind::Kind};
use num_traits::ToPrimitive;

pub struct CompositeFrame {
    pub(crate) ptr: NonNull<sys::rs2_frame>,
}

impl CompositeFrame {
    /// Gets the number of frames included in the composite frame.
    pub fn count(&self) -> usize {
        let count = unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();

            let count = sys::rs2_embedded_frames_count(self.ptr.as_ptr(), &mut err);

            if let Some(_) = NonNull::new(err) {
                0
            } else {
                count as usize
            }
        };
        count
    }

    /// Checks if the composite-frame contains no sub-frames.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn frames_of_kind<K>(&self) -> Vec<K>
    where
        K: std::convert::TryFrom<NonNull<sys::rs2_frame>> + Kind,
    {
        let mut frames = Vec::new();
        for i in 0..self.count() {
            let ptr = unsafe {
                let mut err: *mut sys::rs2_error = ptr::null_mut();
                let ptr = sys::rs2_extract_frame(self.ptr.as_ptr(), i as c_int, &mut err);

                if NonNull::new(err).is_some() {
                    None
                } else {
                    NonNull::new(ptr)
                }
            };

            if let Some(ptr) = ptr {
                unsafe {
                    let mut err: *mut sys::rs2_error = ptr::null_mut();
                    let is_kind = sys::rs2_is_frame_extendable_to(
                        ptr.as_ptr(),
                        K::extension().to_u32().unwrap(),
                        &mut err,
                    );
                    if NonNull::new(err).is_none() && is_kind != 0 {
                        if let Ok(f) = K::try_from(ptr) {
                            frames.push(f);
                        }
                    }
                }
            }
        }
        frames
    }
}

impl Drop for CompositeFrame {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_release_frame(self.ptr.as_ptr());
        }
    }
}
