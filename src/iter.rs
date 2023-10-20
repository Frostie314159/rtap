use bin_utils::ReadCtx;
use macro_bits::{bit, check_bit, incomplete_const_array};
use try_take::try_take;
#[cfg(not(debug_assertions))]
use no_panic::no_panic;

use crate::{error::RadiotapError, field_types::RadiotapField};

incomplete_const_array! {
    #[filler((1, 1))]
    const ALIGN_SIZE_TABLE: [(usize, usize); 31] = [
        0 => (8, 8),
        3 => (2, 4),
        4 => (2, 2),
        7 => (2, 2),
        8 => (2, 2),
        9 => (2, 2),
        14 => (2, 2),
        15 => (2, 2),
        18 => (4, 8),
        19 => (1, 3),
        20 => (4, 8),
        21 => (2, 12),
        22 => (8, 12),
        23 => (2, 6),
        24 => (2, 12),
        25 => (2, 6),
        27 => (2, 4),
        28 => (4, usize::MAX), // Variable length
        30 => (2, 6)
    ];
}

const fn calculate_padding(offset: usize, align: usize) -> usize {
    let aligned_offset = (offset + align - 1) & !(align - 1);
    aligned_offset - offset
}

#[cfg_attr(not(debug_assertions), no_panic)]
/// Create an iterator over the fields of a Radiotap header.
/// 
/// This takes in the entire header.
pub fn create_radiotap_iterator(
    data: &mut (impl ExactSizeIterator<Item = u8> + Clone),
) -> Result<impl Iterator<Item = RadiotapField> + '_, RadiotapError> {
    // Header parsing
    match data.next() {
        Some(0x00) => {}
        Some(_) => return Err(RadiotapError::VersionIsNotZero),
        None => return Err(RadiotapError::HeaderIncomplete),
    }
    data.advance_by(1)
        .map_err(|_| RadiotapError::HeaderIncomplete)?;

    let length = u16::from_le_bytes(
        data.next_chunk()
            .map_err(|_| RadiotapError::HeaderIncomplete)?,
    );

    let chunks = data.clone().array_chunks::<4>().map(u32::from_le_bytes);
    let presence_count = chunks
        .clone()
        .take_while(|x| check_bit!(bit!(31), x))
        .count()
        + 1;
    let mut payload_iter = data
        .clone()
        .skip(presence_count * core::mem::size_of::<u32>());
    data.advance_by(
        length
            .checked_sub(4)
            .ok_or(RadiotapError::SkipLenTooShort)? as usize,
    )
    .map_err(|_| RadiotapError::UnderlyingIterEndedEarly)?;
    let mut offset = presence_count * core::mem::size_of::<u32>() + 4;
    let mut is_next_ns_vendor = false;
    Ok(chunks
        .take(presence_count)
        .flat_map(move |x| {
            (0..31)
                .map(|bit| {
                    if check_bit!(x, bit!(bit)) {
                        if bit == 29 {
                            is_next_ns_vendor = false;
                            return None;
                        }
                        if bit == 30 {
                            is_next_ns_vendor = true;
                        }
                        if !is_next_ns_vendor {
                            Some(bit)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .next_chunk::<31>()
                .unwrap()
        })
        .flatten()
        .map_while(move |x| {
            let (align, size) = ALIGN_SIZE_TABLE[x];
            let padding = calculate_padding(offset, align); // The difference in the iterator offset, before and after alignment.

            let mut aligned_iter = payload_iter.by_ref().skip(padding); // Align the iterator.
            offset += padding + size; // Move the offset further along.
            RadiotapField::from_bytes(&mut try_take(&mut aligned_iter, size).ok()?, x as u8)
                .inspect(|field| {
                    if let RadiotapField::VendorNamespace { skip_length, .. } = field {
                        let _ = payload_iter.advance_by(*skip_length as usize);
                    }
                }).ok()
        }).fuse())
}
