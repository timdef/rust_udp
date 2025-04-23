    let udp_datagram_bytes: &[u8] = &[
        0b1100_1111,
        0b1011_0011, // Source Port (53123)
        0b0000_0000,
        0b0011_0101, // Destination Port (53 - DNS)
        0b0000_0000,
        0b0000_1100, // Length (12 bytes - 8 for header + 4 for "test")
        0b1010_1011,
        0b1100_1101, // Checksum
        0b0111_0100, // T - Data
        0b0110_0101, // e
        0b0111_0011, // s
        0b0111_0100, // t
    ];