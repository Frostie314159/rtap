use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub struct RxFlags: u16 {
        pub bad_plcp: bool => bit!(2)
    }
}
