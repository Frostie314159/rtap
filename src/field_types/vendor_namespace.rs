use bin_utils::{ParserError, ReadFixed};

#[derive(Debug, Clone, Copy)]
pub struct VendorNamespace {
    pub oui: [u8; 3],
    pub sub_ns: u8,
    pub skip_length: u16,
}
impl ReadFixed<6> for VendorNamespace {
    fn from_bytes(data: &[u8; 6]) -> Result<Self, ParserError> {
        let mut oui = [0; 3];
        oui.copy_from_slice(&data[0..3]);
        let sub_ns = data[3];
        let skip_length = u16::from_le_bytes(data[4..6].try_into().unwrap());
        Ok(Self {
            oui,
            sub_ns,
            skip_length,
        })
    }
}
