use feetech_servo_sdk::packet_handler::PacketHandler;

fn main() {
    let mut packet_handler = PacketHandler::new("/dev/ttyUSB0", 1_000_000);
    packet_handler.ping(1);
}
