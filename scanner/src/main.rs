use std::env;
use std::net::{IpAddr};

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

    println!("Scanning Ports: {:?} of IP: {}", scan_args.ports, scan_args.ip);
}

fn parse_port_range(range: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid port range format. Please use <START_PORT>-<END_PORT>".into());
    }
    let start: u16 = parts[0].parse()?;
    let end: u16 = parts[1].parse()?;
    Ok((start..=end).collect())
}

fn main() {
    interface();
}
