use anyhow::{ensure, Result};
use realsense_rust::{
    config::Config,
    context::Context,
    frame::DepthFrame,
    kind::{Rs2Format, Rs2ProductLine, Rs2StreamKind},
    pipeline::InactivePipeline,
};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::time::Duration;

pub fn main() -> Result<()> {
    // Check for depth or color-compatible devices.
    let mut queried_devices = HashSet::new();
    queried_devices.insert(Rs2ProductLine::Any);
    let context = Context::new()?;
    let devices = context.query_devices(queried_devices);
    ensure!(!devices.is_empty(), "No devices found");

    // create pipeline
    let pipeline = InactivePipeline::try_from(&context)?;
    let mut config = Config::new();
    config.enable_stream(Rs2StreamKind::Depth, 0, 640, 0, Rs2Format::Z16, 30)?;
    config.enable_stream(Rs2StreamKind::Color, 0, 640, 0, Rs2Format::Rgb8, 30)?;
    let mut pipeline = pipeline.start(Some(&config))?;

    // process frames
    for _ in 0..1000 {
        let timeout = Duration::from_millis(1000);
        let frames = pipeline.wait(Some(timeout))?;
        let mut depth_frames = frames.frames_of_extension::<DepthFrame>();
        if depth_frames.is_empty() {
            continue;
        }

        // Debug width and height calls
        let depth_frame = depth_frames.pop().unwrap();
        // println!("{:#?}", depth_frame);
        let distance = depth_frame.distance(depth_frame.width() / 2, depth_frame.height() / 2)?;
        print!("\rCurrent distance of center pixel: {:15} m", distance);
    }

    Ok(())
}
