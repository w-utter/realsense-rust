//! Type for representing a pose frame taken from an IMU or pose-sensor.

use super::prelude::{CouldNotGetFrameSensorError, FrameConstructionError, FrameEx};
use crate::{
    check_rs2_error,
    common::*,
    kind::{Extension, Rs2Extension, Rs2FrameMetadata, Rs2TimestampDomain},
    sensor::Sensor,
    stream::StreamProfile,
};
use anyhow::Result;
use std::convert::TryFrom;

pub struct PoseFrame<'a> {
    frame_ptr: NonNull<sys::rs2_frame>,
    timestamp: f64,
    timestamp_domain: Rs2TimestampDomain,
    frame_stream_profile: StreamProfile<'a>,
    data: sys::rs2_pose,
    should_drop: bool,
}

pub enum Confidence {
    Failed,
    Low,
    Medium,
    High,
}

impl<'a> PoseFrame<'a> {
    pub fn translation(&self) -> [f32; 3] {
        let sys::rs2_vector { x, y, z } = self.data.translation;
        [x, y, z]
    }

    pub fn velocity(&self) -> [f32; 3] {
        let sys::rs2_vector { x, y, z } = self.data.velocity;
        [x, y, z]
    }

    pub fn acceleration(&self) -> [f32; 3] {
        let sys::rs2_vector { x, y, z } = self.data.acceleration;
        [x, y, z]
    }

    pub fn rotation(&self) -> [f32; 4] {
        let sys::rs2_quaternion { x, y, z, w } = self.data.rotation;
        [x, y, z, w]
    }

    pub fn angular_velocity(&self) -> [f32; 3] {
        let sys::rs2_vector { x, y, z } = self.data.angular_velocity;
        [x, y, z]
    }

    pub fn angular_acceleration(&self) -> [f32; 3] {
        let sys::rs2_vector { x, y, z } = self.data.angular_acceleration;
        [x, y, z]
    }

    pub fn tracker_confidence(&self) -> Confidence {
        match self.data.tracker_confidence {
            0x0 => Confidence::Failed,
            0x1 => Confidence::Low,
            0x2 => Confidence::Medium,
            0x3 => Confidence::High,
            _ => panic!("Unknown confidence, please report a bug!"),
        }
    }

    pub fn mapper_confidence(&self) -> Confidence {
        match self.data.tracker_confidence {
            0x0 => Confidence::Failed,
            0x1 => Confidence::Low,
            0x2 => Confidence::Medium,
            0x3 => Confidence::High,
            _ => panic!("Unknown confidence, please report a bug!"),
        }
    }
}

impl<'a> Drop for PoseFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            if self.should_drop {
                sys::rs2_release_frame(self.frame_ptr.as_ptr());
            }
        }
    }
}

unsafe impl<'a> Send for PoseFrame<'a> {}

impl<'a> Extension for PoseFrame<'a> {
    fn extension() -> Rs2Extension {
        Rs2Extension::PoseFrame
    }
}

impl<'a> TryFrom<NonNull<sys::rs2_frame>> for PoseFrame<'a> {
    type Error = anyhow::Error;

    fn try_from(frame_ptr: NonNull<sys::rs2_frame>) -> Result<Self, Self::Error> {
        unsafe {
            let mut err = ptr::null_mut::<sys::rs2_error>();

            let timestamp = sys::rs2_get_frame_timestamp(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetTimestamp)?;

            let timestamp_domain =
                sys::rs2_get_frame_timestamp_domain(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetTimestampDomain)?;

            let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

            let nonnull_profile_ptr =
                NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
            let profile = StreamProfile::try_from(nonnull_profile_ptr)?;

            let mut pose_data = MaybeUninit::uninit();
            sys::rs2_pose_frame_get_pose_data(frame_ptr.as_ptr(), pose_data.as_mut_ptr(), &mut err);
            check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

            Ok(PoseFrame {
                frame_ptr,
                timestamp,
                timestamp_domain: Rs2TimestampDomain::from_u32(timestamp_domain).unwrap(),
                frame_stream_profile: profile,
                data: pose_data.assume_init(),
                should_drop: true,
            })
        }
    }
}

impl<'a> FrameEx<'a> for PoseFrame<'a> {
    fn profile(&'a self) -> &'a StreamProfile<'a> {
        &self.frame_stream_profile
    }

    fn sensor(&self) -> Result<Sensor> {
        unsafe {
            let mut err = std::ptr::null_mut::<sys::rs2_error>();
            let sensor_ptr = sys::rs2_get_frame_sensor(self.frame_ptr.as_ptr(), &mut err);
            check_rs2_error!(err, CouldNotGetFrameSensorError)?;

            Ok(Sensor::try_from(NonNull::new(sensor_ptr).unwrap())?)
        }
    }

    fn timestamp(&self) -> f64 {
        self.timestamp
    }

    fn timestamp_domain(&self) -> Rs2TimestampDomain {
        self.timestamp_domain
    }

    fn metadata(&self, metadata_kind: Rs2FrameMetadata) -> Option<std::os::raw::c_longlong> {
        unimplemented!();
    }

    fn supports_metadata(&self, metadata_kind: Rs2FrameMetadata) -> bool {
        unimplemented!();
    }

    unsafe fn get_owned_frame_ptr(mut self) -> NonNull<sys::rs2_frame> {
        self.should_drop = false;

        self.frame_ptr
    }
}
