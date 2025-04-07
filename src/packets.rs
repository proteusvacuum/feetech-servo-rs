use std::fmt::Display;

use crate::instruction::Instruction;
use crate::memory_location::{MemoryLocation, MemoryValue};
use crate::utils;

#[derive(Debug)]
pub struct StatusPacket {
    // https://emanual.robotis.com/docs/en/dxl/protocol1/#status-packetreturn-packet
    id: u8,
    length: u8,
    error: u8,
    params: Vec<u8>,
    checksum: u8,
}

impl StatusPacket {
    pub fn new(header: &[u8], id: u8, length: u8, error: u8, params: &[u8], checksum: u8) -> Self {
        assert!(header == [0xFF, 0xFF]);
        let computed_checksum = utils::compute_checksum(id, length, error, params);
        assert!(checksum == computed_checksum); // TODO: handle this

        Self {
            id,
            length,
            error,
            params: params.to_vec(),
            checksum,
        }
    }

    pub fn extract_data(&self) -> u16 {
        match self.params.len() {
            1 => self.params[0] as u16,
            2 => u16::from_le_bytes([self.params[0], self.params[1]]),
            _ => 0u16,
        }
    }
}

impl Display for StatusPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, length: {}, error: {}, checksum: {}, data: {}",
            self.id,
            self.length,
            self.error,
            self.checksum,
            self.extract_data()
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct InstructionPacket {
    // https://emanual.robotis.com/docs/en/dxl/protocol1/#instruction-packet
    // header0: u8,
    // header1: u8,
    pub id: u8,
    length: u8,
    instruction: u8,
    parameters: Vec<u8>,
    checksum: u8,
}

impl InstructionPacket {
    pub fn new(id: u8, instruction: Instruction, parameters: &[u8]) -> Self {
        let length: u8 = (parameters.len() + 2) as u8;
        let instruction: u8 = instruction.into();
        Self {
            id,
            length,
            instruction,
            checksum: utils::compute_checksum(id, length, instruction, parameters),
            parameters: parameters.to_vec(),
        }
    }

    pub fn read_from_memory_location(id: u8, memory_location: MemoryLocation) -> Self {
        Self::new(
            id,
            Instruction::Read,
            &[memory_location.addr, memory_location.size],
        )
    }

    pub fn write_to_memory_location(
        id: u8,
        memory_location: MemoryLocation,
        value: MemoryValue,
    ) -> Self {
        match value {
            MemoryValue::U16(val) => {
                assert_eq!(memory_location.size, 2);
                let low: u8 = (val >> 8) as u8;
                let high: u8 = (val & 0x00FF) as u8;
                Self::new(id, Instruction::Write, &[memory_location.addr, high, low])
            }
            MemoryValue::U8(val) => {
                assert_eq!(memory_location.size, 1);
                Self::new(id, Instruction::Write, &[memory_location.addr, val])
            }
            MemoryValue::Bool(val) => {
                assert_eq!(memory_location.size, 1);
                Self::new(id, Instruction::Write, &[memory_location.addr, val as u8])
            }
        }
    }

    pub fn get_total_packet_length(&self) -> u32 {
        // "Header0, Header1, ID, Length" is added to the length of the packet
        self.length as u32 + 4
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![
            0xFF, // The first 2 bytes are always 0xff.
            0xFF, // AKA. "Header"
            self.id,
            self.length,
            self.instruction,
        ];
        bytes.extend_from_slice(&self.parameters);
        bytes.push(self.checksum);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_packet_bytes() {
        let motor_id = 0x5;
        let cases = vec![
            (
                InstructionPacket::new(motor_id, Instruction::Ping, &[]),
                vec![0xff, 0xff, motor_id, 2, 1, 247],
            ),
            (
                // Read Current Location: 0X38
                InstructionPacket::new(motor_id, Instruction::Read, &[0x38]),
                vec![0xff, 0xff, motor_id, 3, 2, 0x38, 189],
            ),
            (
                // Write Target Location: 0x3A
                InstructionPacket::new(motor_id, Instruction::Write, &[0x3A, 12, 12]),
                vec![0xff, 0xff, motor_id, 5, 3, 0x3A, 12, 12, 160],
            ),
        ];
        for (instruction_packet, expected) in cases {
            assert_eq!(instruction_packet.as_bytes(), expected);
        }
    }
}
