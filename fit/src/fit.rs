use crate::crc;
use crate::header::Header;
use crate::record::Record;
use crate::record_header::{MessageType, RecordHeader};
use std::io::{BufReader, Read, Seek};
use std::{fs::File, path::PathBuf};

// TODO: Make records a new Struct
#[derive(Debug, PartialEq)]
pub struct Fit {
    pub protocol_version: u8,
    pub profile_version: u16,
    pub records: Vec<Record>,
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
            Ok(file) => {
                let mut buf = BufReader::new(file);

                Fit::read(&mut buf)
            }
            Err(_) => Err(Error::FileNotFound),
        }
    }

    fn read(from: &mut BufReader<File>) -> Result<Fit, Error> {
        let mut checksum_data = vec![];
        from.read_to_end(&mut checksum_data).unwrap();
        from.rewind().unwrap();

        let header = match Header::from(from) {
            Ok(header) => header,
            Err(_error) => return Err(Error::FileNotValid),
        };

        let (checksum_data, checksum_bytes) = checksum_data.split_at(checksum_data.len() - 2);
        let checksum = u16::from_le_bytes([checksum_bytes[0], checksum_bytes[1]]);
        if crc::valid(checksum_data, checksum) {
        } else {
            return Err(Error::FileNotValid);
        }

        let mut records = vec![];

        let mut record_header_byte = vec![0u8; 1];
        let _ = from.read_exact(&mut record_header_byte);
        let next_record_header = RecordHeader::from(record_header_byte[0]);

        match next_record_header.message_type {
            MessageType::Definition => {
                records.push(Record::definition(from));
            }
            MessageType::Data => {
                println!("Data");
                println!("{}", from.stream_position().unwrap());
            }
            MessageType::CompressedTimestamp => {
                println!("CompressedTimestamp");
                println!("{}", from.stream_position().unwrap());
            }
        }

        Ok(Fit {
            protocol_version: header.protocol_version,
            profile_version: header.profile_version,
            records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
