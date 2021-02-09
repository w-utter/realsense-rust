//! The enumeration of timestamp domains.

use num_derive::{FromPrimitive, ToPrimitive};
use realsense_sys as sys;
use std::ffi::CStr;

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2TimestampDomain {
    HardwareClock = sys::rs2_timestamp_domain_RS2_TIMESTAMP_DOMAIN_HARDWARE_CLOCK,
    SystemTime = sys::rs2_timestamp_domain_RS2_TIMESTAMP_DOMAIN_SYSTEM_TIME,
    GlobalTime = sys::rs2_timestamp_domain_RS2_TIMESTAMP_DOMAIN_GLOBAL_TIME,
    // Not included since this just tells us the total number of domains
    //
    // Count = sys::rs2_timestamp_domain_RS2_TIMESTAMP_DOMAIN_COUNT,
}

impl Rs2TimestampDomain {
    pub fn as_cstr(&self) -> &'static CStr {
        unsafe {
            let ptr = sys::rs2_timestamp_domain_to_string(*self as sys::rs2_timestamp_domain);
            CStr::from_ptr(ptr)
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.as_cstr().to_str().unwrap()
    }
}

impl ToString for Rs2TimestampDomain {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}
