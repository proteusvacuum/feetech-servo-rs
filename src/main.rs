use std::{thread::sleep, time::Duration};

use feetech_servo_sdk::{commands::Command, driver::Driver};

fn main() {
    let mut driver = Driver::new("/dev/ttyACM0");
    let current_position = driver.act(1, Command::ReadCurrentPosition).unwrap();
    println!("Current position: {}", current_position);
    let target_position = current_position - 200;
    driver
        .act(1, Command::WriteTargetPosition(target_position))
        .unwrap();
    let mut current_pos = driver.act(1, Command::ReadCurrentPosition).unwrap();
    while current_pos.abs_diff(target_position) > 10 {
        current_pos = driver.act(1, Command::ReadCurrentPosition).unwrap();
        sleep(Duration::from_millis(50));
        println!("Current position: {}", current_pos);
    }
    driver.act(1, Command::WriteTorqueSwitch(false)).unwrap();
    println!("Sleeping...");
}
