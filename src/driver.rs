use crate::{commands::Command, packet_handler::PacketHandler};

pub struct Driver<'a> {
    packet_handler: &'a mut PacketHandler,
}

impl<'a> Driver<'a> {
    pub fn new(packet_handler: &'a mut PacketHandler) -> Self {
        Self { packet_handler }
    }

    pub fn act(&mut self, motor_id: u8, command: Command) -> Option<u16> {
        let packet = command.to_instruction_packet(motor_id);
        let status_packet = match self.packet_handler.tx_rx_packet(packet).ok()? {
            crate::packet_handler::RxStatus::Success(Some(packet)) => packet,
            _ => return None,
        };
        Some(status_packet.extract_data())
    }
}
