use crate::memory_location::{
    MemoryValue, ACCELERATION, CURRENT_POSITION, ID, TARGET_POSITION, TEMPERATURE, TORQUE_SWITCH,
};
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
            WriteCommand::TargetPosition(position) => InstructionPacket::write_to_memory_location(
                motor_id,
                TARGET_POSITION,
                MemoryValue::U16(*position),
            ),
            WriteCommand::TorqueSwitch(value) => InstructionPacket::write_to_memory_location(
                motor_id,
                TORQUE_SWITCH,
                MemoryValue::Bool(*value),
            ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn command_instruction_packet() {
        let motor_id = 0x25;

        assert_eq!(
            WriteCommand::TargetPosition(1025).to_instruction_packet(motor_id),
            InstructionPacket::new(motor_id, Instruction::Write, &[0x2A, 0x1, 0x4])
        );
        assert_eq!(
            ReadCommand::Temperature.to_instruction_packet(motor_id),
            InstructionPacket::new(motor_id, Instruction::Read, &[0x3F, 1])
        );
    }
}
