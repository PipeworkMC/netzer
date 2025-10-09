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
    let function_body = (match (&input.data) {
        Data::Struct(data) => structs ::encode::derive_netencode_struct_encode (&input, data),
        Data::Enum(data)   => enums   ::encode::derive_netencode_enum_encode   (&input, data),
        Data::Union(_)     => { return quote!{ compile_error!("NetEncode can not be derived for unions"); }.into(); },
    });

    let ident = &input.ident;
    quote!{
        impl<NetzerDeriveNetEncodeNetFormat : ::netzer::NetFormat>
            ::netzer::NetEncode<NetzerDeriveNetEncodeNetFormat>
            for #ident
        {
            #[allow(clippy::clone_on_copy, clippy::needless_borrow)]
            async fn encode<NetzerDeriveNetEncodeWrite : ::netzer::AsyncWrite>(&self, mut netzer_derive_netencode_writer : NetzerDeriveNetEncodeWrite) -> ::netzer::Result {
                #function_body
                Ok(())
            }
        }
    }.into()
}


#[proc_macro_derive(NetDecode, attributes(netzer))]
pub fn derive_netdecode(item : TokenStream) -> TokenStream {
    let input         = parse_macro_input!(item as DeriveInput);
    // let function_body = (match (&input.data) {
    //     Data::Struct(data) => structs ::encode::derive_netencode_struct_decode (&input, data),
    //     Data::Enum(data)   => enums   ::encode::derive_netencode_enum_decode   (&input, data),
    //     Data::Union(_)     => { return quote!{ compile_error!("NetDecode can not be derived for unions"); }.into(); },
    // });
    let function_body = quote!{ compile_error!("TODO"); };

    let ident = &input.ident;
    quote!{
        impl<NetzerDeriveNetDecodeNetFormat : ::netzer::NetFormat>
            ::netzer::NetDecode<NetzerDeriveNetDecodeNetFormat>
            for #ident
        {
            #[allow(clippy::clone_on_copy, clippy::needless_borrow)]
            async fn decode<NetzerDeriveNetDecodeRead : ::netzer::AsyncRead>(&self, mut netzer_derive_netdecode_reader : NetzerDeriveDecoderead) -> ::netzer::Result {
                #function_body
                Ok(())
            }
        }
    }.into()
}
