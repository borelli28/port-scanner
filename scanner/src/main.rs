use std::env;
use std::net::IpAddr;


struct ScanArgs {
    ip: IpAddr,
    ports: Vec<u16>,
}

fn interface() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Not enough arguments provided. Usage: scanner -ip <IP_ADDRESS> -port <PORT>");
        std::process::exit(1);
    }

    let ip_arg_index = args.iter().position(|arg| arg == "-ip").expect("Missing -ip argument");
    let ip: IpAddr = args[ip_arg_index + 1].parse().expect("Invalid IP address");

    let port_arg_index = args.iter().position(|arg| arg == "-port").expect("Missing -port argument");
    let port: u16 = args[port_arg_index + 1].parse().expect("Invalid port number");

    ScanArgs {ip, port};

    println!("Scanning Port: {} of IP: {}", port, ip);
}

fn main() {
    interface();
}