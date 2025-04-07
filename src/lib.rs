#![allow(dead_code)]

//! # Feetech Bus Servo SDK (Rust)

//! This is a Rust implementation of the **[Feetech bus servo SDK](https://gitee.com/ftservo/SCServoSDK/tree/master)** to control the [Feetech STS line](https://www.feetechrc.com/sts_ttl_series%20servo.html) of smart serial bus servo motors.
//!
//! This SDK provides a high-level `Driver` abstraction to manage a chain of servos.
//!
//! This was tested with twelve STS3215 servo motors, used in the [S0-ARM100](https://github.com/TheRobotStudio/SO-ARM100) robot arm.
//!
//! ## Usage
//!
//! ```no_run
//! use feetech_servo_rs::Driver;
//! use feetech_servo_rs::{ReadCommand::CurrentPosition, WriteCommand::TargetPosition};
//!
//! let motor_id = 1u8;
//! let mut driver = Driver::new("/dev/ttyUSB0");
//! let current_position: u16 = driver.read(motor_id, CurrentPosition).unwrap();
//! driver.write(motor_id, TargetPosition(current_position + 5u16)).unwrap();
//! ```
mod commands;
mod driver;

pub use commands::{ReadCommand, WriteCommand};
pub use driver::Driver;

mod instruction;
mod memory_location;
mod packet_handler;
mod packets;
mod serial;
mod utils;
