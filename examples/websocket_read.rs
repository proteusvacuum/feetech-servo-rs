use std::fs::File;
use std::io::BufRead;

use std::io;
use std::sync::{mpsc, Arc, Mutex};

use feetech_servo_rs::{Driver, WriteCommand::TargetPosition};

use std::{net::TcpListener, thread::spawn};

use tungstenite::{accept, Message};

fn read_calibration_file(name: &str) -> Vec<i32> {
    let file = File::open(format!("./calibration/{}", name)).expect("File not found");
    let mut reader = io::BufReader::new(file);
    let mut values = String::new();
    reader.read_line(&mut values).unwrap();
    values
        .split(",")
        .map(|v| v.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    println!("Enter the follower port (default: /dev/ttyACM0):");
    let mut follower_port = String::new();
    let _ = io::stdin().read_line(&mut follower_port);

    let (tx, rx) = mpsc::channel::<Vec<i32>>();
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let rx_client = Arc::clone(&rx);
    spawn(move || {
        let follower_port = match follower_port.trim() {
            "" => "/dev/ttyACM0",
            other => other,
        };
        let mut follower = Driver::new(follower_port);

        let follower_zero = read_calibration_file("./follower");
        loop {
            let angles = {
                let rx_lock = rx_client.lock().unwrap();
                rx_lock.recv().unwrap()
            };
            let target_positions: Vec<u16> = angles
                .iter()
                .zip(&follower_zero)
                .map(|(target, zero)| angle_to_step(*target, *zero))
                .collect();

            for motor_id in 1u8..=6u8 {
                follower
                    .write(
                        motor_id,
                        TargetPosition(target_positions[(motor_id - 1) as usize]),
                    )
                    .unwrap();
            }
        }
    });

    let server = TcpListener::bind("10.100.11.67:9002").unwrap();
    for stream in server.incoming() {
        let tx_driver = Arc::clone(&tx);
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                if let Message::Text(data) = websocket.read().unwrap() {
                    let angles: Vec<i32> = data
                        .split(",")
                        .map(|d| d.trim())
                        .map(|d| d.parse::<i32>().unwrap())
                        .collect();
                    if tx_driver.lock().unwrap().send(angles).is_err() {
                        break;
                    }
                }
            }
        });
    }
}

const STEPS_PER_REV: i32 = 4096;
const DEGREES_PER_STEP: f32 = 360.0 / STEPS_PER_REV as f32;

fn angle_to_step(angle: i32, zero_step: i32) -> u16 {
    if !(-180..=180).contains(&angle) {
        panic!("Angle must be between -180 and 180 degrees");
    }
    let mut angle_norm = angle;
    if angle_norm < 0 {
        angle_norm += 360;
    }
    let step_offset = ((angle_norm as f32) / DEGREES_PER_STEP).round() as i32;
    (zero_step + step_offset).rem_euclid(STEPS_PER_REV) as u16
}
