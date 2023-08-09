use crate::header::Header;
use std::io::Read;
use std::{fs::File, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Fit {
    raw: Vec<u8>,
    header: Header,
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
                    Ok(_) => {
                        let header = match Header::from(&raw) {
                            Ok(header) => header,
                            Err(_error) => return Err(Error::FileNotValid),
                        };
                        Ok(Fit { raw, header })
                    }
                    Err(_) => Err(Error::FileNotFound),
                }
            }
            Err(_) => Err(Error::FileNotFound),
        }
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
        let mut file = File::open("../data/Activity.fit").unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let activity = Fit::from_file(path).unwrap();

        assert_eq!(*activity.raw, data);
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
