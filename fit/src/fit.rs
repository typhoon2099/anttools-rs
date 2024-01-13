use crate::header::Header;
use std::io::Read;
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
            Ok(mut file) => {
                let mut raw = vec![];
                match file.read_to_end(&mut raw) {
                    Ok(_) => Fit::from_bytes(raw),
                    Err(_) => Err(Error::FileNotFound),
                }
            }
            Err(_) => Err(Error::FileNotFound),
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Fit, Error> {
        let header = match Header::from(&data) {
            Ok(header) => header,
            Err(_error) => return Err(Error::FileNotValid),
        };

        let header_length = header.header_length.into();
        let data_length = header.data_length as usize;

        let records = data
            [header_length..(header_length + data_length)]
            .to_vec();

        println!("{} {}", records.len(), header.data_length);

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
    use std::io::Read;
    use std::{fs::File, path::PathBuf};

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
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let activity = Fit::from_bytes(data).unwrap();

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
        let path: PathBuf = PathBuf::from("../data/Activity_invalid.fit");
        let activity = Fit::from_file(path);
        let expected = Err(Error::FileNotValid);

        assert_eq!(activity, expected);
    }
}
