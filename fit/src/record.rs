use std::io::{Read, Seek};



#[derive(Debug, PartialEq)]
pub struct Record {
    pub data: Vec<u8>,
}

// TODO: Record::Message
// Defined as Record Header byte and record content, which can be a Definition Message or a Data
// Message
impl Record {
    pub fn definition(from: &mut (impl Read + Seek)) -> Record {
        let mut reserved: [u8; 1] = [0; 1];
        from.read_exact(&mut reserved).unwrap();

        let mut architecture: [u8; 1] = [0; 1];
        from.read_exact(&mut architecture).unwrap();

        let mut global_message_number: [u8; 2] = [0; 2];
        from.read_exact(&mut global_message_number).unwrap();

        let mut number_of_fields: [u8; 1] = [0; 1];
        from.read_exact(&mut number_of_fields).unwrap();

        let mut fields: Vec<u8> = vec![0; number_of_fields[0] as usize * 3];
        from.read_exact(&mut fields).unwrap();

        let mut number_of_developer_fields: [u8; 1] = [0; 1];
        from.read_exact(&mut number_of_developer_fields).unwrap();

        let mut developer_fields: Vec<u8> = vec![0; number_of_developer_fields[0] as usize * 3];
        from.read_exact(&mut developer_fields).unwrap();

        let data = [
            &reserved[..],
            &architecture[..],
            &global_message_number[..],
            &number_of_fields[..],
            &fields[..],
            &number_of_developer_fields[..],
            &developer_fields[..],
        ]
        .concat();

        Record { data }
    }

    // TODO: pub fn data(definition) -> some data {}
}
