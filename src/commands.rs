use crate::instruction::Instruction;
use crate::packets::InstructionPacket;

pub enum Command {
    Ping,
    ReadId,
    WriteTorqueSwitch(bool),
    ReadCurrentPosition,

    WriteTargetPosition(u16),
}

impl Command {
    pub fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket {
        match self {
            Command::Ping => InstructionPacket::new(motor_id, Instruction::Ping, &[]),
            Command::ReadId => InstructionPacket::new(motor_id, Instruction::Read, &[0x5, 1]),
            Command::WriteTorqueSwitch(value) => {
                InstructionPacket::new(motor_id, Instruction::Write, &[0x28, *value as u8])
            }
            Command::ReadCurrentPosition => {
                InstructionPacket::new(motor_id, Instruction::Read, &[0x38, 2])
            }
            Command::WriteTargetPosition(target_position) => {
                let low: u8 = (*target_position >> 8) as u8;
                let high: u8 = (*target_position & 0x00FF) as u8;
                InstructionPacket::new(motor_id, Instruction::Write, &[0x2A, high, low])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_instruction_packet() {
        let motor_id = 0x25;
        let cases = vec![
            (
                Command::Ping,
                InstructionPacket::new(motor_id, Instruction::Ping, &[]),
            ),
            (
                Command::WriteTargetPosition(1025),
                InstructionPacket::new(motor_id, Instruction::Write, &[0x2A, 0x1, 0x4]),
            ),
        ];
        for (command, instruction_packet) in cases {
            assert_eq!(command.to_instruction_packet(motor_id), instruction_packet);
        }
    }
}
