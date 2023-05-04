use std::io::Read;
use std::{fs::File, path::PathBuf};

pub struct Fit {
    raw: Vec<u8>,
}

impl Fit {
    pub fn from_file(path: PathBuf) -> Option<Fit> {
        let mut file = File::open(path).unwrap();
        let mut raw = vec![];
        file.read_to_end(&mut raw).ok()?;
        Some(Fit { raw })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::{fs::File, path::PathBuf};

    #[test]
    fn raw() {
        let path: PathBuf = PathBuf::from("../data/Activity.fit");
        let mut file = File::open("../data/Activity.fit").unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let activity = Fit::from_file(path).unwrap();

        assert_eq!(*activity.raw, data);
    }
}
