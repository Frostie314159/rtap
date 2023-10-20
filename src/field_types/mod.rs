use alloc::{vec, vec::Vec};
use bin_utils::{ParserError, ReadCtx, Write};
use core::fmt::{Debug, Formatter};
use try_take::try_take;

use self::{
    channel_flags::ChannelFlags, flags::RadiotapFlags, rx_flags::RxFlags, tx_flags::TxFlags,
};

pub mod channel_flags;
pub mod flags;
pub mod rx_flags;
pub mod tx_flags;
pub(crate) mod vendor_namespace;

#[derive(PartialEq, Clone, Copy)]
pub enum RadiotapField {
    TSFT {
        /// Unit: us
        mac_time: u64,
    },
    Flags {
        flags: RadiotapFlags,
    },
    Rate {
        /// Unit: kbps
        rate: u8,
    },
    Channel {
        /// Unit: MHz
        frequency: u16,
        flags: ChannelFlags,
    },
    /// FHSS parameters
    ///
    /// NOTE: If you ever see this in real-life, please let met know.
    /// The last time, this was standardized, was in the original IEEE 802.11 standard.
    FHSS {
        hop_set: u8,
        hop_pattern: u8,
    },
    AntennaSignal {
        /// Unit: dBm
        signal: i8,
    },
    AntennaNoise {
        /// Unit: dBm
        noise: i8,
    },
    LockQuality {
        /// Unitless
        quality: u16,
    },
    TxAttenuation {
        /// Unitless
        attenuation: u16,
    },
    DBTxAttenuation {
        /// dB
        attenuation: u16,
    },
    TxPower {
        /// Unit: dBm
        power: i8,
    },
    Antenna {
        index: u8,
    },
    DBAntennaSignal {
        /// Unit: dB
        signal: u8,
    },
    DBAntennaNoise {
        /// Unit: dB
        noise: u8,
    },
    RxFlags {
        flags: RxFlags,
    },
    TxFlags {
        flags: TxFlags,
    },
    RtsRetries {
        retries: u8,
    },
    DataRetries {
        retries: u8,
    },
    VendorNamespace {
        oui: [u8; 3],
        sub_ns: u8,
        skip_length: u16,
    },
}
impl Debug for RadiotapField {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            RadiotapField::TSFT { mac_time } => f.write_fmt(format_args!("mac_time: {mac_time}")),
            RadiotapField::Flags { flags } => flags.fmt(f),
            RadiotapField::Rate { rate } => {
                f.write_fmt(format_args!("rate: {}.{} Mbps", rate / 2, (rate & 1) * 5))
            }
            RadiotapField::Channel { frequency, flags } => f
                .debug_struct("Channel")
                .field("frequency", &format_args!("{frequency} MHz").as_str())
                .field("flags", &flags)
                .finish(),
            RadiotapField::FHSS {
                hop_set,
                hop_pattern,
            } => f
                .debug_struct("FHSS")
                .field("hop_set", &hop_set)
                .field("hop_patter", &hop_pattern)
                .finish(),
            RadiotapField::AntennaSignal { signal } => {
                f.write_fmt(format_args!("antenna_signal: {signal} dBm"))
            }
            RadiotapField::AntennaNoise { noise } => {
                f.write_fmt(format_args!("antenna_noise: {noise} dBm"))
            }
            RadiotapField::LockQuality { quality } => {
                f.write_fmt(format_args!("lock_quality: {quality}"))
            }
            RadiotapField::TxAttenuation { attenuation } => {
                f.write_fmt(format_args!("tx_attenuation: {attenuation}"))
            }
            RadiotapField::DBTxAttenuation { attenuation } => {
                f.write_fmt(format_args!("db_tx_attenuation: {attenuation} dB"))
            }
            RadiotapField::TxPower { power } => f.write_fmt(format_args!("tx_power: {power} dBm")),
            RadiotapField::Antenna { index } => f.write_fmt(format_args!("antenna_index: {index}")),
            RadiotapField::DBAntennaSignal { signal } => {
                f.write_fmt(format_args!("antenna_signal: {signal} dB"))
            }
            RadiotapField::DBAntennaNoise { noise } => {
                f.write_fmt(format_args!("antenna_noise: {noise} dB"))
            }
            RadiotapField::RxFlags { flags } => flags.fmt(f),
            RadiotapField::TxFlags { flags } => flags.fmt(f),
            RadiotapField::RtsRetries { retries } => {
                f.write_fmt(format_args!("rts_retries: {retries}"))
            }
            RadiotapField::DataRetries { retries } => {
                f.write_fmt(format_args!("data_retries: {retries}"))
            }
            RadiotapField::VendorNamespace {
                oui,
                sub_ns,
                skip_length,
            } => f
                .debug_struct("VendorNamespace")
                .field("oui", oui)
                .field("sub_ns", sub_ns)
                .field("skip_length", skip_length)
                .finish(),
        }
    }
}
impl ReadCtx<u8> for RadiotapField {
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        field_type: u8,
    ) -> Result<Self, ParserError> {
        Ok(match field_type {
            0 => Self::TSFT {
                mac_time: u64::from_le_bytes(data.next_chunk().unwrap()),
            },
            1 => Self::Flags {
                flags: RadiotapFlags::from_representation(data.next().unwrap()),
            },
            2 => Self::Rate {
                rate: data.next().unwrap(),
            },
            3 => Self::Channel {
                frequency: u16::from_le_bytes(data.next_chunk().unwrap()),
                flags: ChannelFlags::from_representation(u16::from_le_bytes(
                    data.next_chunk().unwrap(),
                )),
            },
            4 => Self::FHSS {
                hop_set: data.next().unwrap(),
                hop_pattern: data.next().unwrap(),
            },
            5 => Self::AntennaSignal {
                signal: data.next().unwrap() as i8,
            },
            6 => Self::AntennaNoise {
                noise: data.next().unwrap() as i8,
            },
            7 => Self::LockQuality {
                quality: u16::from_le_bytes(data.next_chunk().unwrap()),
            },
            8 => Self::TxAttenuation {
                attenuation: u16::from_le_bytes(data.next_chunk().unwrap()),
            },
            9 => Self::DBTxAttenuation {
                attenuation: u16::from_le_bytes(data.next_chunk().unwrap()),
            },
            10 => Self::TxPower {
                power: data.next().unwrap() as i8,
            },
            11 => Self::Antenna {
                index: data.next().unwrap(),
            },
            12 => Self::DBAntennaSignal {
                signal: data.next().unwrap(),
            },
            14 => Self::RxFlags {
                flags: RxFlags::from_representation(u16::from_le_bytes(data.next_chunk().unwrap())),
            },
            15 => Self::TxFlags {
                flags: TxFlags::from_representation(u16::from_le_bytes(data.next_chunk().unwrap())),
            },
            16 => Self::RtsRetries {
                retries: data.next().unwrap(),
            },
            17 => Self::DataRetries {
                retries: data.next().unwrap(),
            },
            30 => {
                let data = &mut try_take(data, 6).map_err(ParserError::TooLittleData)?;
                Self::VendorNamespace {
                    oui: data.next_chunk().unwrap(),
                    sub_ns: data.next().unwrap(),
                    skip_length: u16::from_le_bytes(data.next_chunk().unwrap()),
                }
            }
            _ => return Err(ParserError::ValueNotUnderstood),
        })
    }
}
impl Write for RadiotapField {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            RadiotapField::TSFT { mac_time } => mac_time.to_le_bytes().to_vec(),
            RadiotapField::Flags { flags } => {
                vec![flags.to_representation()]
            }
            RadiotapField::Rate { rate } => {
                vec![*rate]
            }
            RadiotapField::Channel { frequency, flags } => frequency
                .to_le_bytes()
                .into_iter()
                .chain(flags.to_representation().to_le_bytes())
                .collect(),
            RadiotapField::FHSS {
                hop_set,
                hop_pattern,
            } => {
                vec![*hop_set, *hop_pattern]
            }
            RadiotapField::AntennaSignal { signal } => {
                vec![*signal as u8]
            }
            RadiotapField::AntennaNoise { noise } => {
                vec![*noise as u8]
            }
            RadiotapField::LockQuality { quality } => quality.to_le_bytes().to_vec(),
            RadiotapField::TxAttenuation { attenuation } => attenuation.to_le_bytes().to_vec(),
            RadiotapField::DBTxAttenuation { attenuation } => attenuation.to_le_bytes().to_vec(),
            RadiotapField::TxPower { power } => {
                vec![*power as u8]
            }
            RadiotapField::Antenna { index } => {
                vec![*index]
            }
            RadiotapField::DBAntennaSignal { signal } => {
                vec![*signal]
            }
            RadiotapField::DBAntennaNoise { noise } => {
                vec![*noise]
            }
            RadiotapField::RxFlags { flags } => flags.to_representation().to_le_bytes().to_vec(),
            RadiotapField::TxFlags { flags } => flags.to_representation().to_le_bytes().to_vec(),
            RadiotapField::RtsRetries { retries } => {
                vec![*retries]
            }
            RadiotapField::DataRetries { retries } => {
                vec![*retries]
            }
            RadiotapField::VendorNamespace {
                oui,
                sub_ns,
                skip_length,
            } => oui
                .iter()
                .chain(core::iter::once(sub_ns))
                .copied()
                .chain(skip_length.to_le_bytes())
                .collect(),
        }
    }
}
