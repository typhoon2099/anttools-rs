#[derive(Debug, PartialEq)]
pub enum MessageType {
    Definition,
    Data,
    CompressedTimestamp,
}

// TODO: Record::Header
#[derive(Debug, PartialEq)]
pub struct RecordHeader {
    pub message_type: MessageType,
    pub message_type_specific: bool,
    pub local_message_type: u8,
    pub time_offset: Option<u8>,
}

impl RecordHeader {
    pub fn from(from: u8) -> RecordHeader {
        let message_type = Self::message_type(&from);

        RecordHeader {
            message_type,
            message_type_specific: Self::message_type_specific(&from),
            local_message_type: Self::local_message_type(&from),
            time_offset: Self::time_offset(&from),
        }
    }

    const COMPRESSED: u8 = 0b1000_0000;
    fn compressed(from: &u8) -> bool {
        Self::COMPRESSED & from == Self::COMPRESSED
    }

    fn message_type(from: &u8) -> MessageType {
        match Self::compressed(from) {
            true => MessageType::CompressedTimestamp,
            false => match Self::definition(from) {
                true => MessageType::Definition,
                false => MessageType::Data,
            },
        }
    }

    const NORMAL_MESSAGE_TYPE: u8 = 0b0100_0000;
    fn definition(from: &u8) -> bool {
        Self::NORMAL_MESSAGE_TYPE & from == Self::NORMAL_MESSAGE_TYPE
    }

    const MESSAGE_TYPE_SPECIFIC: u8 = 0b0010_0000;
    fn message_type_specific(from: &u8) -> bool {
        if Self::compressed(from) {
            false
        } else {
            Self::MESSAGE_TYPE_SPECIFIC & from == Self::MESSAGE_TYPE_SPECIFIC
        }
    }

    const COMPRESSED_LOCAL_MESSAGE_TYPE: u8 = 0b0110_0000;
    const NORMAL_LOCAL_MESSAGE_TYPE: u8 = 0b0000_1111;
    fn local_message_type(from: &u8) -> u8 {
        if Self::compressed(from) {
            (Self::COMPRESSED_LOCAL_MESSAGE_TYPE & from) >> 5
        } else {
            Self::NORMAL_LOCAL_MESSAGE_TYPE & from
        }
    }

    const TIME_OFFSET: u8 = 0b0001_1111;
    fn time_offset(from: &u8) -> Option<u8> {
        if Self::compressed(from) {
            Some(Self::TIME_OFFSET & from)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_message() {
        let header = RecordHeader::from(0b01000000);

        assert_eq!(header.message_type, MessageType::Definition);
    }

    #[test]
    fn data_message() {
        let header = RecordHeader::from(0);

        assert_eq!(header.message_type, MessageType::Data);
    }

    #[test]
    fn message_type_specific() {
        let header = RecordHeader::from(0b0010_0000);

        assert!(header.message_type_specific);
    }

    #[test]
    fn local_message_type() {
        let header = RecordHeader::from(0b0100_1111);

        assert_eq!(header.local_message_type, 15);
    }

    #[test]
    fn time_offset() {
        let header = RecordHeader::from(0b0100_0000);

        assert_eq!(header.time_offset, None);
    }

    #[test]
    fn compressed_timestamp_message() {
        let header = RecordHeader::from(0b1000_0000);

        assert_eq!(header.message_type, MessageType::CompressedTimestamp);
    }

    #[test]
    fn compressed_local_message_type() {
        let header = RecordHeader::from(0b1110_0000);

        assert_eq!(header.local_message_type, 3);
    }

    #[test]
    fn compressed_timeoffset() {
        let header = RecordHeader::from(0b1001_1111);

        assert_eq!(header.time_offset, Some(31));
    }
}
