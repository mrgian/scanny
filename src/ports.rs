use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::consts::PORTS;
use crate::model::Port;
use crate::Subdomain;

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = PORTS
        .into_iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();

    subdomain
}

fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("creating socket addresses")
        .collect();

    if socket_addresses.len() == 0 {
        return Port {
            port: port,
            is_open: false,
        };
    }

    println!("Scanning: {}", &socket_addresses[0]);

    let connection = TcpStream::connect_timeout(&socket_addresses[0], timeout);
    let is_open: bool;

    match connection {
        Ok(_c) => is_open = true,
        Err(_e) => is_open = false,
    }

    Port {
        port: port,
        is_open: is_open,
    }
}
