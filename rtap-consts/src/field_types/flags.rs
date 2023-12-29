use macro_bits::{bit, bitfield};

bitfield! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    /// Radiotap flags
    ///
    /// See the [radiotap spec](https://www.radiotap.org/fields/Flags.html)
    pub struct RadiotapFlags: u8 {
        pub during_cfp: bool => bit!(0),
        pub short_preamble: bool => bit!(1),
        pub wep: bool => bit!(2),
        pub fragmented: bool => bit!(3),
        pub includes_fcs: bool => bit!(4),
        pub has_padding: bool => bit!(5),
        pub bad_fcs: bool => bit!(6)
    }
}
