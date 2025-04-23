// Can show the recc'ed with
// Source Port (2 bytes): The port number of the sending application
// Destination Port (2 bytes): The port number of the receiving application
// Length (2 bytes): The length of the UDP datagram, including both header and data, measured in bytes
// Checksum (2 bytes): Used for error-checking of the header and data
use std::net::UdpSocket;
use std::process;

#[derive(Debug, Clone)]
struct UdpDatagram {
    source: Port,
    destination: Port,
    length: u16,
    checksum: Checksum,
    payload: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Port(u16);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Checksum(u16);

impl UdpDatagram {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        
        if bytes.len() < 8 {
            return None;
        }

        let length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let Ok(payload) = String::from_utf8(bytes[8..length as usize].to_vec()) else { return None };

        Some(UdpDatagram {
            source: Port(u16::from_be_bytes([bytes[0], bytes[1]])),
            destination: Port(u16::from_be_bytes([bytes[2], bytes[3]])),
            length,
            checksum: Checksum(u16::from_be_bytes([bytes[6], bytes[7]])),
            payload,
        })
    }
}

fn main() {
    println!("Listening for datagrams...");
    let socket = match UdpSocket::bind("127.0.0.1:8080") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error binding to socket: {}", e);
            process::exit(1);
        }
    };
    let mut buff = [0; 1024];

    match socket.recv_from(&mut buff) {
        Ok((size, src_addr)) => {
            println!("{buff:?}");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error receiving datagram {}", e);
            process::exit(1);
        }
    }

    // match UdpDatagram::from_bytes(udp_datagram_bytes) {
    //     Some(datagram) => {
    //         print!("Parsed Datagram: {datagram:?}");
    //         println!("Source: {:?}", datagram.source);
    //         println!("Destination: {:?}", datagram.destination);
    //         println!("Length: {:?}", datagram.length);
    //         println!("Checksum: {:?}", datagram.checksum);
    //         println!("Payload: {:?}", datagram.payload);
    //     },
    //     None => println!("Could not parse datagram."),
    // }
}
