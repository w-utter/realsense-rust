//! Trait for describing basic frame operations

pub enum FrameConstructionError {
    CouldNotGetWidth(String),
    CouldNotGetHeight(String),
    CouldNotGetStride(String),
    CouldNotGetBitsPerPixel(String),
    CouldNotGetFrameStreamProfile(String),
    CouldNotGetDataSize(String),
    CouldNotGetData(String),
}

pub trait VideoFrameUnsafeEx {
    type Output: Sized;

    fn get_unchecked(&self, col: usize, row: usize) -> Self::Output;
}

pub trait VideoFrameEx: VideoFrameUnsafeEx {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn stride(&self) -> usize;

    fn bits_per_pixel(&self) -> usize;

    fn get(&self, col: usize, row: usize) -> Option<Self::Output>;
}
