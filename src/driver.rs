use crate::{commands::Command, packet_handler::PacketHandler};

pub struct Driver {
    packet_handler: PacketHandler,
    motor_ids: Vec<u8>,
}

impl Driver {
    pub fn new(port_name: &str, motor_ids: &[u8]) -> Self {
        let baud_rate = 1_000_000;
        Self {
            packet_handler: PacketHandler::new(port_name, baud_rate),
            motor_ids: motor_ids.to_vec(),
        }
    }

    pub fn act(&mut self, motor_id: u8, command: Command) -> Option<u16> {
        let packet = command.to_instruction_packet(motor_id);
        let status_packet = match self.packet_handler.tx_rx_packet(packet).ok()? {
            crate::packet_handler::RxStatus::Success(Some(packet)) => packet,
            _ => return None,
        };
        Some(status_packet.extract_data())
    }

    pub fn cleanup(&mut self) {
        for motor_id in self.motor_ids.clone() {
            self.act(motor_id, Command::WriteTorqueSwitch(false))
                .unwrap();
        }
    }
}
