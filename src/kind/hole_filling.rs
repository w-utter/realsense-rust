//! The enumeration of persistence controls.

use num_derive::{FromPrimitive, ToPrimitive};

#[repr(usize)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HoleFillingMode {
    FillFromLeft = 0,
    FarestFromAround = 1,
    NearestFromAround = 2,
}
