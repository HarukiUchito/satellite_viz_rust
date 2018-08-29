#[macro_use]
extern crate rosrust_codegen;

// If you wanted
// * messages: std_msgs/String, sensor_msgs/Imu
// * services: roscpp_tutorials/TwoInts
// * and all the message types used by them, like geometry_msgs/Vector3
rosmsg_main!("rtklibros/satellite", "rtklibros/satellites");