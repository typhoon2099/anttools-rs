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
        let compressed = 0b1000_0000 & from == 0b1000_0000;

        let message_type = match compressed {
            true => MessageType::CompressedTimestamp,
            false => {
                let definition = 0b0100_0000 & from == 0b0100_0000;

                match definition {
                    true => MessageType::Definition,
                    false => MessageType::Data,
                }
            }
        };

        let message_type_specific = 0b0010_0000 & from == 0b0010_0000;
        let local_message_type = 0b0000_1111 & from;

        RecordHeader {
            message_type,
            message_type_specific,
            local_message_type,
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
    fn message_type_specific() {
        let header = RecordHeader::from(&0b00100000);

        assert!(header.message_type_specific);
    }

    #[test]
    fn local_message_type() {
        let header = RecordHeader::from(&0b01001111);

        assert_eq!(header.local_message_type, 15);
    }

    #[test]
    fn compressed_timestamp_message() {
        let header = RecordHeader::from(&0b10000000);

        assert_eq!(header.message_type, MessageType::CompressedTimestamp);
    }
}
