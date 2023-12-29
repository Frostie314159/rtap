use crate::field_types::ChannelFlags;
use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub struct ExtendedChannelFlags: u32 {
        pub flags: ChannelFlags => bit!(0,1,2,3,4,5,6,7,8,9,10,11),
        pub gsm_channel: bool => bit!(12),
        pub status_turbo_channel: bool => bit!(13),
        pub half_rate_channel: bool => bit!(14),
        pub quarter_rate_channel: bool => bit!(15),
        pub ht_twenty_channel: bool => bit!(16),
        pub ht_fourty_plus_channel: bool => bit!(17),
        pub ht_fourty_minus_channel: bool => bit!(18)
    }
}
