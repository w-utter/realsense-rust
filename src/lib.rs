#![doc = include_str!("../README.md")]

pub mod base;
pub mod config;
pub mod context;
pub mod device;
pub mod device_hub;
pub mod docs;
mod error;
pub mod frame;
pub mod kind;
pub mod pipeline;
pub mod sensor;
pub mod stream_profile;

// pub mod frame_queue;
// pub mod processing_block;
// pub mod processing_block_kind;
// pub mod processing_block_list;

/// The module collects common used traits from this crate.
pub mod prelude {
    pub use crate::frame::{FrameCategory, FrameEx};
}

// pub use frame_queue::FrameQueue;
// pub use processing_block::{
//     Align, AnyProcessingBlock, Colorizer, DecimationFilter, DisparityFilter, HoleFillingFilter,
//     HuffmanDepthDecompress, PointCloud, ProcessingBlock, RatesPrinter, SpatialFilter, Syncer,
//     TemporalFilter, ThresholdFilter, UnitsTransform, YuyDecoder, ZeroOrderFilter,
// };
// pub use processing_block_list::{ProcessingBlockList, ProcessingBlockListIntoIter};
