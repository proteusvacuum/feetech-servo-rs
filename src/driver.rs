use crate::{commands::Command, packet_handler::PacketHandler};

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
    /// - This driver currently assumes the serial baud rate is fixed at **1 Mbps** (1,000,000 baud).
    ///
    /// # Panics
    /// This function will panic if invalid port names are provided
    pub fn new(port_name: &str) -> Self {
        let baud_rate = 1_000_000;
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
    pub fn act(&mut self, motor_id: u8, command: Command) -> Option<u16> {
        let packet = command.to_instruction_packet(motor_id);
        let status_packet = match self.packet_handler.tx_rx_packet(packet).ok()? {
            crate::packet_handler::RxStatus::Success(Some(packet)) => packet,
            // TODO: Handle all other cases!
            _ => return None,
        };
        Some(status_packet.extract_data())
    }
}
