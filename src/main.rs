use feetech_servo_sdk::{commands::Command, commands::Motor, packet_handler::PacketHandler};

fn main() {
    let baud_rate = 1_000_000;
    let mut packet_handler = PacketHandler::new("/dev/ttyACM1", baud_rate);
    let mut motor = Motor::new(&mut packet_handler, 1);
    motor.act(Command::Ping);
    motor.act(Command::ReadId);
    motor.act(Command::WriteTorqueSwitch(false));
}
