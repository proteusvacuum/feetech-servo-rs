use feetech_servo_rs::{Command, Driver};
use std::io;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    println!("Enter the port (default: /dev/ttyACM0):");
    let mut port = String::new();
    let _ = io::stdin().read_line(&mut port);
    let port = match port.trim() {
        "" => "/dev/ttyACM0",
        other => other,
    };
    let mut driver = Driver::new(port);

    ctrlc::set_handler(move || {
        println!("\n\n\n\nCtrl+C detected, shutting down...");
        running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    });
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let mut positions: Vec<u16> = Vec::with_capacity(6 as usize);
        for motor_id in 1..=6 {
            positions.push(driver.act(motor_id, Command::ReadCurrentPosition).unwrap());
        }
    }
}
