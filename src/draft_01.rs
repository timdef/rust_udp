// Source Port (2 bytes): The port number of the sending application
// Destination Port (2 bytes): The port number of the receiving application
// Length (2 bytes): The length of the UDP datagram, including both header and data, measured in bytes
// Checksum (2 bytes): Used for error-checking of the header and data
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
        println!("Length: {length}");
        let Ok(payload) = String::from_utf8(bytes[8..length as usize].to_vec()) else {
            return None;
        };

        Some(UdpDatagram {
            source: Port(u16::from_be_bytes([bytes[0], bytes[1]])),
            destination: Port(u16::from_be_bytes([bytes[2], bytes[3]])),
            length,
            checksum: Checksum(u16::from_be_bytes([bytes[6], bytes[7]])),
            payload,
        })
    }
}

fn main() -> std::io::Result<()> {
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
    let rfc_example: &[u8] = &[
        0b11001111, 0b10110011, // Source Port: 53171
        0b00000000, 0b00110101, // Destination Port: 53 (DNS)
        0b00000000, 0b00001100, // Length: 12 bytes (8 header + 4 data)
        0b10101011, 0b11001101, // Checksum
        // Payload: "test"
        0b01110100, 0b01100101, 0b01110011, 0b01110100,
    ];
    Ok(())
}



// let neuromancer_udp_message: &[u8] = &[
//     0b01010100,
//     0b01101000,
//     0b01100101,
//     0b00100000,
//     0b01110011,
//     0b01101011,
//     0b01111001,
//     0b00100000,
//     0b01100001,
//     0b01100010,
//     0b01101111,
//     0b01110110,
//     0b01100101,
//     0b00100000,
//     0b01110100,
//     0b01101000,
//     0b01100101,
//     0b00100000,
//     0b01110000,
//     0b01101111,
//     0b01110010,
//     0b01110100,
//     0b00100000,
//     0b01110111,
//     0b01100001,
//     0b01110011,
//     0b00100000,
//     0b01110100,
//     0b01101000,
//     0b01100101,
//     0b00100000,
//     0b01100011,
//     0b01101111,
//     0b01101100,
//     0b01101111,
//     0b01110010,
//     0b00100000,
//     0b01101111,
//     0b01100110,
//     0b00100000,
//     0b01100001,
//     0b00100000,
//     0b01010101,
//     0b01000100,
//     0b01010000,
//     0b00100000,
//     0b01110000,
//     0b01100001,
//     0b01100011,
//     0b01101011,
//     0b01100101,
//     0b01110100,
//     0b00101100,
//     0b00100000,
//     0b01110100,
//     0b01110101,
//     0b01101110,
//     0b01100101,
//     0b01100100,
//     0b00100000,
//     0b01110100,
//     0b01101111,
//     0b00100000,
//     0b01100001,
//     0b00100000,
//     0b01100100,
//     0b01100101,
//     0b01100001,
//     0b01100100,
//     0b00100000,
//     0b01100011,
//     0b01101000,
//     0b01100001,
//     0b01101110,
//     0b01101110,
//     0b01100101,
//     0b01101100,
//     0b00101110
// ];
