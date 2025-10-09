use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    DeriveInput, Data
};
use quote::quote;


mod value;
mod structs;
mod enums;
mod error;

mod util;


#[proc_macro_derive(NetEncode, attributes(netzer))]
pub fn derive_netencode(item : TokenStream) -> TokenStream {
    let input         = parse_macro_input!(item as DeriveInput);
    let (function_body, error_decl,) = (match (&input.data) {
        Data::Struct(data) => structs ::encode::derive_netencode_struct_encode (&input, data),
        Data::Enum(data)   => enums   ::encode::derive_netencode_enum_encode   (&input, data),
        Data::Union(_)     => { return quote!{ compile_error!("NetEncode can not be derived for unions"); }.into(); },
    });

    let vis   = input.vis;
    let ident = &input.ident;

    let error_ident = error_decl.ident;

    quote!{

        impl<NetzerDeriveNetEncodeProtocol : ::netzer::Protocol>
            ::netzer::NetEncode<NetzerDeriveNetEncodeProtocol>
            for #ident
        {
            type Error = #error_ident;
            async fn encode<NetzerDeriveNetEncodeWrite : ::netzer::AsyncWrite>(&self, mut netzer_derive_netencode_writer : NetzerDeriveNetEncodeWrite) -> ::core::result::Result<(), <Self as ::netzer::NetEncode<NetzerDeriveNetEncodeProtocol>>::Error> {
                #function_body
                Ok(())
            }
        }

        #vis enum #error_ident {}

    }.into()
}


#[proc_macro_derive(NetDecode, attributes(netzer))]
pub fn derive_netdecode(_item : TokenStream) -> TokenStream {
    todo!("derive NetDecode")
}
