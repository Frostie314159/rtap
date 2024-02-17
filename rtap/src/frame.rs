use scroll::{ctx::TryFromCtx, Endian, Pread};

use crate::{field_types::RadiotapField, iter::create_radiotap_iterator};

pub struct RadiotapFrame<'a> {
    field_bytes: &'a [u8],
    pub payload: &'a [u8]
}
impl<'a> TryFromCtx<'a> for RadiotapFrame<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        offset += 2; // Skip version and padding
        let skip_length = from.gread_with::<u16>(&mut offset, Endian::Little)? as usize;
        let field_bytes = from.gread_with(&mut offset, skip_length - 4)?;
        let payload_length = from.len() - offset;
        if payload_length != 0 {

        }
        let payload = from.gread_with(&mut offset, payload_length)?;
        Ok((
            Self {
                field_bytes,
                payload,
            },
            offset,
        ))
    }
}
impl<'a> RadiotapFrame<'a> {
    pub fn get_field_iter(&'a self) -> impl Iterator<Item = RadiotapField> + 'a {
        create_radiotap_iterator(self.field_bytes)
    }
}
