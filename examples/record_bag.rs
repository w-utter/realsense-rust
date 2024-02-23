//! Record a ROSbag file with a RealSense camera.
//!
//! - The settings below all correspond to Intel RealSense's recommended calibration settings.
//! - The laser emitter is turned off.
//!
//! # ROSbag Playback Warning
//!
//! The ROSbag file produced by this program is _not_ written to spec. It will not play back in a
//! program like Foxglove, for instance, without first being [converted to an
//! MCAP](https://github.com/foxglove/mcap/tree/main/go/cli/mcap#installing). Even then, the color
//! camera will be encoded `YUYV`, which is not a ROSbag standard.
//!
//! However, these settings are necessary for calibration. Note that the data throughput of these
//! settings is extreme; 30 seconds of recording at these settings will be about 3GB of data.

use anyhow::{ensure, Result};
use getopts::Options;
use realsense_rust::{
    config::Config,
    context::Context,
    kind::{Rs2CameraInfo, Rs2Format, Rs2Option, Rs2StreamKind},
    pipeline::InactivePipeline,
};
use std::{collections::HashSet, convert::TryFrom};

/// The duration of the recording in seconds
const RECORDING_DURATION_SEC: u64 = 30;

/// Print usage information for this program
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("b", "bag", "Path to the bag file", "BAG_FILE");
    opts.optopt("h", "help", "Print this help menu", "HELP");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("Error: {}", f);
            std::process::exit(1);
        }
    };
    if !matches.opt_present("b") || matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    let bag_file = matches.opt_str("b").unwrap();
    if bag_file.ends_with(".bag") {
        println!("Recording to bag file: {}", bag_file);
    } else {
        eprintln!("Error: Bag file must end with .bag");
        std::process::exit(1);
    }

    // Check for depth or color-compatible devices.
    let queried_devices = HashSet::new(); // Query any devices
    let context = Context::new()?;
    let devices = context.query_devices(queried_devices);
    ensure!(!devices.is_empty(), "No devices found");

    // Turn off our laser emitter
    if let Some(device) = devices.first() {
        for mut sensor in device.sensors() {
            _ = sensor.set_option(Rs2Option::EmitterEnabled, 0.0);
        }
    }

    // create pipeline
    let pipeline = InactivePipeline::try_from(&context)?;
    let mut config = Config::new();
    config
        .enable_device_from_serial(devices[0].info(Rs2CameraInfo::SerialNumber).unwrap())?
        .enable_record_to_file(bag_file)?
        .disable_all_streams()?
        .enable_stream(Rs2StreamKind::Color, None, 1920, 1080, Rs2Format::Yuyv, 15)?
        .enable_stream(
            Rs2StreamKind::Infrared,
            Some(1),
            1280,
            800,
            Rs2Format::Y16,
            15,
        )?
        .enable_stream(
            Rs2StreamKind::Infrared,
            Some(2),
            1280,
            800,
            Rs2Format::Y16,
            15,
        )?;

    // Change pipeline's type from InactivePipeline -> ActivePipeline
    let mut pipeline = pipeline.start(Some(config))?;

    // Process frames. We must `wait` on frames to record them to the bag file.
    let now = std::time::Instant::now();
    while now.elapsed().as_secs() <= RECORDING_DURATION_SEC {
        if let Err(e) = pipeline.wait(None) {
            eprintln!("Pipeline failed; Reason {e}");
            break;
        }
    }

    pipeline.stop();

    Ok(())
}
