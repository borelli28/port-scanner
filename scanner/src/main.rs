use std::env;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;


struct ScanArgs {
    ip: IpAddr,
    ports: Vec<u16>,
}

fn interface() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Not enough arguments provided. Usage: scanner -ip <IP_ADDRESS> -port <PORT_RANGE>");
        std::process::exit(1);
    }

    let ip_arg_index = args.iter().position(|arg| arg == "-ip").expect("Missing -ip argument");
    let ip_str = &args[ip_arg_index + 1];
    let ip: IpAddr = ip_str.parse().expect("Invalid IP address");

    let port_arg_index = args.iter().position(|arg| arg == "-port").expect("Missing -port argument");
    let port_range_str = &args[port_arg_index + 1];
    let ports: Vec<u16> = parse_port_range(port_range_str).expect("Invalid port range");

    let scan_args = ScanArgs {
        ip,
        ports,
    };

    scanner(scan_args.ip, &scan_args.ports);
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

fn scanner(ip: IpAddr, ports: &[u16]) {
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

    println!("Open ports: {}", open_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
    println!("Closed ports: {}", closed_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
    println!("Filtered ports: {}", filtered_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
}

fn main() {
    interface();
}
