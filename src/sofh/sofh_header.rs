
/// Minimal implementation of the
/// [Simple Open Framing Header specification](https://github.com/FIXTradingCommunity/fix-simple-open-framing-header/blob/master/v1-1-RC1/sofh.md)
/// for this example project.
///
/// **NOTE**: This implementation does not include error checking for
///     reasonable lengths or valid encoding types.
#[derive(Copy, Clone)]
pub struct SofhHeader {
    pub message_length: u32,
    pub encoding_type: u16,
}

impl From<&[u8; 6]> for SofhHeader {
    fn from(header_bytes: &[u8; 6]) -> Self {
        let mut len: [u8; 4] = Default::default();
        len.copy_from_slice(&header_bytes[0..4]);

        let mut encoding: [u8; 2] = Default::default();
        encoding.copy_from_slice(&header_bytes[4..6]);

        if cfg!(feature = "sofh_big_endian") {
            SofhHeader {
                message_length: u32::from_be_bytes(len),
                encoding_type: u16::from_be_bytes(encoding),
            }
        } else {
            SofhHeader {
                message_length: u32::from_le_bytes(len),
                encoding_type: u16::from_le_bytes(encoding),
            }
        }
    }
}

impl Into<[u8; 6]> for SofhHeader {
    fn into(self) -> [u8; 6] {
        let mut buf: [u8; 6] = Default::default();

        if cfg!(feature = "sofh_big_endian") {
            buf[..4].copy_from_slice(&self.message_length.to_be_bytes());
            buf[4..6].copy_from_slice(&self.encoding_type.to_be_bytes());
        } else {
            buf[..4].copy_from_slice(&self.message_length.to_le_bytes());
            buf[4..6].copy_from_slice(&self.encoding_type.to_le_bytes());
        }

        buf
    }
}


#[cfg(test)]
mod tests {
    use crate::sofh::SofhHeader;

    #[test]
    fn sofh_encode_decode() {
        let header = SofhHeader {
            message_length: 1,
            encoding_type: 2,
        };
        let encoded: [u8; 6] = header.into();
        let decoded: SofhHeader = SofhHeader::from(&encoded);
        assert_eq!(header.message_length, decoded.message_length);
        assert_eq!(header.encoding_type, decoded.encoding_type);
    }

}
