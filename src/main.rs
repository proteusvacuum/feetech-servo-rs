use feetech_servo_sdk::packet_handler::PacketHandler;

fn main() {
    let mut packet_handler = PacketHandler::new("/dev/ttyACM0", 1_000_000);
    let result = dbg!(packet_handler.ping(1));
    if let Ok(feetech_servo_sdk::packet_handler::RxStatus::Success(Some(status_packet))) = result {
        println!("{}", status_packet);
    }
}
