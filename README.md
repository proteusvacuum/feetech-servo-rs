# Feetech Bus Servo SDK (Rust)

This is a Rust implementation of the **[Feetech bus servo SDK](https://gitee.com/ftservo/SCServoSDK/tree/master)** to control the [Feetech STS line](https://www.feetechrc.com/sts_ttl_series%20servo.html) of smart serial bus servo motors.

This is still a work-in-progress and should not be considered stable.

This SDK provides a high-level `Driver` abstraction to manage a chain of servos.

This was tested with twelve STS3215 servo motors, used in the [S0-ARM100](https://github.com/TheRobotStudio/SO-ARM100) robot arm.

## Usage

```rust
use feetech_servo_rs::driver::Driver;
use feetech_servo_rs::commands::Command;

let mut driver = Driver::new("/dev/ttyUSB0");
let current_position: u16 = driver.act(motor_id, Command::ReadCurrentPosition).unwrap();
driver.act(motor_id, Command::WriteTargetPosition(current_position + 5u16)).unwrap();
```

## Examples

- **Teleoperation Demo**: See [`examples/teloperate.rs`](https://github.com/proteusvacuum/feetech-servo-rs/blob/main/examples/teloperate.rs) for a real-time leader-follower example, where the leader controls the follower exactly.


Made with :heart: at the [Recurse Center](https://www.recurse.com)
