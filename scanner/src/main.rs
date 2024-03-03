use std::env;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;


struct ScanArgs {   // State
    ip: IpAddr,
    ports: Vec<u16>,
    open_ports: Vec<u16>,
    closed_ports: Vec<u16>,
    filtered_ports: Vec<u16>,
}

fn parse_port_range(range: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid port range format. Please use <START_PORT>-<END_PORT>".into());
        }
        let start: u16 = parts[0].parse()?;
        let end: u16 = parts[1].parse()?;
        Ok((start..=end).collect())
    } else {
        let port: u16 = range.parse()?;
        Ok(vec![port])
    }
}

fn scanner(ip: IpAddr, ports: &[u16]) -> (Vec<u16>, Vec<u16>, Vec<u16>) {
    let mut open_ports = Vec::new();
    let mut closed_ports = Vec::new();
    let mut filtered_ports = Vec::new();

    for &port in ports {
        let socket = SocketAddr::new(ip, port);
        match TcpStream::connect_timeout(&socket, Duration::from_secs(3)) {
            Ok(_) => open_ports.push(port),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::ConnectionRefused {
                    closed_ports.push(port);
                } else {
                    filtered_ports.push(port);
                }
            }
        }
    }

    return (open_ports, closed_ports, filtered_ports) 
}


pub fn main() {
    println!("Hello");
}
