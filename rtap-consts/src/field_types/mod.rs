use core::fmt::Debug;
use scroll::{
    ctx::{TryFromCtx, TryIntoCtx},
    Pread, Pwrite, Endian,
};

use self::{
    channel_flags::ChannelFlags, flags::RadiotapFlags, rx_flags::RxFlags, tx_flags::TxFlags,
};
use crate::*;

pub mod channel_flags;
pub mod flags;
pub mod rx_flags;
pub mod tx_flags;
pub mod vendor_namespace;
pub mod xchannel_flags;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    ExtendedChannel {},
    VendorNamespace {
        oui: [u8; 3],
        sub_ns: u8,
        skip_length: u16,
    },
}
impl RadiotapField {
    pub const fn to_bit(&self) -> usize {
        match self {
            Self::Antenna { .. } => ANTENNA,
            Self::AntennaNoise { .. } => DBM_ANTNOISE,
            Self::AntennaSignal { .. } => DBM_ANTSIGNAL,
            Self::Channel { .. } => CHANNEL,
            Self::DBAntennaNoise { .. } => DB_ANTNOISE,
            Self::DBAntennaSignal { .. } => DBM_ANTSIGNAL,
            Self::DBTxAttenuation { .. } => DB_TX_ATTENUATION,
            Self::DataRetries { .. } => DATA_RETRIES,
            Self::ExtendedChannel {} => XCHANNEL,
            Self::FHSS { .. } => FHSS,
            Self::Flags { .. } => FLAGS,
            Self::LockQuality { .. } => LOCK_QUALITY,
            Self::Rate { .. } => RATE,
            Self::RtsRetries { .. } => RTS_RETRIES,
            Self::RxFlags { .. } => RX_FLAGS,
            Self::TSFT { .. } => TSFT,
            Self::TxAttenuation { .. } => TX_ATTENUATION,
            Self::TxFlags { .. } => TX_FLAGS,
            Self::TxPower { .. } => DBM_TX_POWER,
            Self::VendorNamespace { .. } => VENDOR_NS_NEXT,
        }
    }
}
impl<'a> TryFromCtx<'a, u8> for RadiotapField {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], ctx: u8) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        Ok(match ctx as usize {
            TSFT => (
                Self::TSFT {
                    mac_time: from.gread(&mut offset)?,
                },
                8,
            ),
            FLAGS => (
                Self::Flags {
                    flags: RadiotapFlags::from_bits(from.gread(&mut offset)?),
                },
                1,
            ),
            RATE => (
                Self::Rate {
                    rate: from.gread(&mut offset)?,
                },
                1,
            ),
            CHANNEL => (
                Self::Channel {
                    frequency: from.gread(&mut offset)?,
                    flags: ChannelFlags::from_bits(from.gread(&mut offset)?),
                },
                4,
            ),
            FHSS => (
                Self::FHSS {
                    hop_set: from.gread(&mut offset)?,
                    hop_pattern: from.gread(&mut offset)?,
                },
                2,
            ),
            DBM_ANTSIGNAL => (
                Self::AntennaSignal {
                    signal: from.gread(&mut offset)?,
                },
                1,
            ),
            DBM_ANTNOISE => (
                Self::AntennaNoise {
                    noise: from.gread(&mut offset)?,
                },
                1,
            ),
            LOCK_QUALITY => (
                Self::LockQuality {
                    quality: from.gread(&mut offset)?,
                },
                2,
            ),
            TX_ATTENUATION => (
                Self::TxAttenuation {
                    attenuation: from.gread(&mut offset)?,
                },
                2,
            ),
            DB_TX_ATTENUATION => (
                Self::DBTxAttenuation {
                    attenuation: from.gread(&mut offset)?,
                },
                2,
            ),
            DBM_TX_POWER => (
                Self::TxPower {
                    power: from.gread(&mut offset)?,
                },
                1,
            ),
            ANTENNA => (
                Self::Antenna {
                    index: from.gread(&mut offset)?,
                },
                1,
            ),
            DB_ANTSIGNAL => (
                Self::DBAntennaSignal {
                    signal: from.gread(&mut offset)?,
                },
                1,
            ),
            RX_FLAGS => (
                Self::RxFlags {
                    flags: RxFlags::from_bits(from.gread(&mut offset)?),
                },
                2,
            ),
            TX_FLAGS => (
                Self::TxFlags {
                    flags: TxFlags::from_bits(from.gread(&mut offset)?),
                },
                2,
            ),
            RTS_RETRIES => (
                Self::RtsRetries {
                    retries: from.gread(&mut offset)?,
                },
                1,
            ),
            DATA_RETRIES => (
                Self::DataRetries {
                    retries: from.gread(&mut offset)?,
                },
                1,
            ),

            VENDOR_NS_NEXT => (
                {
                    Self::VendorNamespace {
                        oui: {
                            let mut tmp = [0x00; 3];
                            from.gread_inout(&mut offset, &mut tmp)?;
                            tmp
                        },
                        sub_ns: from.gread(&mut offset)?,
                        skip_length: from.gread(&mut offset)?,
                    }
                },
                6,
            ),
            _ => {
                return Err(scroll::Error::BadInput {
                    size: from.len(),
                    msg: "Unknown field type",
                })
            }
        })
    }
}
impl TryIntoCtx for RadiotapField {
    type Error = scroll::Error;
    fn try_into_ctx(self, buffer: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        match self {
            Self::TSFT { mac_time } => buffer.pwrite(mac_time, 0),
            Self::Flags { flags } => buffer.pwrite(flags.into_bits(), 0),
            Self::Rate { rate } => buffer.pwrite(rate, 0),
            Self::Channel { frequency, flags } => {
                buffer.pwrite_with(frequency, 0, Endian::Little)?;
                buffer.pwrite_with(flags.into_bits(), 2, Endian::Little)?;
                Ok(4)
            }
            _ => todo!(),
        }
    }
}
