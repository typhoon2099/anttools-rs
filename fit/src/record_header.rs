#[derive(Debug, PartialEq)]
pub enum MessageType {
    Definition,
    Data,
    CompressedTimestamp,
}

pub struct RecordHeader {
    pub message_type: MessageType,
    pub message_type_specific: bool,
    pub local_message_type: u8,
    pub time_offset: Option<u8>,
}

impl RecordHeader {
    pub fn from(from: &u8) -> RecordHeader {
        let compressed = 0b10000000 & from == 0b10000000;

        let message_type = match compressed {
            true => MessageType::CompressedTimestamp,
            false => {
                let definition = 0b01000000 & from == 0b01000000;

                match definition {
                    true => MessageType::Definition,
                    false => MessageType::Data,
                }
            }
        };
        RecordHeader {
            message_type,
            message_type_specific: false,
            local_message_type: 0,
            time_offset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_message() {
        let header = RecordHeader::from(&0b01000000);

        assert_eq!(header.message_type, MessageType::Definition);
    }

    #[test]
    fn data_message() {
        let header = RecordHeader::from(&0);

        assert_eq!(header.message_type, MessageType::Data);
    }

    #[test]
    fn compressed_timestamp_message() {
        let header = RecordHeader::from(&0b10000000);

        assert_eq!(header.message_type, MessageType::CompressedTimestamp);
    }
}
