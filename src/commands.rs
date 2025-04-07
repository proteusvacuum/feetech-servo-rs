use crate::instruction::Instruction;
use crate::memory_location::{MemoryValue, ACCELERATION, CURRENT_POSITION, ID, TEMPERATURE};
use crate::packets::InstructionPacket;

pub trait IntoInstructionPacket {
    fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket;
}

pub enum WriteCommand {
    Acceleration(u8),
    TargetPosition(u16),
    TorqueSwitch(bool),
}

impl IntoInstructionPacket for WriteCommand {
    fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket {
        match self {
            WriteCommand::Acceleration(acceleration) => {
                InstructionPacket::write_to_memory_location(
                    motor_id,
                    ACCELERATION,
                    MemoryValue::U8(*acceleration),
                )
            }
            WriteCommand::TargetPosition(_) => todo!(),
            WriteCommand::TorqueSwitch(_) => todo!(),
        }
    }
}

pub enum ReadCommand {
    Acceleration,
    Id,
    CurrentPosition,
    Temperature,
}

impl IntoInstructionPacket for ReadCommand {
    fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket {
        match self {
            ReadCommand::Id => InstructionPacket::read_from_memory_location(motor_id, ID),
            ReadCommand::CurrentPosition => {
                InstructionPacket::read_from_memory_location(motor_id, CURRENT_POSITION)
            }
            ReadCommand::Temperature => {
                InstructionPacket::read_from_memory_location(motor_id, TEMPERATURE)
            }
            ReadCommand::Acceleration => {
                InstructionPacket::read_from_memory_location(motor_id, ACCELERATION)
            }
        }
    }
}

pub enum Command {
    Ping,
    WriteTorqueSwitch(bool),
    ReadCurrentPosition,
    ReadTemperature,
    WriteTargetPosition(u16),
    WriteAcceleration(u8),
}

impl Command {
    pub fn to_instruction_packet(&self, motor_id: u8) -> InstructionPacket {
        match self {
            Command::Ping => InstructionPacket::new(motor_id, Instruction::Ping, &[]),
            Command::WriteTorqueSwitch(value) => {
                InstructionPacket::new(motor_id, Instruction::Write, &[0x28, *value as u8])
            }
            Command::ReadCurrentPosition => {
                InstructionPacket::new(motor_id, Instruction::Read, &[0x38, 2])
            }
            Command::ReadTemperature => {
                InstructionPacket::new(motor_id, Instruction::Read, &[0x3F, 1])
            }
            Command::WriteTargetPosition(target_position) => {
                let low: u8 = (*target_position >> 8) as u8;
                let high: u8 = (*target_position & 0x00FF) as u8;
                InstructionPacket::new(motor_id, Instruction::Write, &[0x2A, high, low])
            }
            Command::WriteAcceleration(acceleration) => {
                InstructionPacket::new(motor_id, Instruction::Write, &[0x29, *acceleration])
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
            (
                Command::ReadTemperature,
                InstructionPacket::new(motor_id, Instruction::Read, &[0x3F, 1]),
            ),
        ];
        for (command, instruction_packet) in cases {
            assert_eq!(command.to_instruction_packet(motor_id), instruction_packet);
        }
    }
}
