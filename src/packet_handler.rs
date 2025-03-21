use thiserror::Error;

use crate::{
    packets::{InstructionPacket, StatusPacket},
    serial::Serial,
};

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
    port: Serial,
}

impl PacketHandler {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self {
            port: Serial::new(port_name, baud_rate).expect("error connecting to serial port"),
        }
    }

    pub fn tx_rx_packet(&mut self, packet: InstructionPacket) -> RxResult {
        let result = self.tx_packet(&packet);
        match result {
            Ok(_) => {
                if packet.id == 0xFE {
                    // WARNING : Status Packet will not be returned if Broadcast ID(0xFE) is used.
                    return Ok(RxStatus::Success(None));
                }
                self.rx_packet()
            }
            Err(_) => todo!(),
        }
    }

    pub fn tx_packet(&mut self, packet: &InstructionPacket) -> TxResult {
        if packet.get_total_packet_length() > 250 {
            return Err(TxError::TxError);
        }
        match self.port.write(&packet.as_bytes()) {
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
