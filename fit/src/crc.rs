pub fn valid(data: &[u8], checksum: u16) -> bool {
    self::calculate(data) == checksum
}

fn calculate(from: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    let crc_table: [u16; 16] = [
        0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
        0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
    ];

    for byte in from.iter() {
        let tmp = crc_table[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ crc_table[(byte & 0xF) as usize];

        let tmp = crc_table[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ crc_table[((byte >> 4) & 0xF) as usize];
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [u8; 12] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84];

    #[test]
    fn is_valid() {
        assert!(valid(&DATA, 17310));
    }

    #[test]
    fn is_invalid() {
        assert!(!valid(&DATA, 2929));
    }
}
