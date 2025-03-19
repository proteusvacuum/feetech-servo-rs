use serialport::SerialPort;

#[derive(Debug)]
pub struct Serial {
    port: Box<dyn SerialPort>,
}

impl Serial {
    pub fn new(port_name: &str, baud_rate: u32) -> std::io::Result<Self> {
        let port = serialport::new(port_name, baud_rate).open()?;
        Ok(Self { port })
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.port.write_all(data)?;
        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buffer)
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> std::io::Result<()> {
        self.port.read_exact(buffer)
    }
}
