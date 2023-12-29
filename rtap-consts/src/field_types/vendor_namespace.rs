#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct VendorNamespace {
    pub oui: [u8; 3],
    pub sub_ns: u8,
    pub skip_length: u16,
}
