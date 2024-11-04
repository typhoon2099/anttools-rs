use std::io::{Read, Seek};



#[derive(Debug, PartialEq)]
pub struct Record {
    pub data: Vec<u8>,
}

pub enum Endianness {
    Big,
    Little,
}

pub struct Definition {
    pub endianness: Endianness,
    pub global_message_number: u16,
    pub fields: Vec<Field>,
    pub developer_fields: Vec<DeveloperField>,
}

pub struct Field {
    field_definition_number: u8,
    size: u8,
    base_type: u8,
}

pub struct DeveloperField {
    field_definition_number: u8,
    size: u8,
    base_type: u8,
}

// TODO: Record::Message
// Defined as Record Header byte and record content, which can be a Definition Message or a Data
// Message
impl Record {
    pub fn definition(from: &mut (impl Read + Seek)) -> Definition {
        let mut reserved: [u8; 1] = [0; 1];
        from.read_exact(&mut reserved).unwrap();

        let mut architecture: [u8; 1] = [0; 1];
        from.read_exact(&mut architecture).unwrap();
        let endianness = if architecture[0] == 1 {
            Endianness::Big
        } else {
            Endianness::Little
        };

        let mut global_message_number: [u8; 2] = [0; 2];
        from.read_exact(&mut global_message_number).unwrap();

        let global_message_number = match endianness {
            Endianness::Little => u16::from_le_bytes(global_message_number),
            Endianness::Big => u16::from_be_bytes(global_message_number),
        };

        let mut number_of_fields: [u8; 1] = [0; 1];
        from.read_exact(&mut number_of_fields).unwrap();

        let mut fields: Vec<u8> = vec![0; number_of_fields[0] as usize * 3];
        from.read_exact(&mut fields).unwrap();

        let mut number_of_developer_fields: [u8; 1] = [0; 1];
        from.read_exact(&mut number_of_developer_fields).unwrap();

        let mut developer_fields: Vec<u8> = vec![0; number_of_developer_fields[0] as usize * 3];
        from.read_exact(&mut developer_fields).unwrap();

        Definition {
            endianness,
            global_message_number,
            fields: vec![],
            developer_fields: vec![],
        }
    }

    pub fn data(definition: Definition) {}
}
