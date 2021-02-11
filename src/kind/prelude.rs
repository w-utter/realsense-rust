//! Common traits used throughout the crate.

use super::extension::Rs2Extension;

pub trait Extension {
    fn extension() -> Rs2Extension;
}
