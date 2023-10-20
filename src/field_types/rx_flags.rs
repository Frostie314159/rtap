use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct RxFlags: u16 {
        pub bad_plcp: bool => bit!(2)
    }
}
