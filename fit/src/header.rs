use crate::crc::valid;
use std::io::{Read, Seek, SeekFrom};
use std::str;

#[derive(Debug, PartialEq)]
pub enum Error {
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
    pub fn from(from: &mut (impl Read + Seek)) -> Result<Header, Error> {
        let mut header_bytes: [u8; 1] = [0];
        let _ = from.read(&mut header_bytes);
        let header_length = header_bytes[0];

        let mut protocol_version_bytes: [u8; 1] = [0];
        let _ = from.read(&mut protocol_version_bytes);
        let protocol_version = protocol_version_bytes[0];

        let mut profile_version_bytes: [u8; 2] = [0; 2];
        let _ = from.read(&mut profile_version_bytes);
        let profile_version = u16::from_le_bytes(profile_version_bytes);

        let mut data_length_bytes: [u8; 4] = [0; 4];
        let _ = from.read(&mut data_length_bytes);
        let data_length = u32::from_le_bytes(data_length_bytes);

        let mut fit_text_bytes: [u8; 4] = [0; 4];
        let _ = from.read(&mut fit_text_bytes);
        if str::from_utf8(&fit_text_bytes) != Ok(".FIT") {
            return Err(Error::FitTextMissing);
        }

        // 12 Byte headers are legacy and don't contain a checksum
        if header_length > 12 {
            const CHECKSUM_SIZE: usize = 2;
            const CHECKSUMMED_LENGTH: usize = 12;

            let mut checksum_bytes: [u8; CHECKSUM_SIZE] = [0; CHECKSUM_SIZE];
            let _ = from.read(&mut checksum_bytes);
            let checksum = u16::from_le_bytes(checksum_bytes);

            let _ = from.rewind();
            let mut header:[u8; CHECKSUMMED_LENGTH] = [0; CHECKSUMMED_LENGTH];
            let _ = from.read_exact(&mut header);

            // Move forward 2 bytes to ignore the checksum
            let _ = from.seek(SeekFrom::Current(CHECKSUM_SIZE as i64));

            if checksum > 0 && !valid(&header, checksum) {
                return Err(Error::ChecksumFailed);
            }
        }

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
    use std::io::Cursor;

    use super::*;

    const LEGACY_HEADER: [u8; 12] = [12, 16, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84];
    const VALID_HEADER: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 158, 67];
    const NO_FIT: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 0, 0, 0, 0, 158, 67];
    const INVALID_HEADER: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 158, 66];
    const NO_CRC: [u8; 14] = [14, 32, 99, 8, 128, 111, 1, 0, 46, 70, 73, 84, 0, 0];

    #[test]
    fn properties() {
        let header = Header::from(&mut Cursor::new(VALID_HEADER.as_slice())).unwrap();

        assert_eq!(header.protocol_version, 32);
        assert_eq!(header.profile_version, 2147);
        assert_eq!(header.data_length, 94080);
        assert_eq!(header.header_length, 14);
    }

    #[test]
    fn leaves_cursor_in_correct_place() {
        let mut cursor = Cursor::new(VALID_HEADER.as_slice());
        let _header = Header::from(&mut cursor).unwrap();

        let cursor_position = cursor.stream_position().unwrap();

        assert_eq!(cursor_position, 14);
    }

    #[test]
    fn no_fit_text() {
        let result = Header::from(&mut Cursor::new(NO_FIT.as_slice()));

        assert_eq!(result, Err(Error::FitTextMissing));
    }

    #[test]
    fn invalid_crc() {
        let result = Header::from(&mut Cursor::new(INVALID_HEADER.as_slice()));

        assert_eq!(result, Err(Error::ChecksumFailed));
    }

    #[test]
    fn no_crc_legacy() {
        let result = Header::from(&mut Cursor::new(LEGACY_HEADER.as_slice()));

        assert!(result.is_ok());
    }

    #[test]
    fn no_crc() {
        let result = Header::from(&mut Cursor::new(NO_CRC.as_slice()));

        assert!(result.is_ok());
    }
}
