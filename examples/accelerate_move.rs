use std::io::{self};

use feetech_servo_rs::{Command, Driver};

fn main() {
    println!("Enter the port (default: /dev/tty.usbmodem58FD0166701");
    let mut port = String::new();
    let _ = io::stdin().read_line(&mut port);
    let port = match port.trim() {
        "" => "/dev/tty.usbmodem58FD0166701",
        other => other,
    };
    let mut driver = Driver::new(port);

    let mo1_pos = driver.act(1, Command::ReadCurrentPosition).unwrap();
    println!("Motor 1 is at {}", mo1_pos);

    println!("Enter how fast do you want to go? ");
    let mut new_accel = String::new();
    let _ = io::stdin().read_line(&mut new_accel);
    let new_accel: u8 = new_accel.trim().parse().expect("Please type a number!");

    driver
        .act(1, Command::WriteAcceleration(new_accel))
        .unwrap();

    println!("Enter where do you want to go?");
    let mut new_pos = String::new();
    let _ = io::stdin().read_line(&mut new_pos);
    let new_pos: u16 = new_pos.trim().parse().expect("Please type a number!");

    driver
        .act(1, Command::WriteTargetPosition(new_pos))
        .unwrap();
}
