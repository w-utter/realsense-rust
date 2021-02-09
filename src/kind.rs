//! Defines the common used enums.

use crate::common::*;

mod camera_info;
mod color_scheme;
mod extension;
mod format;
mod frame_metadata;
mod option;
mod stream_kind;
mod timestamp_domain;

pub use camera_info::Rs2CameraInfo;
pub use color_scheme::ColorScheme;
pub use extension::Rs2Extension;
pub use format::Rs2Format;
pub use frame_metadata::Rs2FrameMetadata;
pub use option::Rs2Option;
pub use stream_kind::Rs2StreamKind;
pub use timestamp_domain::TimestampDomain;

/// The enumeration of persistence controls.
#[repr(usize)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PersistenceControl {
    Disabled = 0,
    Valid8OutOf8 = 1,
    Valid2OutOf3 = 2,
    Valid2OutOf4 = 3,
    Valid2OutOf8 = 4,
    Valid1OutOf2 = 5,
    Valid1OutOf5 = 6,
    Valid1OutOf8 = 7,
    Indefinitely = 8,
}

/// The enumeration of persistence controls.
#[repr(usize)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HoleFillingMode {
    FillFromLeft = 0,
    FarestFromAround = 1,
    NearestFromAround = 2,
}
