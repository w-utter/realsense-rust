//! Type for representing a points frame

use super::prelude::{FrameConstructionError, FrameEx, PointsFrameEx};
use crate::{
    check_rs2_error,
    common::*,
    kind::{Extension, Rs2Extension},
    stream::StreamProfile,
};
use anyhow::Result;

pub struct PointsFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    frame_stream_profile: StreamProfile<'a>,
    num_points: usize,
    vertices_data_ptr: NonNull<sys::rs2_vertex>,
    texture_data_ptr: NonNull<sys::rs2_pixel>,
    should_drop: bool,
}

impl<'a> Extension for PointsFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::Points
    }
}

impl<'a> FrameEx<'a> for PointsFrame<'a> {
    fn profile(&'a self) -> &'a StreamProfile<'a> {
        &self.frame_stream_profile
    }

    unsafe fn get_owned_frame_ptr(mut self) -> NonNull<sys::rs2_frame> {
        self.should_drop = false;

        self.frame_ptr
    }
}

impl<'a> Drop for PointsFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            if self.should_drop {
                // Vertices and Texture pointer lifetimes are managed by the
                // frame, so dropping the frame should suffice.
                sys::rs2_release_frame(self.frame_ptr.as_ptr());
            }
        }
    }
}

unsafe impl<'a> Send for PointsFrame<'a> {}

impl<'a> std::convert::TryFrom<NonNull<sys::rs2_frame>> for PointsFrame<'a> {
    type Error = anyhow::Error;

    fn try_from(frame_ptr: NonNull<sys::rs2_frame>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = StreamProfile::try_from(nonnull_profile_ptr)?;

            let num_points = sys::rs2_get_frame_points_count(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetPointCount)?;

            let vertices_ptr = sys::rs2_get_frame_vertices(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            let texture_ptr = sys::rs2_get_frame_texture_coordinates(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            Ok(PointsFrame {
                frame_ptr,
                frame_stream_profile: profile,
                num_points: num_points as usize,
                vertices_data_ptr: NonNull::new(vertices_ptr).unwrap(),
                texture_data_ptr: NonNull::new(texture_ptr).unwrap(),
                should_drop: true,
            })
        }
    }
}

impl<'a> PointsFrameEx<'a> for PointsFrame<'a> {
    fn vertices(&'a self) -> &'a [sys::rs2_vertex] {
        unsafe {
            slice::from_raw_parts::<sys::rs2_vertex>(
                self.vertices_data_ptr.as_ptr(),
                self.num_points,
            )
        }
    }

    // SAFETY:
    // The librealsense2 C++ API directly casts the rs2_pixel* returned from
    // rs2_get_frame_texture_coordinates() into a texture_coordinate*, thereby
    // re-interpreting [[c_int; 2]; N] as [[c_float; 2]; N] values.
    // Note that C does not generally guarantee that sizeof(int) == sizeof(float).
    fn texture_coordinates(&'a self) -> &'a [[f32; 2]] {
        unsafe {
            slice::from_raw_parts::<[f32; 2]>(
                self.texture_data_ptr.as_ptr().cast::<[f32; 2]>(),
                self.num_points,
            )
        }
    }
    fn points_count(&self) -> usize {
        self.num_points
    }
}
