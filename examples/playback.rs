use std::io::{BufRead, Write};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{
    fs::{self, File},
    io::{self},
    path::Path,
    process::exit,
};

use feetech_servo_rs::{Command, Driver};

fn read_position_file(name: &str) -> Vec<i32> {
    let file = File::open(format!("./positions/{}", name)).expect("File not found");
    let mut reader = io::BufReader::new(file);
    let mut values = String::new();
    reader.read_line(&mut values).unwrap();
    values
        .split(",")
        .map(|v| v.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    println!("Enter the port (default: /dev/ttyACM0):");
    let mut port = String::new();
    let _ = io::stdin().read_line(&mut port);
    let port = match port.trim() {
        "" => "/dev/ttyACM0",
        other => other,
    };
    let mut driver = Driver::new(port);

    let num_motors = 6;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    ctrlc::set_handler(move || {
        println!("\n\n\n\nCtrl+C detected, shutting down...");
        running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // let song = vec![60, 60, 67, 67, 69, 69, 67]; // twinkle
    let song = vec![
        (60, 500),
        (62, 500),
        (64, 500),
        (65, 500),
        (67, 500),
        (69, 500),
        (71, 500),
        (72, 500),
    ];

    // let playback = vec![
    //     ("60a", 1000),
    //     ("60", 1000),
    //     ("0", 500),
    //     ("62a", 1000),
    //     ("62", 1000),
    //     ("0", 500),
    //     ("64a", 1000),
    //     ("64", 1000),
    //     ("0", 500),
    //     // (65, 1000),
    //     // (0, 500),
    //     // (67, 1000),
    //     // (0, 500),
    //     // (69, 1000),
    //     // (0, 500),
    //     // (71, 1000),
    //     // (0, 500),
    //     // (72, 1000),
    //     // (0, 500),
    // ];

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let intermediate_duration = 400;
        for (note_value, duration) in &song {
            let intermediate = get_intermediate_position(*note_value);
            let intermediate_positions = read_position_file(&intermediate);
            for motor_id in 1u8..=6u8 {
                driver.act(
                    motor_id,
                    Command::WriteTargetPosition(
                        intermediate_positions[(motor_id - 1) as usize] as u16,
                    ),
                );
            }
            sleep(Duration::from_millis(intermediate_duration));
            let positions = read_position_file(&(note_value.to_string()));
            for motor_id in 1u8..=6u8 {
                driver.act(
                    motor_id,
                    Command::WriteTargetPosition(positions[(motor_id - 1) as usize] as u16),
                );
            }
            sleep(Duration::from_millis(*duration));

            for motor_id in 1u8..=6u8 {
                driver.act(
                    motor_id,
                    Command::WriteTargetPosition(
                        intermediate_positions[(motor_id - 1) as usize] as u16,
                    ),
                );
            }
            sleep(Duration::from_millis(intermediate_duration));
        }
    }

    for motor_id in 1u8..=6u8 {
        driver
            .act(motor_id, Command::WriteTorqueSwitch(false))
            .unwrap();
    }
}

fn get_intermediate_position(position: u16) -> String {
    format!("{position}a")
}
