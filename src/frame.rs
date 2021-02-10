//! Defines the frame type including sensor data.

mod composite;
mod frame_traits;
mod image;
mod iter;
mod pixel;
mod pose;

pub use self::image::{DepthFrame, DisparityFrame, VideoFrame};
pub use composite::CompositeFrame;
pub use frame_traits::{DepthFrameEx, DisparityFrameEx, VideoFrameEx};
pub use pixel::PixelKind;
pub use pose::PoseFrame;

// UV texture coordinates.
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct TextureCoordinate {
//     // SAFETY: See safe_transmute::TriviallyTransmutable trait implementation for this type.
//     pub u: f32,
//     pub v: f32,
// }
//
// // SAFETY: TextureCoordinate is a POD type.
// unsafe impl safe_transmute::TriviallyTransmutable for TextureCoordinate {}
//
// impl PointsFrame {
//     /// Gets vertices of point cloud.
//     pub fn vertices<'a>(&'a self) -> Result<&'a [sys::rs2_vertex]> {
//         let n_points = self.points_count()?;
//         unsafe {
//             let mut checker = ErrorChecker::new();
//             let ptr = sys::rs2_get_frame_vertices(self.ptr.as_ptr(), checker.inner_mut_ptr());
//             checker.check()?;
//             let slice = slice::from_raw_parts::<sys::rs2_vertex>(ptr, n_points);
//             Ok(slice)
//         }
//     }
//
//     /// Gets texture coordinates of each point of point cloud.
//     pub fn texture_coordinates<'a>(&'a self) -> Result<&'a [TextureCoordinate]> {
//         unsafe {
//             let n_points = self.points_count()?;
//             let mut checker = ErrorChecker::new();
//             let ptr =
//                 sys::rs2_get_frame_texture_coordinates(self.ptr.as_ptr(), checker.inner_mut_ptr());
//             checker.check()?;
//
//             // SAFETY:
//             // The librealsense2 C++ API directly casts the rs2_pixel* returned from
//             // rs2_get_frame_texture_coordinates() into a texture_coordinate*, thereby
//             // re-interpreting [[c_int; 2]; N] as [[c_float; 2]; N] values.
//             // Note that C does not generally guarantee that sizeof(int) == sizeof(float).
//             let slice = slice::from_raw_parts::<sys::rs2_pixel>(ptr, n_points);
//             let bytes =
//                 slice::from_raw_parts::<u8>(slice.as_ptr() as *const u8, mem::size_of_val(slice));
//             let tcs =
//                 safe_transmute::transmute_many::<TextureCoordinate, PedanticGuard>(bytes).unwrap();
//             debug_assert_eq!(tcs.len(), n_points);
//             Ok(tcs)
//         }
//     }
//
//     /// Gets number of points in frame.
//     pub fn points_count(&self) -> Result<usize> {
//         unsafe {
//             let mut checker = ErrorChecker::new();
//             let val = sys::rs2_get_frame_points_count(self.ptr.as_ptr(), checker.inner_mut_ptr());
//             checker.check()?;
//             Ok(val as usize)
//         }
//     }
// }
//
// impl MotionFrame {
//     /// Gets motion data.
//     pub fn motion(&self) -> Result<[f32; 3]> {
//         let slice = safe_transmute::transmute_many::<f32, PedanticGuard>(self.data()?).unwrap();
//         match *slice {
//             [x, y, z] => Ok([x, y, z]),
//             _ => unreachable!("please report bug"),
//         }
//     }
// }
//
