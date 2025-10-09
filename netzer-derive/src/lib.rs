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
#[inline(always)]
pub fn derive_netencode(item : TokenStream) -> TokenStream {
    let input         = parse_macro_input!(item as DeriveInput);
    let function_body = (match (&input.data) {
        Data::Struct(data) => structs ::encode::derive_netencode_struct_encode (&input, data),
        Data::Enum(data)   => enums   ::encode::derive_netencode_enum_encode   (&input, data),
        Data::Union(_)     => { return quote!{ compile_error!("NetEncode can not be derived for unions"); }.into(); },
    });
    let ident = &input.ident;
    quote!{
        impl<NetzerDeriveNetEncodeProtocol : ::netzer::Protocol>
            ::netzer::NetEncode<NetzerDeriveNetEncodeProtocol>
            for #ident
        {
            type Error = (); // TODO
            fn encode<W : ::std::io::Write>(&self, netzer_derive_netencode_writer : W) -> ::core::result::Result<(), <Self as ::netzer::NetEncode<NetzerDeriveNetEncodeProtocol>>::Error> {
                #function_body
                Ok(())
            }
        }
    }.into()
}
