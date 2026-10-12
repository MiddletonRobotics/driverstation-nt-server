use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn list_ports() -> Result<(), serialport::Error> {
    let ports = serialport::available_ports()?;

    for port in ports {
        println!("Port: {}", port.port_name);
        println!("Type: {:?}", port.port_type);
    }

    Ok(())
}

pub fn send_bytes(port_name: &str, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    thread::sleep(Duration::from_secs(2));

    port.write_all(bytes)?;
    port.flush()?;

    Ok(())
}