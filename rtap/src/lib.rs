#![allow(stable_features)]
#![feature(
    iter_next_chunk,
    iter_array_chunks,
    array_chunks,
    iter_advance_by,
    result_option_inspect
)]

extern crate alloc;

pub mod error;
pub use rtap_consts::field_types;
pub mod frame;
mod iter;
#[doc(hidden)]
pub use bin_utils;
