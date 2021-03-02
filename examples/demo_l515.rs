use anyhow::{ensure, Result};
use realsense_rust::{
    config::Config,
    context::Context,
    frame::DepthFrame,
    kind::{Rs2Format, Rs2ProductLine, Rs2StreamKind},
    pipeline::InactivePipeline,
};
use std::{
    collections::HashSet,
    convert::TryFrom,
    io::{self, Write},
    time::Duration,
};

pub fn main() -> Result<()> {
    // Check for depth or color-compatible devices.
    let mut queried_devices = HashSet::new();
    queried_devices.insert(Rs2ProductLine::Any);
    let context = Context::new()?;
    let devices = context.query_devices(queried_devices);
    ensure!(!devices.is_empty(), "No devices found.");

    // create pipeline
    let pipeline = InactivePipeline::try_from(&context)?;
    let mut config = Config::new();
    config.enable_stream(Rs2StreamKind::Depth, 0, 320, 0, Rs2Format::Z16, 30)?;
    config.enable_stream(Rs2StreamKind::Infrared, 0, 320, 0, Rs2Format::Y8, 30)?;
    if !pipeline.can_resolve(&config) {
        println!("Cannot resolve assigned config. Check the config for incompatible types.");
        return Ok(());
    }
    let mut pipeline = pipeline.start(Some(&config))?;

    // process frames
    for _ in 0..1000 {
        // The L515 takes a while to initialize. Make this 5 seconds.
        let timeout = Duration::from_millis(5000);
        let frames = pipeline.wait(Some(timeout))?;

        let mut depth_frames = frames.frames_of_extension::<DepthFrame>();
        if depth_frames.is_empty() {
            continue;
        }
        let depth_frame = depth_frames.pop().unwrap();
        let distance = depth_frame.distance(depth_frame.width() / 2, depth_frame.height() / 2)?;
        if distance == 0.0 {
            continue;
        }
        print!("\rCurrent distance of center pixel: {:<15} m", distance);
        io::stdout().flush().unwrap();
    }

    Ok(())
}
