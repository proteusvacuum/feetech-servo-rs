use crate::packet_handler::{Instruction, InstructionPacket, PacketHandler};

pub enum Command {
    Ping,
    ReadId,
    WriteTorqueSwitch(bool),
}

impl Command {
    fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket {
        match self {
            Command::Ping => InstructionPacket::new(motor_id, Instruction::Ping.into(), &[]),
            Command::ReadId => {
                InstructionPacket::new(motor_id, Instruction::Read.into(), &[0x5, 1])
            }
            Command::WriteTorqueSwitch(value) => {
                InstructionPacket::new(motor_id, Instruction::Write.into(), &[0x28, *value as u8])
            }
        }
    }
}

pub struct Motor<'a> {
    handler: &'a mut PacketHandler,
    id: u8,
}

impl<'a> Motor<'a> {
    pub fn new(handler: &'a mut PacketHandler, id: u8) -> Self {
        Self { handler, id }
    }

    pub fn act(&mut self, command: Command) {
        let packet = command.to_instruction_packet(self.id);
        dbg!(self.handler.tx_rx_packet(packet));
    }
}
