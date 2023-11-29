use std::str;
use crate::crc::valid;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongLength,
    FitTextMissing,
    ChecksumFailed,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_length: u32,
    pub header_length: u8,
}

impl Header {
    pub fn from(from: &[u8]) -> Result<Header, Error> {
        let header_length = *from.first().unwrap();

        if from.len() < header_length.into() {
            return Err(Error::WrongLength);
        }

        let header = &from[..header_length.into()];

        if str::from_utf8(&header[8..=11]) != Ok(".FIT") {
            return Err(Error::FitTextMissing);
        }

        // 12 Byte headers are legacy and don't contain a checksum
        if header_length >= 14 {
            let checksum = u16::from_le_bytes([header[12], header[13]]);

            if checksum > 0 && !valid(&header[0..12], checksum) {
                return Err(Error::ChecksumFailed);
            }
        }

        let protocol_version = *header.get(1).unwrap();
        let profile_version = u16::from_le_bytes([header[2], header[3]]);
        let data_length = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

        Ok(Header {
            protocol_version,
            profile_version,
            data_length,
            header_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEGACY_HEADER: [u8; 12] = [12, 16, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84];
    const VALID_HEADER: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 158, 67];
    const NO_FIT: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 0, 0, 0, 0, 158, 67];
    const INVALID_HEADER: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 158, 66];
    const NO_CRC: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 0, 0];

    #[test]
    fn properties() {
        let header = Header::from(&VALID_HEADER).unwrap();

        assert_eq!(header.protocol_version, 32);
        assert_eq!(header.profile_version, 2147);
        assert_eq!(header.data_length, 94080);
        assert_eq!(header.header_length, 14);
    }

    #[test]
    fn wrong_length() {
        let result = Header::from(&VALID_HEADER[0..11]);

        assert_eq!(result, Err(Error::WrongLength));
    }

    #[test]
    fn no_fit_text() {
        let result = Header::from(&NO_FIT);

        assert_eq!(result, Err(Error::FitTextMissing));
    }

    #[test]
    fn invalid_crc() {
        let result = Header::from(&INVALID_HEADER);

        assert_eq!(result, Err(Error::ChecksumFailed));
    }

    #[test]
    fn no_crc_legacy() {
        let result = Header::from(&LEGACY_HEADER);

        assert!(result.is_ok());
    }

    #[test]
    fn no_crc() {
        let result = Header::from(&NO_CRC);

        assert!(result.is_ok());
    }
}
