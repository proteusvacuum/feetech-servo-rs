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

    let quarter = 500;
    let dotted_quarter = 750;
    let eighth = 250;
    let half = 1000;
    let dotted_half = 1500;
    let aboussikum = vec![
        ("Eb", quarter),
        ("C", quarter),
        ("C", quarter),
        ("C", quarter),
        ("Eb", quarter),
        ("C", quarter),
        ("C", quarter),
        ("C", quarter),
        ("D", quarter),
        ("D", quarter),
        ("G", quarter),
        ("F", quarter),
        ("Eb", quarter),
        ("C", quarter),
        ("C", quarter),
        ("Rest", quarter), // Rest
        ("Eb", quarter),
        ("C", quarter),
        ("C", quarter),
        ("C", quarter),
        ("Eb", quarter),
        ("C", quarter),
        ("C", quarter),
        ("C", quarter),
        ("Eb", quarter),
        ("Eb", quarter),
        ("D", quarter),
        ("Eb", quarter),
        ("C", dotted_half),
        ("Rest", quarter),
    ];

    let ode = vec![
        ("E", quarter),
        ("E", quarter),
        ("F", quarter),
        ("G", quarter),
        ("G", quarter),
        ("F", quarter),
        ("E", quarter),
        ("D", quarter),
        ("C", quarter),
        ("C", quarter),
        ("D", quarter),
        ("E", quarter),
        ("E", dotted_quarter),
        ("D", eighth),
        ("D", half),
        ("E", quarter),
        ("E", quarter),
        ("F", quarter),
        ("G", quarter),
        ("G", quarter),
        ("F", quarter),
        ("E", quarter),
        ("D", quarter),
        ("C", quarter),
        ("C", quarter),
        ("D", quarter),
        ("E", quarter),
        ("D", dotted_quarter),
        ("C", eighth),
        ("C", half),
        ("D", quarter),
        ("D", quarter),
        ("E", quarter),
        ("C", quarter),
        ("D", quarter),
        ("E", eighth),
        ("F", eighth),
        ("E", quarter),
        ("C", quarter),
        ("D", quarter),
        ("E", eighth),
        ("F", eighth),
        ("E", quarter),
        ("D", quarter),
        ("C", quarter),
        ("D", quarter),
        ("G", half),
        ("E", quarter),
        ("E", quarter),
        ("F", quarter),
        ("G", quarter),
        ("G", quarter),
        ("F", quarter),
        ("E", quarter),
        ("D", quarter),
        ("C", quarter),
        ("C", quarter),
        ("D", quarter),
        ("E", quarter),
        ("D", dotted_quarter),
        ("C", eighth),
        ("C", half + half),
    ];

    let heart = vec![
        // Heart and soul
        ("F", dotted_quarter),
        ("F", dotted_quarter),
        ("F", dotted_half + quarter),
        ("F", eighth),
        ("E", quarter),
        ("D", eighth),
        ("E", quarter),
        ("F", eighth),
        ("G", quarter),
        ("A", dotted_quarter),
        ("A", dotted_quarter),
        ("A", dotted_half),
        ("A", eighth),
        ("G", quarter),
        ("F", eighth),
        ("G", quarter),
        ("A", eighth),
        ("Bb", dotted_quarter),
        ("C2", dotted_half),
        ("F", dotted_half),
        ("C2", eighth),
        ("Bb", dotted_quarter),
        ("A", eighth),
        ("G", quarter),
        ("A", quarter),
        ("F", dotted_half),
        ("E", eighth),
        ("D", dotted_half),
        ("C", eighth),
        ("D", dotted_half),
        ("C", eighth),
        ("D", dotted_half),
        ("E", dotted_half),
    ];

    let scale = vec![
        ("C", half),
        ("D", half),
        ("E", half),
        ("F", half),
        ("G", half),
        ("A", half),
        ("Bb", half),
        ("B", half),
        ("C2", half),
    ];
    let bb = vec![("Bb", half)];
    let cc = vec![("C", half)];

    let song = heart;

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        // let intermediate_duration = 400;
        for (note, duration) in &song {
            let intermediate_duration = *duration / 3;
            let note_value = get_number_from_note(*note);
            let intermediate = get_intermediate_position(note_value);
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
            sleep(Duration::from_millis(*duration / 3));

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

fn get_number_from_note(note: &str) -> u16 {
    match note {
        "C" => 60,
        "D" => 62,
        "Eb" => 63,
        "E" => 64,
        "F" => 65,
        "G" => 67,
        "A" => 69,
        "Bb" => 70,
        "B" => 71,
        "C2" => 72,
        "Rest" => 200,
        _ => 60,
    }
}
