//! Common traits used throughout the crate.

use super::extension::Rs2Extension;

/// A type describing the data held within an rs2_frame.
pub trait Extension {
    /// Identifies the proper RS2 extension for the type implementing this trait.
    fn extension() -> Rs2Extension;
}
