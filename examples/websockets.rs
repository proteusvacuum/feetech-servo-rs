use std::fs::File;
use std::io::BufRead;

use std::sync::{mpsc, Arc, Mutex};
use std::{io, thread::sleep, time::Duration};

use feetech_servo_rs::{Driver, ReadCommand::CurrentPosition};

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

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
    println!("Enter the leader port (default: /dev/ttyACM1):");
    let mut leader_port = String::new();
    let _ = io::stdin().read_line(&mut leader_port);

    let server = TcpListener::bind("10.100.11.67:9001").unwrap();

    let (tx, rx) = mpsc::channel::<Vec<i32>>();
    let rx = Arc::new(Mutex::new(rx));

    let tx_driver = tx.clone();
    spawn(move || {
        let leader_port = match leader_port.trim() {
            "" => "/dev/ttyACM1",
            other => other,
        };
        let mut leader = Driver::new(leader_port);
        let mut leader_positions: Vec<u16> = [0; 6].to_vec();

        let leader_zero = read_calibration_file("./leader");
        loop {
            for motor_id in 1u8..=6u8 {
                leader_positions[(motor_id - 1) as usize] =
                    leader.read(motor_id, CurrentPosition).unwrap();
            }
            let leader_angles: Vec<i32> = leader_positions
                .iter()
                .zip(&leader_zero)
                .map(|(step, zero)| step_to_angle(*step as i32, *zero))
                .collect();
            if tx_driver.send(leader_angles).is_err() {
                break;
            }
            sleep(Duration::from_millis(10));
        }
    });

    for stream in server.incoming() {
        let rx_client = Arc::clone(&rx);

        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let angles = {
                    let rx_lock = rx_client.lock().unwrap();
                    rx_lock.recv().unwrap()
                };

                if websocket.send(format!("{:?}", angles).into()).is_err() {
                    break;
                }
            }
        });
    }
}

const STEPS_PER_REV: i32 = 4096;
const DEGREES_PER_STEP: f32 = 360.0 / STEPS_PER_REV as f32;

fn step_to_angle(current_step: i32, zero_step: i32) -> i32 {
    let mut delta = (current_step - zero_step) % STEPS_PER_REV;
    if delta < 0 {
        delta += STEPS_PER_REV;
    }
    let mut angle = (delta as f32 * DEGREES_PER_STEP).round() as i32;
    if angle > 180 {
        angle -= 360;
    }
    angle
}
