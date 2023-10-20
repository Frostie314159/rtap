#![feature(
    iter_next_chunk,
    iter_array_chunks,
    iter_advance_by,
    result_option_inspect
)]

extern crate alloc;

pub mod error;
pub mod field_types;
mod iter;
pub use iter::create_radiotap_iterator;
#[doc(hidden)]
pub use bin_utils;
