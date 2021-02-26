//! Tests for evaluating connectivity / configuration of sensors

#![cfg(feature = "test-single-device")]

use anyhow::Result;
use realsense_rust::{
    context::Context,
    device::Device,
    kind::{Rs2CameraInfo, Rs2ProductLine},
};
use std::collections::HashSet;

/// Ensure at least one intel device is "connected" as far as the driver is concerned.
///
/// Seems dumb but this is a necessary check for every other test.
#[test]
fn ensure_at_least_one_intel_device_connected() {
    let context = Context::new().unwrap();
    let mut mask = HashSet::new();
    mask.insert(Rs2ProductLine::AnyIntel);

    let devices = context.query_devices(mask);

    assert!(devices.len() >= 1);
}
