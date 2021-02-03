//! Type for representing stream information (format, etc)

use crate::common::*;

pub enum StreamConstructionError {
    FailedToRetrieveStreamData(String),
    FailedToDetermineIsDefault(String),
}

pub enum DataError {
    FailedToGetExtrinsics(String),
    FailedToSetExtrinsics(String),
    StreamDoesNotHaveVideoIntrinsics,
    StreamDoesNotHaveMotionIntrinsics,
    FailedToGetIntrinsics(String),
    FailedToGetMotionIntrinsics(String),
}

pub struct Profile {
    ptr: NonNull<sys::rs2_stream_profile>,
    stream: sys::rs2_stream,
    format: sys::rs2_format,
    index: usize,
    unique_id: i32,
    framerate: i32,
    is_default: bool,
}

impl Profile {
    pub(crate) fn new(
        profile: NonNull<sys::rs2_stream_profile>,
    ) -> std::result::Result<Self, StreamConstructionError> {
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();

            let mut stream = MaybeUninit::uninit();
            let mut format = MaybeUninit::uninit();
            let mut index = MaybeUninit::uninit();
            let mut unique_id = MaybeUninit::uninit();
            let mut framerate = MaybeUninit::uninit();

            sys::rs2_get_stream_profile_data(
                profile.as_ptr(),
                stream.as_mut_ptr(),
                format.as_mut_ptr(),
                index.as_mut_ptr(),
                unique_id.as_mut_ptr(),
                framerate.as_mut_ptr(),
                &mut err,
            );

            if NonNull::new(err).is_some() {
                return Err(StreamConstructionError::FailedToRetrieveStreamData(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));
            }

            let is_default = sys::rs2_is_stream_profile_default(profile.as_ptr(), &mut err);

            if NonNull::new(err).is_some() {
                Err(StreamConstructionError::FailedToDetermineIsDefault(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(Profile {
                    ptr: profile,
                    stream: stream.assume_init(),
                    format: format.assume_init(),
                    index: index.assume_init() as usize,
                    unique_id: unique_id.assume_init(),
                    framerate: framerate.assume_init(),
                    is_default: is_default != 0,
                })
            }
        }
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }

    pub fn stream(&self) -> sys::rs2_stream {
        self.stream
    }

    pub fn format(&self) -> sys::rs2_format {
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
        to_profile: &Profile,
    ) -> std::result::Result<sys::rs2_extrinsics, DataError> {
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();
            let mut extrinsics = MaybeUninit::<sys::rs2_extrinsics>::uninit();

            sys::rs2_get_extrinsics(
                self.ptr.as_ptr(),
                to_profile.ptr.as_ptr(),
                extrinsics.as_mut_ptr(),
                &mut err,
            );

            if NonNull::new(err).is_some() {
                Err(DataError::FailedToGetExtrinsics(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(extrinsics.assume_init())
            }
        }
    }

    pub fn set_extrinsics(
        &self,
        to_profile: &Profile,
        extrinsics: sys::rs2_extrinsics,
    ) -> Result<(), DataError> {
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();
            sys::rs2_register_extrinsics(
                self.ptr.as_ptr(),
                to_profile.ptr.as_ptr(),
                extrinsics,
                &mut err,
            );
            if NonNull::new(err).is_some() {
                Err(DataError::FailedToSetExtrinsics(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    pub fn intrinsics(&self) -> std::result::Result<sys::rs2_intrinsics, DataError> {
        match self.stream {
            sys::rs2_stream_RS2_STREAM_DEPTH => (),
            sys::rs2_stream_RS2_STREAM_COLOR => (),
            sys::rs2_stream_RS2_STREAM_INFRARED => (),
            sys::rs2_stream_RS2_STREAM_FISHEYE => (),
            _ => {
                return Err(DataError::StreamDoesNotHaveVideoIntrinsics);
            }
        }
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();
            let mut intrinsics = MaybeUninit::<sys::rs2_intrinsics>::uninit();

            sys::rs2_get_video_stream_intrinsics(
                self.ptr.as_ptr(),
                intrinsics.as_mut_ptr(),
                &mut err,
            );

            if NonNull::new(err).is_some() {
                Err(DataError::FailedToGetIntrinsics(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(intrinsics.assume_init())
            }
        }
    }

    pub fn motion_intrinsics(
        &self,
    ) -> std::result::Result<sys::rs2_motion_device_intrinsic, DataError> {
        match self.stream {
            sys::rs2_stream_RS2_STREAM_GYRO => (),
            sys::rs2_stream_RS2_STREAM_ACCEL => (),
            _ => {
                return Err(DataError::StreamDoesNotHaveMotionIntrinsics);
            }
        }
        unsafe {
            let mut err: *mut sys::rs2_error = ptr::null_mut();
            let mut intrinsics = MaybeUninit::<sys::rs2_motion_device_intrinsic>::uninit();
            sys::rs2_get_motion_intrinsics(self.ptr.as_ptr(), intrinsics.as_mut_ptr(), &mut err);
            if NonNull::new(err).is_some() {
                Err(DataError::FailedToGetMotionIntrinsics(
                    CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(intrinsics.assume_init())
            }
        }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        unsafe {
            sys::rs2_delete_stream_profile(self.ptr.as_ptr());
        }
    }
}
