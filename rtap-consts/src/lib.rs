use macro_bits::{gen_consts, incomplete_const_array};

pub mod field_types;

gen_consts! {
    const_type: usize,

    TSFT => 0,
    FLAGS => 1,
    RATE => 2,
    CHANNEL => 3,
    FHSS => 4,
    DBM_ANTSIGNAL => 5,
    DBM_ANTNOISE => 6,
    LOCK_QUALITY => 7,
    TX_ATTENUATION => 8,
    DB_TX_ATTENUATION => 9,
    DBM_TX_POWER => 10,
    ANTENNA => 11,
    DB_ANTSIGNAL => 12,
    DB_ANTNOISE => 13,
    RX_FLAGS => 14,
    TX_FLAGS => 15,
    RTS_RETRIES => 16,
    DATA_RETRIES => 17,
    XCHANNEL => 18,
    MCS => 19,
    AMPDU_STATUS => 20,
    VHT => 21,
    TIMESTAMP => 22,
    HE => 23,
    HE_MU => 24,
    ZERO_LEN_PSDU => 26,
    LSIG => 27,
    TLV => 28,
    RADIOTAP_NS_NEXT => 29,
    VENDOR_NS_NEXT => 30,
    EXT_PRESENCE_BITMAP => 31,
    EHT_USIG => 33,
    EHT => 34
}
incomplete_const_array! {
    #[filler((1, 1))]
    pub const ALIGN_SIZE_TABLE: [(usize, usize); 31] = [
        TSFT => (8, 8),
        CHANNEL => (2, 4),
        FHSS => (2, 2),
        LOCK_QUALITY => (2, 2),
        TX_ATTENUATION => (2, 2),
        DB_TX_ATTENUATION => (2, 2),
        RX_FLAGS => (2, 2),
        TX_FLAGS => (2, 2),
        XCHANNEL => (4, 8),
        MCS => (1, 3),
        AMPDU_STATUS => (4, 8),
        VHT => (2, 12),
        TIMESTAMP => (8, 12),
        HE => (2, 6),
        HE_MU => (2, 12),
        ZERO_LEN_PSDU => (2, 6),
        LSIG => (2, 4),
        TLV => (4, usize::MAX), // Variable length
        VENDOR_NS_NEXT => (2, 6)
    ];
}
