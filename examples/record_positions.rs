use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::exit,
};

use feetech_servo_rs::{Command, Driver};

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

    println!("Enter the note number:");
    let mut name = String::new();
    let _ = io::stdin().read_line(&mut name);
    let name = name.trim();

    println!("Set the motors to the position.\nStart (y/[N])?");
    let mut go = String::new();
    let _ = io::stdin().read_line(&mut go);
    match go.to_ascii_lowercase().trim() {
        "yes" | "y" => {}
        _ => {
            exit(1);
        }
    }

    println!("Recording positions");

    let mut positions: Vec<u16> = Vec::with_capacity(num_motors as usize);
    for motor_id in 1..=num_motors {
        positions.push(driver.act(motor_id, Command::ReadCurrentPosition).unwrap());
    }
    let path = format!("./positions/{}", name);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).expect("Failed to make calibration directory");
    }
    let mut f = File::create(&path).expect("Failed to create file");
    let line = positions
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Positions: {}", line);
    writeln!(f, "{}", line).expect("Error writing file!");
    println!("Wrote to: {}", path);
}
