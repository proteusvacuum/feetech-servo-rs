#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;
use thiserror::Error;

use crate::serial::Serial;

fn compute_checksum(id: u8, length: u8, instruction: u8, parameters: &[u8]) -> u8 {
    // https://emanual.robotis.com/docs/en/dxl/protocol1/#checksum-instruction-packet
    let mut checksum: u16 = 0; // avoid overflows, so set as u16
    checksum += id as u16;
    checksum += length as u16;
    checksum += instruction as u16;
    for param in parameters {
        checksum += *param as u16;
    }
    (!checksum & 0xff) as u8
}

enum Instruction {
    Ping,
    Read,
    Write,
    RegWrite,
    Action,
    SyncWrite,
    SyncRead,
}

impl Instruction {
    fn length(&self) -> u8 {
        // TODO: Do we want to do this like this?
        // It should be able to calculate it by itself by counting something,
        // I'm just not sure what it is counting yet
        match self {
            Instruction::Ping => 2,
            Instruction::Read => todo!(),
            Instruction::Write => todo!(),
            Instruction::RegWrite => todo!(),
            Instruction::Action => todo!(),
            Instruction::SyncWrite => todo!(),
            Instruction::SyncRead => todo!(),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Ping => 1,
            Instruction::Read => 2,
            Instruction::Write => 3,
            Instruction::RegWrite => 4,
            Instruction::Action => 5,
            Instruction::SyncWrite => 0x83,
            Instruction::SyncRead => 0x82,
        }
    }
}

#[derive(Debug)]
struct InstructionPacket {
    // https://emanual.robotis.com/docs/en/dxl/protocol1/#instruction-packet
    // header0: u8,
    // header1: u8,
    id: u8,
    length: u8,
    instruction: u8,
    parameters: Vec<u8>,
    checksum: u8,
}

impl InstructionPacket {
    fn new(id: u8, length: u8, instruction: u8, parameters: &[u8]) -> Self {
        Self {
            id,
            length,
            instruction,
            checksum: compute_checksum(id, length, instruction, &parameters),
            parameters: parameters.to_vec(),
        }
    }

    fn get_total_packet_length(&self) -> u32 {
        // "Header0, Header1, ID, Length" is added to the length of the packet
        self.length as u32 + 4
    }

    fn as_bytes(&self) -> Vec<u8> {
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
    fn new(header: &[u8], id: u8, length: u8, error: u8, params: &[u8], checksum: u8) -> Self {
        assert!(header == [0xFF, 0xFF]);
        let computed_checksum = compute_checksum(id, length, error, params);
        // assert!(checksum == computed_checksum); // TODO: handle this

        Self {
            id,
            length,
            error,
            params: params.to_vec(),
            checksum,
        }
    }
}

impl Display for StatusPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, length: {}, error: {}, checksum: {}",
            self.id, self.length, self.error, self.checksum
        )?;
        if self.params.len() == 2 {
            //TODO: take into account Big Endian if the protocol demands it
            let word = u16::from_le_bytes([self.params[0], self.params[1]]);
            write!(f, ", data: {}", word)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Endianness {
    Little,
    Big,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TxStatus {
    Success,
}

#[derive(Debug, Error)]
pub enum TxError {
    #[error("port is currently busy")]
    PortBusy,
    #[error("tx failed")]
    TxFail,
    #[error("tx encountered an error")]
    TxError,
    #[error("not available")]
    NotAvailable,
}

type TxResult = Result<TxStatus, TxError>;

#[derive(Debug)]
pub enum RxStatus {
    Success(Option<StatusPacket>),
    RxWaiting,
}

#[derive(Debug, Error)]
pub enum RxError {
    #[error("port is currently busy")]
    PortBusy,
    #[error("rx failed")]
    RxFail,
    #[error("rx timeout")]
    RxTimeout,
    #[error("rx corrupt")]
    RxCorrupt,
    #[error("not available")]
    NotAvailable,
}

pub type RxResult = Result<RxStatus, RxError>;

#[derive(Debug)]
pub struct PacketHandler {
    endianness: Endianness,
    port: Serial,
}

impl PacketHandler {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self {
            endianness: Endianness::Little,
            port: Serial::new(port_name, baud_rate).expect("error connecting to serial port"),
        }
    }

    pub fn ping(&mut self, motor_id: u8) -> RxResult {
        // TODO: Length is hardcoded here
        let tx_packet = InstructionPacket::new(motor_id, 2, Instruction::Ping.into(), &[]);
        self.tx_rx_packet(tx_packet)?;

        let read_packet = InstructionPacket::new(motor_id, 4, Instruction::Read.into(), &[0x38, 2]);
        self.tx_rx_packet(read_packet)
    }

    pub fn move_motor(&mut self, motor_id: u8, target_position: u16) {
        let low: u8 = (target_position >> 8) as u8;
        let high: u8 = (target_position & 0x00FF) as u8;

        let write_packet =
            InstructionPacket::new(motor_id, 5, Instruction::Write.into(), &[0x2A, high, low]);
        dbg!(&write_packet);
        dbg!(write_packet.as_bytes());
        self.tx_rx_packet(write_packet);
    }

    fn tx_rx_packet(&mut self, packet: InstructionPacket) -> RxResult {
        let result = dbg!(self.tx_packet(&packet));
        match result {
            Ok(status) => {
                if packet.id == 0xFE {
                    // WARNING : Status Packet will not be returned if Broadcast ID(0xFE) is used.
                    return Ok(RxStatus::Success(None));
                }
                return self.rx_packet();
            }
            Err(_) => todo!(),
        }
    }

    fn tx_packet(&mut self, packet: &InstructionPacket) -> TxResult {
        if packet.get_total_packet_length() > 250 {
            return Err(TxError::TxError);
        }
        match self.port.write(&dbg!(packet.as_bytes())) {
            Ok(_) => Ok(TxStatus::Success),
            Err(_) => Err(TxError::TxFail),
        }
    }

    fn rx_packet(&mut self) -> RxResult {
        let mut header: [u8; 2] = [0; 2];
        self.port
            .read_exact(&mut header)
            .expect("reading header failed"); // TODO
        assert!(header == [0xFF, 0xFF]); // TODO

        let mut meta: [u8; 3] = [0; 3];
        self.port
            .read_exact(&mut meta)
            .expect("reading metadata contents failed"); // TODO

        let length = meta[1]; // Length = number of Parameters + 2
        let num_params = (length - 2) as usize;
        let mut params = vec![0u8; num_params];

        self.port
            .read_exact(&mut params)
            .expect("reading param contents failed"); // TODO
        let mut checksum: [u8; 1] = [0; 1];
        self.port
            .read_exact(&mut checksum)
            .expect("reading checksum contents failed"); // TODO
        let status_packet =
            StatusPacket::new(&header, meta[0], meta[1], meta[2], &params, checksum[0]);
        Ok(RxStatus::Success(Some(status_packet)))
    }
}
