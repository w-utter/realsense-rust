//! Trait for describing the kind of data a frame holds

use crate::common::*;

pub trait Kind {
    fn extension() -> sys::rs2_extension;
}
