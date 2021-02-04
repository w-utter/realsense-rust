//! Generic iterator types for implementing frame iterators.

use super::frame_trait::{VideoFrameEx, VideoFrameUnsafeEx};

pub struct ImageIter<'a, F>
where
    F: VideoFrameEx,
{
    pub(crate) frame: &'a F,
    pub(crate) column: usize,
    pub(crate) row: usize,
}

impl<'a, F> Iterator for ImageIter<'a, F>
where
    F: VideoFrameEx,
{
    type Item = <F as VideoFrameUnsafeEx>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column >= self.frame.width() || self.row >= self.frame.height() {
            return None;
        }

        let next = self.frame.at_no_bounds_check(self.column, self.row);

        self.column += 1;

        if self.column >= self.frame.width() {
            self.column = 0;
            self.row += 1;
        }
        Some(next)
    }
}
