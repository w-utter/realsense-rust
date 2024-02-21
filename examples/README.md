# Examples

Look at the source code of the examples to find hepful comments and pointers about how to get the
most out of this API. Run any of the examples here using cargo: `$cargo run --example <demo name>`

## For all devices

-   [enumerate_devices.rs](enumerate_devices.rs): Enumerates all devices connected to the host that
    are compatible with the Intel RealSense library.
-   [opencv.rs](opencv.rs): Streams from a device with depth and color sensors, converts the frames
    to OpenCV Mats, and visualizes the results using OpenCV's High-Level GUI library.
-   [record_to_bag.rs](record_to_bag.rs): Records a stream to a ROSbag file. The stream settings are
    currently set to the calibration configuration for a RealSense 435i.

## 435i

-   [demo_435i.rs](demo_435i.rs): Runs a subset of streams on the 435i. Prints out the center pixel
    value of the depth image and the current gyro value.

## L515

-   [demo_l515.rs](demo_l515.rs): Runs a subset of streams on the L515. Prints out the center pixel
    value of the depth image.
