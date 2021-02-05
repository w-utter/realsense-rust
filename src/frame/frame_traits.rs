//! Trait for describing basic frame operations

use crate::common::*;

pub enum FrameConstructionError {
    CouldNotGetWidth(String),
    CouldNotGetHeight(String),
    CouldNotGetStride(String),
    CouldNotGetBitsPerPixel(String),
    CouldNotGetFrameStreamProfile(String),
    CouldNotGetDataSize(String),
    CouldNotGetData(String),
}

pub trait VideoFrameUnsafeEx {
    type Output: Sized;

    fn get_unchecked(&self, col: usize, row: usize) -> Self::Output;
}

pub trait VideoFrameEx: VideoFrameUnsafeEx {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn get(&self, col: usize, row: usize) -> Option<Self::Output>;
}

#[doc(hidden)]
macro_rules! check_rs2_error {
    ($rs2_error:expr, $result:expr) => {
        // We make this alias here to type check $rs2_error.
        let err: *mut sys::rs2_error = $rs2_error;
        if NonNull::new(err).is_some() {
            return Err($result(
                std::ffi::CStr::from_ptr(sys::rs2_get_error_message(err))
                    .to_str()
                    .unwrap()
                    .to_string(),
            ));
        }
    };
}
