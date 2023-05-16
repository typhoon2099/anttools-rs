use std::str;

pub struct Header {
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32,
}

impl Header {
    pub fn from(from: &[u8]) -> Option<Header> {
        let header_size = *from.first().unwrap() as usize;

        if from.len() < header_size {
            return None
        }

        let header = &from[..header_size];

        if str::from_utf8(&header[8..=11]) != Ok(".FIT") {
            return None
        }

        let protocol_version = *header.get(1).unwrap();
        let profile_version = u16::from_le_bytes([header[2], header[3]]);
        let data_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

        Some(Header {
            protocol_version,
            profile_version,
            data_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_HEADER: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 158, 67];
    const NO_FIT: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 0, 0, 0, 0, 158, 67];

    #[test]
    fn properties() {
        let header = Header::from(&VALID_HEADER).unwrap();

        assert_eq!(header.protocol_version, 32);
        assert_eq!(header.profile_version, 2147);
        assert_eq!(header.data_size, 94080);
    }

    #[test]
    fn wrong_length() {
        let header = Header::from(&VALID_HEADER[0..11]);

        assert!(header.is_none());
    }

    #[test]
    fn no_fit_text() {
        let header = Header::from(&NO_FIT);

        assert!(header.is_none());
    }
}
