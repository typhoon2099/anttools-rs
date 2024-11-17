use crate::crc;
use crate::header::Header;
use std::io::{BufReader, Read, Seek};
use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Fit<'a> {
    header: Header,
    record_data: Option<&'a mut BufReader<File>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    FileNotFound,
    FileNotValid,
}

impl<'a> Fit<'a> {
    pub fn from_file(path: PathBuf) -> Result<Fit<'static>, Error> {
        let file = File::open(path);
        match file {
            Ok(file) => {
                let buf = BufReader::new(file);

                Fit::read(buf)
            }
            Err(_) => Err(Error::FileNotFound),
        }
    }

    fn read(mut from: BufReader<File>) -> Result<Fit<'a>, Error> {
        let mut checksum_data = vec![];
        from.read_to_end(&mut checksum_data).unwrap();
        from.rewind().unwrap();

        let header = match Header::from(&mut from) {
            Ok(header) => header,
            Err(_error) => return Err(Error::FileNotValid),
        };

        let (checksum_data, checksum_bytes) = checksum_data.split_at(checksum_data.len() - 2);
        let checksum = u16::from_le_bytes([checksum_bytes[0], checksum_bytes[1]]);
        if crc::valid(checksum_data, checksum) {
        } else {
            return Err(Error::FileNotValid);
        }

        Ok(Self {
            header,
            record_data: None,
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
        let activity = Fit::from_file(path);

        assert!(activity.is_ok());
    }

    #[test]
    fn missing_file_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_missing.fit");
        let activity = Fit::from_file(path);
        let expected = Error::FileNotFound;

        assert_eq!(activity.unwrap_err(), expected);
    }

    #[test]
    fn invalid_fit_file_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_invalid_header.fit");
        let activity = Fit::from_file(path);
        let expected = Error::FileNotValid;

        assert_eq!(activity.unwrap_err(), expected);
    }

    #[test]
    fn invalid_fit_data_returns_error() {
        let path: PathBuf = PathBuf::from("../data/Activity_invalid_data.fit");
        let activity = Fit::from_file(path);
        let expected = Error::FileNotValid;

        assert_eq!(activity.unwrap_err(), expected);
    }
}
