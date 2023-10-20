use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct TxFlags: u16 {
        pub failed_excessive_retries: bool => bit!(0),
        pub cts_to_self_protection: bool => bit!(1),
        pub rts_cts_handshake: bool => bit!(2),
        pub no_ack: bool => bit!(3),
        pub pre_conf_seq_no: bool => bit!(4),
        pub dont_reorder: bool => bit!(5)
    }
}
