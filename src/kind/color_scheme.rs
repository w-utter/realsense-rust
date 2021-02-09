//! The enumeration of color schemes.

use num_derive::{FromPrimitive, ToPrimitive};

#[repr(usize)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    Jet = 0,
    Classic = 1,
    WhiteToBlack = 2,
    BlackToWhite = 3,
    Bio = 4,
    Cold = 5,
    Warm = 6,
    Quantized = 7,
    Pattern = 8,
    Hue = 9,
}
