//! Tests for evaluating connectivity / configuration of sensors

#![cfg(feature = "test-single-device")]

use realsense_rust::{
    config::Config,
    context::Context,
    kind::{Rs2CameraInfo, Rs2Format, Rs2ProductLine, Rs2StreamKind},
    pipeline::InactivePipeline,
};
use std::{collections::HashSet, convert::TryFrom};

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

#[test]
fn can_resolve_all_streams_always() {
    let context = Context::new().unwrap();
    let mut config = Config::new();
    config.enable_all_streams().unwrap();

    let pipeline = InactivePipeline::try_from(&context).unwrap();

    assert!(pipeline.can_resolve(&config));
}

#[test]
fn can_resolve_color_and_depth_and_infrared_on_d400_series() {
    let context = Context::new().unwrap();
    let devices = context.query_devices(HashSet::new());

    let device = devices.iter().find(|d| {
        let product = d.info(Rs2CameraInfo::ProductLine).unwrap();
        product.to_str().unwrap() == "D400"
    });

    if let Some(device) = device {
        let serial = device.info(Rs2CameraInfo::SerialNumber).unwrap();
        let mut config = Config::new();

        config
            .enable_device_from_serial(serial)
            .unwrap()
            .disable_all_streams()
            .unwrap()
            .enable_stream(Rs2StreamKind::Color, Some(0), 0, 0, Rs2Format::Rgba8, 30)
            .unwrap()
            .enable_stream(Rs2StreamKind::Depth, Some(0), 0, 0, Rs2Format::Z16, 30)
            .unwrap()
            // RealSense doesn't seem to like index zero for the IR cameras
            //
            // Really not sure why? This seems like an implementation issue, but in practice most
            // won't be after the IR image directly (I think?).
            .enable_stream(Rs2StreamKind::Infrared, Some(1), 0, 0, Rs2Format::Any, 30)
            .unwrap()
            .enable_stream(Rs2StreamKind::Infrared, Some(2), 0, 0, Rs2Format::Any, 30)
            .unwrap();

        let pipeline = InactivePipeline::try_from(&context).unwrap();

        assert!(pipeline.can_resolve(&config));
        assert!(pipeline.resolve(&config).is_some());
    } else {
        return;
    }
}
