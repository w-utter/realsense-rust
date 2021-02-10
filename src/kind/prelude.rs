//! Common traits used throughout the crate.

use super::extension::Rs2Extension;

pub trait Kind {
    fn extension() -> Rs2Extension;
}
