use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    DeriveInput, Data
};
use quote::quote;


mod value;
mod structs;
mod enums;

mod util;


#[proc_macro_derive(NetEncode, attributes(netzer))]
pub fn derive_netencode(item : TokenStream) -> TokenStream {
    let input         = parse_macro_input!(item as DeriveInput);
    (match (&input.data) {
        Data::Struct(data) => structs ::encode::derive_netencode_struct (&input, data),
        Data::Enum(data)   => enums   ::encode::derive_netencode_enum   (&input, data),
        Data::Union(_)     => { return quote!{ compile_error!("NetEncode can not be derived for unions"); }.into(); },
    }).into()
}


#[proc_macro_derive(NetDecode, attributes(netzer))]
pub fn derive_netdecode(item : TokenStream) -> TokenStream {
    let input         = parse_macro_input!(item as DeriveInput);
    (match (&input.data) {
        Data::Struct(data) => structs ::decode::derive_netdecode_struct (&input, data),
        Data::Enum(data)   => enums   ::decode::derive_netdecode_enum   (&input, data),
        Data::Union(_)     => { return quote!{ compile_error!("NetDecode can not be derived for unions"); }.into(); },
    }).into()
}
