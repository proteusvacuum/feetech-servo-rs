use std::io::{self};

use feetech_servo_rs::{
    Driver,
    ReadCommand::{Acceleration, Temperature},
};

fn main() {
    println!("Enter the port (default: /dev/ttyACM0):");
    let mut port = String::new();
    let _ = io::stdin().read_line(&mut port);
    let port = match port.trim() {
        "" => "/dev/ttyACM0",
        other => other,
    };
    let mut driver = Driver::new(port);

    println!("Enter the number of motors:");
    let mut num_motors = String::new();
    let _ = io::stdin().read_line(&mut num_motors);
    let num_motors: u8 = num_motors.trim().parse().expect("Please type a number!");
    for motor_id in 1..=num_motors {
        println!(
            "motor {motor_id} temperature:{}",
            driver.read(motor_id, Temperature).unwrap()
        );
        println!(
            "motor {motor_id} acceleration: {}",
            driver.read(motor_id, Acceleration).unwrap()
        )
    }
}
