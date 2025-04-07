use crate::{
    commands::{IntoInstructionPacket, ReadCommand, WriteCommand},
    packet_handler::PacketHandler,
};

pub struct Driver {
    packet_handler: PacketHandler,
}

impl Driver {
    /// Creates a new `Driver` instance for a chain of servos connected to a single serial port.
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of the serial port (e.g., `"/dev/ttyUSB0"` or `"COM3"`).
    ///
    /// # Example
    /// ```no_run
    /// use feetech_servo_rs::Driver;
    ///
    /// let driver = Driver::new("/dev/ttyUSB0");
    /// ```
    ///
    /// # Notes
    /// - If you need to specify a different baud rate, use [`Driver::with_baud_rate`] instead.
    ///
    /// # Panics
    /// This function will panic if invalid port names are provided
    pub fn new(port_name: &str) -> Self {
        Self::with_baud_rate(port_name, 1_000_000)
    }

    /// Creates a new `Driver` instance for a chain of servos connected to a single serial port,
    /// with a user-specified baud rate.
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of the serial port (e.g., `"/dev/ttyUSB0"` or `"COM3"`).
    /// * `baud_rate` - The baud rate to use (e.g., `1_000_000` for 1 Mbps).
    ///
    /// # Example
    /// ```no_run
    /// use feetech_servo_rs::Driver;
    ///
    /// let driver = Driver::with_baud_rate("/dev/ttyUSB0", 57600);
    /// ```
    ///
    /// # Panics
    /// This function will panic if an invalid port name is provided.
    pub fn with_baud_rate(port_name: &str, baud_rate: u32) -> Self {
        Self {
            packet_handler: PacketHandler::new(port_name, baud_rate),
        }
    }

    /// Sends a target command to a single servo
    /// # Arguments
    ///
    /// * `motor_id` - The ID of the servo (`1..=253`)
    /// * `command` - The [`Command`] to send (e.g., `ReadCurrentPosition`, `WriteTargetPosition`)
    /// # Returns
    ///
    /// * `Some(u16)` - The data extracted from the servo's response packet (e.g. current position).
    /// * `None` - If the command was sent to the broadcast address `0xFE`.
    ///
    /// # Example
    /// ```no_run
    /// use feetech_servo_rs::Driver;
    /// use feetech_servo_rs::Command;
    /// let motor_id = 1u8;
    /// let mut driver = Driver::new("/dev/ttyUSB0");
    /// let current_position: u16 = driver.act(motor_id, Command::ReadCurrentPosition).unwrap();
    /// driver.act(motor_id, Command::WriteTargetPosition(current_position + 5u16)).unwrap();
    /// ```
    fn act<T: IntoInstructionPacket>(&mut self, motor_id: u8, command: T) -> Option<u16> {
        let packet = command.to_instruction_packet(motor_id);
        let status_packet = match self.packet_handler.tx_rx_packet(packet).ok()? {
            crate::packet_handler::RxStatus::Success(Some(packet)) => packet,
            // TODO: Handle all other cases!
            _ => return None,
        };
        Some(status_packet.extract_data())
    }

    pub fn read(&mut self, motor_id: u8, command: ReadCommand) -> Option<u16> {
        self.act::<ReadCommand>(motor_id, command)
    }

    pub fn write(&mut self, motor_id: u8, command: WriteCommand) -> Option<u16> {
        self.act::<WriteCommand>(motor_id, command)
    }
}
