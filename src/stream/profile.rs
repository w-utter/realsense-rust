//! Type for representing stream information (format, etc)

use crate::{
    check_rs2_error,
    kind::{Rs2Format, Rs2StreamKind},
};
use anyhow::Result;
use num_traits::FromPrimitive;
use realsense_sys as sys;
use std::{marker::PhantomData, mem::MaybeUninit, ptr::NonNull};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamConstructionError {
    #[error("Could not retrieve stream data. Reason: {0}")]
    CouldNotRetrieveStreamData(String),
    #[error("Could not determine if this is the default stream. Reason: {0}")]
    CouldNotDetermineIsDefault(String),
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Could not get extrinsics. Reason: {0}")]
    CouldNotGetExtrinsics(String),
    #[error("Could not set extrinsics. Reason: {0}")]
    CouldNotSetExtrinsics(String),
    #[error("Stream does not have video intrinsics")]
    StreamDoesNotHaveVideoIntrinsics,
    #[error("Stream does not have motion intrinsics")]
    StreamDoesNotHaveMotionIntrinsics,
    #[error("Could not get video intrinsics. Reason: {0}")]
    CouldNotGetIntrinsics(String),
    #[error("Could not get motion intrinsics. Reason: {0}")]
    CouldNotGetMotionIntrinsics(String),
}

pub struct StreamProfile<'a> {
    // TODO: describe why dropping this pointer is a BAD IDEA (TM)
    // See: docs for rs2_delete_stream_profile
    ptr: NonNull<sys::rs2_stream_profile>,
    stream: Rs2StreamKind,
    format: Rs2Format,
    index: usize,
    unique_id: i32,
    framerate: i32,
    is_default: bool,
    // TODO: describe why this is necessary
    _phantom: PhantomData<&'a ()>,
}

impl<'a> std::convert::TryFrom<NonNull<sys::rs2_stream_profile>> for StreamProfile<'a> {
    type Error = StreamConstructionError;

    fn try_from(stream_profile_ptr: NonNull<sys::rs2_stream_profile>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();

            let mut stream = MaybeUninit::uninit();
            let mut format = MaybeUninit::uninit();
            let mut index = MaybeUninit::uninit();
            let mut unique_id = MaybeUninit::uninit();
            let mut framerate = MaybeUninit::uninit();

            sys::rs2_get_stream_profile_data(
                stream_profile_ptr.as_ptr(),
                stream.as_mut_ptr(),
                format.as_mut_ptr(),
                index.as_mut_ptr(),
                unique_id.as_mut_ptr(),
                framerate.as_mut_ptr(),
                &mut err,
            );
            check_rs2_error!(err, StreamConstructionError::CouldNotRetrieveStreamData)?;

            let is_default =
                sys::rs2_is_stream_profile_default(stream_profile_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, StreamConstructionError::CouldNotDetermineIsDefault)?;

            Ok(StreamProfile {
                ptr: stream_profile_ptr,
                stream: Rs2StreamKind::from_u32(stream.assume_init()).unwrap(),
                format: Rs2Format::from_u32(format.assume_init()).unwrap(),
                index: index.assume_init() as usize,
                unique_id: unique_id.assume_init(),
                framerate: framerate.assume_init(),
                is_default: is_default != 0,
                _phantom: PhantomData {},
            })
        }
    }
}

impl<'a> StreamProfile<'a> {
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    pub fn stream(&self) -> Rs2StreamKind {
        self.stream
    }

    pub fn format(&self) -> Rs2Format {
        self.format
    }

    pub fn stream_index(&self) -> usize {
        self.index
    }

    pub fn unique_id(&self) -> i32 {
        self.unique_id
    }

    pub fn framerate(&self) -> i32 {
        self.framerate
    }

    pub fn get_extrinsics(
        &self,
        to_profile: &StreamProfile,
    ) -> Result<sys::rs2_extrinsics, DataError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let mut extrinsics = MaybeUninit::<sys::rs2_extrinsics>::uninit();

            sys::rs2_get_extrinsics(
                self.ptr.as_ptr(),
                to_profile.ptr.as_ptr(),
                extrinsics.as_mut_ptr(),
                &mut err,
            );
            check_rs2_error!(err, DataError::CouldNotGetExtrinsics)?;

            Ok(extrinsics.assume_init())
        }
    }

    pub fn set_extrinsics(
        &self,
        to_profile: &StreamProfile,
        extrinsics: sys::rs2_extrinsics,
    ) -> Result<(), DataError> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            sys::rs2_register_extrinsics(
                self.ptr.as_ptr(),
                to_profile.ptr.as_ptr(),
                extrinsics,
                &mut err,
            );
            check_rs2_error!(err, DataError::CouldNotSetExtrinsics)?;

            Ok(())
        }
    }

    pub fn intrinsics(&self) -> Result<sys::rs2_intrinsics, DataError> {
        match self.stream {
            Rs2StreamKind::Depth => (),
            Rs2StreamKind::Color => (),
            Rs2StreamKind::Infrared => (),
            Rs2StreamKind::Fisheye => (),
            _ => {
                return Err(DataError::StreamDoesNotHaveVideoIntrinsics);
            }
        }
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let mut intrinsics = MaybeUninit::<sys::rs2_intrinsics>::uninit();

            sys::rs2_get_video_stream_intrinsics(
                self.ptr.as_ptr(),
                intrinsics.as_mut_ptr(),
                &mut err,
            );
            check_rs2_error!(err, DataError::CouldNotGetIntrinsics)?;

            Ok(intrinsics.assume_init())
        }
    }

    pub fn motion_intrinsics(&self) -> Result<sys::rs2_motion_device_intrinsic, DataError> {
        match self.stream {
            Rs2StreamKind::Gyro => (),
            Rs2StreamKind::Accel => (),
            _ => {
                return Err(DataError::StreamDoesNotHaveMotionIntrinsics);
            }
        }
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let mut intrinsics = MaybeUninit::<sys::rs2_motion_device_intrinsic>::uninit();

            sys::rs2_get_motion_intrinsics(self.ptr.as_ptr(), intrinsics.as_mut_ptr(), &mut err);
            check_rs2_error!(err, DataError::CouldNotGetMotionIntrinsics)?;

            Ok(intrinsics.assume_init())
        }
    }
}
