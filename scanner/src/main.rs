use std::env;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;
use iced::{Element, Application, Command, Settings, Theme};
use iced::widget::{Column, button, text};


struct ScanArgs {   // State
    ip: IpAddr,
    ports: Vec<u16>,
}

// fn interface() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 5 {
//         eprintln!("Not enough arguments provided. Usage: scanner -ip <IP_ADDRESS> -port <PORT_RANGE>");
//         std::process::exit(1);
//     }

//     let ip_arg_index = args.iter().position(|arg| arg == "-ip").expect("Missing -ip argument");
//     let ip_str = &args[ip_arg_index + 1];
//     let ip: IpAddr = ip_str.parse().expect("Invalid IP address");

//     let port_arg_index = args.iter().position(|arg| arg == "-port").expect("Missing -port argument");
//     let port_range_str = &args[port_arg_index + 1];
//     let ports: Vec<u16> = parse_port_range(port_range_str).expect("Invalid port range");

//     let scan_args = ScanArgs {
//         ip,
//         ports,
//     };

//     scanner(scan_args.ip, &scan_args.ports);
// }

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

    println!("Open ports: {}", open_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
    println!("Closed ports: {}", closed_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
    println!("Filtered ports: {}", filtered_ports.iter().map(|&port| port.to_string()).collect::<Vec<String>>().join(", "));
    
    // Return an object with all three variables
    let open_ports = Vec::new();
    let closed_ports = Vec::new();
    let filtered_ports = Vec::new();
     return (open_ports, closed_ports, filtered_ports)
    
}

#[derive(Debug, Clone, Copy)]
pub enum Message {  // Messages
    ScanPressed,
}

// impl ScanArgs {  // View Logic
//     pub fn view(&self) -> Column<Message> {
//         column![    // We use a column: a simple vertical layout
//             button("Scan").on_press(Message::ScanPressed),
//             text(self.ip).size(50),
//         ] 
//     }
    
//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::ScanPressed => {
//                 interface();
//             }
//         }
//     }
// }

impl Application for ScanArgs {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (ScanArgs, Command<Message>) {
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

        (scan_args, Command::none())
    }

    fn title(&self) -> String {
        String::from("Simple Port Scanner")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ScanPressed => {
                println!("Button clicked! \n");
                scanner(self.ip, &self.ports);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(button("Scan").on_press(Message::ScanPressed))
            .push(text("Display IP and Port information here")) // Modify this to display relevant information
            .into()
    }
}


pub fn main() -> iced::Result {
    ScanArgs::run(Settings::default())
}
