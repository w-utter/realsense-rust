//! Defines utilities for dealing with errors across the crate

#[doc(hidden)]
#[macro_export]
macro_rules! check_rs2_error {
    ($rs2_error:expr, $result:expr) => {
        // We make this alias here to type check $rs2_error.
        {
            let err: *mut sys::rs2_error = $rs2_error;
            if NonNull::new(err).is_some() {
                Err($result(
                    std::ffi::CStr::from_ptr(sys::rs2_get_error_message(err))
                        .to_str()
                        .unwrap()
                        .to_string(),
                ))
            } else {
                Ok(())
            }
        }
    };
}
