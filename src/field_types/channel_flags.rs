use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Debug, PartialEq, Default, Clone, Copy)]
    pub struct ChannelFlags: u16 {
        pub turbo_channel: bool => bit!(4),
        pub cck_channel: bool => bit!(5),
        pub ofdm_channel: bool => bit!(6),
        pub two_ghz_channel: bool => bit!(7),
        pub five_ghz_channel: bool => bit!(8),
        pub passive_scan_only: bool => bit!(9),
        pub dyn_cck_ofdm_channel: bool => bit!(10),
        pub gfsk_channel: bool => bit!(11)
    }
}
