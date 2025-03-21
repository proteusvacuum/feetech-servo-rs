use std::fmt::Display;

use crate::instruction::Instruction;
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
