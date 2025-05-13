// Source Port (2 bytes): The port number of the sending application
// Destination Port (2 bytes): The port number of the receiving application
// Length (2 bytes): The length of the UDP datagram, including both header and data, measured in bytes
// Checksum (2 bytes): Used for error-checking of the header and data
use std::fmt;

struct UdpDatagram {
    source: Port,
    destination: Port,
    length: u16,
    checksum: u16,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
struct Port(u16);

impl TryFrom<&[u8]> for UdpDatagram {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err("Too short to be a UDPDatagram".to_string());
        }

        let source = Port(u16::from_be_bytes([bytes[0], bytes[1]]));
        let destination = Port(u16::from_be_bytes([bytes[2], bytes[3]]));
        let length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let checksum = u16::from_be_bytes([bytes[6], bytes[7]]);
        let payload = bytes[8..length as usize].to_vec();

        Ok(Self {
            source,
            destination,
            length,
            checksum,
            payload,
        })
    }
}

impl fmt::Display for UdpDatagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.payload))
    }
}

fn main() {
    println!("\n•• Δεcοδιηg Sεcrεt Μεssαgε •••\n");

    match UdpDatagram::try_from(SECRET_MESSAGE) {
        Ok(datagram) => {
            println!(
                "Source: {} Destination: {}, Length: {}, Checksum: {:b}",
                datagram.source.0, datagram.destination.0, datagram.length, datagram.checksum
            );
            println!("Secret message: {datagram}\n\n");
        }
        Err(e) => println!("Failed to parse datagram: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UDP_DATA: &[u8] = &[
        0b11001111, 0b10110011, // Source Port: 53171
        0b00000000, 0b00110101, // Destination Port: 53 (DNS)
        0b00000000, 0b00001100, // Length: 12 bytes (8 header + 4 data)
        0b10101011, 0b11001101, // Checksum
        // Payload: "test"
        0b01110100, 0b01100101, 0b01110011, 0b01110100,
    ];

    #[test]
    fn test_parse_udp_data() {
        let datagram = UdpDatagram::try_from(UDP_DATA);

        assert!(datagram.is_ok());
    }

    #[test]
    fn test_parse_source() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(datagram.source, Port(53171));
    }

    #[test]
    fn test_parse_destination() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(datagram.destination, Port(53));
    }

    #[test]
    fn test_parse_length() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(datagram.length, 12);
    }

    #[test]
    fn test_parse_checksum() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(datagram.checksum, 0b10101011_11001101);
    }

    #[test]
    fn test_parse_payload() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(
            datagram.payload,
            [0b01110100, 0b01100101, 0b01110011, 0b01110100]
        );
    }

    #[test]
    fn test_display() {
        let datagram = UdpDatagram::try_from(UDP_DATA).unwrap();

        assert_eq!(datagram.to_string(), "test");
    }

    #[test]
    fn test_data_too_short() {
        let too_short_data: &[u8] = &[
            0b11001111, 0b10110011, // Source Port: 53171
            0b00000000, 0b00110101, // Destination Port: 53 (DNS)
            0b00000000, 0b00001100, // Length: 12 bytes
            0b10101011, // Only one byte of checksum - incomplete!
        ];

        let result = UdpDatagram::try_from(too_short_data);

        assert!(result.is_err());
    }
}

const SECRET_MESSAGE: &[u8] = &[
    0b11001111, 0b10110011, // Source Port: 53171
    0b00000000, 0b00110101, // Destination Port: 53 (DNS)
    0b00000000, 0b01010110, // Length: 86 bytes
    0b10101011, 0b11001101, // Checksum
    0b01010100, 0b01101000, 0b01100101, 0b00100000, 0b01110011, 0b01101011, 0b01111001, 0b00100000,
    0b01100001, 0b01100010, 0b01101111, 0b01110110, 0b01100101, 0b00100000, 0b01110100, 0b01101000,
    0b01100101, 0b00100000, 0b01110000, 0b01101111, 0b01110010, 0b01110100, 0b00100000, 0b01110111,
    0b01100001, 0b01110011, 0b00100000, 0b01110100, 0b01101000, 0b01100101, 0b00100000, 0b01100011,
    0b01101111, 0b01101100, 0b01101111, 0b01110010, 0b00100000, 0b01101111, 0b01100110, 0b00100000,
    0b01100001, 0b00100000, 0b01010101, 0b01000100, 0b01010000, 0b00100000, 0b01110000, 0b01100001,
    0b01100011, 0b01101011, 0b01100101, 0b01110100, 0b00101100, 0b00100000, 0b01110100, 0b01110101,
    0b01101110, 0b01100101, 0b01100100, 0b00100000, 0b01110100, 0b01101111, 0b00100000, 0b01100001,
    0b00100000, 0b01100100, 0b01100101, 0b01100001, 0b01100100, 0b00100000, 0b01100011, 0b01101000,
    0b01100001, 0b01101110, 0b01101110, 0b01100101, 0b01101100, 0b00101110,
];
