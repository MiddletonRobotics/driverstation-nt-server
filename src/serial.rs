use serialport::SerialPort;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub struct Arduino {
    port: Box<dyn SerialPort>
}

impl Arduino {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, serialport::Error> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10000))
            .open()?;

        Ok(Self {
            port
        })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.port.write_all(data)?;
        self.port.flush()?;

        Ok(())
    }

    pub fn read_bytes(&mut self) -> Result<u8, std::io::Error> {
        let mut buffer = [0u8; 1];
        self.port.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }
}

pub fn list_ports() -> Result<(), serialport::Error> {
    let ports = serialport::available_ports()?;

    for port in ports {
        println!("Port: {}", port.port_name);
        println!("Type: {:?}", port.port_type);
    }

    Ok(())
}
