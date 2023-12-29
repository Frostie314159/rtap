#![feature(iterator_try_collect)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, token::Brace, Attribute, Ident, Visibility, Token, braced, punctuated::Punctuated, ExprConst, Expr, LitInt, parse_macro_input};
use rtap_consts::*;

extern crate proc_macro;

struct RadiotapHeader {
    attrs: Vec<Attribute>,
    vis: Visibility,
    struct_token: Token![struct],
    struct_name: Ident,
    _brace_token: Brace,
    rtap_fields: Punctuated<Ident, Token![,]>
}
impl Parse for RadiotapHeader {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(RadiotapHeader {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            struct_name: input.parse()?,
            _brace_token: braced!(content in input),
            rtap_fields: content.parse_terminated(Ident::parse, Token![,])?
        })
    }
}
macro_rules! gen_match {
    ($string:expr, $($field:expr),*) => {
        match $string.as_str() {
            $(
                stringify!($field) => $field,
            )*
            _ => return None,
        }
    };
}
fn field_from_string(string: String) -> Option<usize> {
    Some(gen_match! {
        string,

        TSFT,
        FLAGS,
        RATE,
        CHANNEL,
        FHSS,
        DBM_ANTSIGNAL,
        DBM_ANTNOISE,
        LOCK_QUALITY,
        TX_ATTENUATION,
        DB_TX_ATTENUATION,
        DBM_TX_POWER,
        ANTENNA,
        DB_ANTSIGNAL,
        DB_ANTNOISE,
        RX_FLAGS,
        TX_FLAGS,
        RTS_RETRIES,
        DATA_RETRIES,
        XCHANNEL,
        MCS,
        AMPDU_STATUS,
        VHT,
        TIMESTAMP,
        HE,
        HE_MU,
        ZERO_LEN_PSDU,
        LSIG
    })
}
#[proc_macro]
/// Generate a Radiotap header.
/// 
/// ```
/// use rtap_macros::create_radiotap_header;
/// const A: u8 = 0;
/// 
/// create_radiotap_header! {
///     #[derive(Debug)]
///     pub struct Test {
///         A,
///         B,
///     }
/// }
/// panic!("{TEXT:#?}")
/// ```
pub fn create_radiotap_header(input: TokenStream) -> TokenStream {
    let radiotap_header = parse_macro_input!(input as RadiotapHeader);
    let fields = radiotap_header.rtap_fields.into_iter().map(|x| x.to_string());
    quote! {
       pub const TEXT: &'static [&'static str] = &[#(#fields),*];  
    }.into()
}
