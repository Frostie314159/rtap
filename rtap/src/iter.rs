use macro_bits::{bit, check_bit};
#[cfg(not(debug_assertions))]
use no_panic::no_panic;
use rtap_consts::ALIGN_SIZE_TABLE;
use scroll::Pread;

use crate::field_types::RadiotapField;

const fn calculate_padding(offset: usize, align: usize) -> usize {
    let aligned_offset = (offset + align - 1) & !(align - 1);
    aligned_offset - offset
}

#[cfg_attr(not(debug_assertions), no_panic)]
pub(crate) fn create_radiotap_iterator<'a>(
    data: &'a [u8],
) -> impl Iterator<Item = RadiotapField> + '_ {
    let chunks = data.array_chunks().copied().map(u32::from_le_bytes);
    let presence_count = chunks
        .clone()
        .take_while(|x| check_bit!(bit!(31), x))
        .count()
        + 1;
    let mut offset = presence_count * core::mem::size_of::<u32>();
    let mut is_next_ns_vendor = false;
    chunks
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
            offset += padding + size; // Move the offset further along.
            data.get((offset - size)..offset)?
                .pread_with::<RadiotapField>(0, x as u8)
                .inspect(|field| {
                    if let RadiotapField::VendorNamespace { skip_length, .. } = field {
                        offset += *skip_length as usize;
                    }
                })
                .ok()
        })
}
/* pub fn order_fields<I: IntoIterator<Item = RadiotapField>>(iter: I) -> Vec<RadiotapField> {
    
} */
