use crate::crc;
use crate::header::Header;
use std::io::{Read, Seek};
use std::{fs::File, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Fit {
    pub protocol_version: u8,
    pub profile_version: u16,
    pub records: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    FileNotFound,
    FileNotValid,
}

impl Fit {
    pub fn from_file(path: PathBuf) -> Result<Fit, Error> {
        let file = File::open(path);
        match file {
            Ok(mut file) => Fit::from_bytes(&mut file),
            Err(_) => Err(Error::FileNotFound),
        }
    }

    pub fn from_bytes(from: &mut (impl Read + Seek)) -> Result<Fit, Error> {
        let header = match Header::from(from) {
            Ok(header) => header,
            Err(_error) => return Err(Error::FileNotValid),
        };

        // let data_length = header.data_length as usize;

        let mut rest = vec![];

        from.read_to_end(&mut rest);

        let (rest, checksum_bytes) = rest.split_at(rest.len() - 2);

        let checksum = u16::from_le_bytes([checksum_bytes[0], checksum_bytes[1]]);

        let mut checksum_data = vec![];
        from.rewind();
        from.read_to_end(&mut checksum_data);
        if crc::valid(&checksum_data[..(checksum_data.len() - 2)], checksum) {
            Ok(Fit {
                protocol_version: header.protocol_version,
                profile_version: header.profile_version,
                records: rest.to_vec(),
            })
        } else {
            Err(Error::FileNotValid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_can_be_loaded() {
        let path: PathBuf = PathBuf::from("../data/Activity.fit");
        let activity = Fit::from_file(path).unwrap();

        assert_eq!(activity.protocol_version, 32);
        assert_eq!(activity.profile_version, 2147);
    }

    #[test]
    fn stores_records() {
        let path: PathBuf = PathBuf::from("../data/Activity.fit");
        let activity = Fit::from_file(path).unwrap();

        assert_eq!(activity.records.len(), 94080);
    }

    #[test]
    fn bytes_can_be_loaded() {
        let mut file = File::open("../data/Activity.fit").unwrap();
        let activity = Fit::from_bytes(&mut file).unwrap();

        assert_eq!(activity.protocol_version, 32);
    }

    #[test]
    fn missing_file_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_missing.fit");
        let activity = Fit::from_file(path);
        let expected = Err(Error::FileNotFound);

        assert_eq!(activity, expected);
    }

    #[test]
    fn invalid_fit_file_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_invalid_header.fit");
        let activity = Fit::from_file(path);
        let expected = Err(Error::FileNotValid);

        assert_eq!(activity, expected);
    }

    #[test]
    fn invalid_fit_data_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_invalid_data.fit");
        let activity = Fit::from_file(path);
        let expected = Err(Error::FileNotValid);

        assert_eq!(activity, expected);
    }
}
