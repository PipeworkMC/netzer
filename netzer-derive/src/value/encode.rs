use super::ValueAttrArgs;
use proc_macro2::TokenStream;
use quote::quote;


pub(crate) fn derive_netencode_value(opts : &ValueAttrArgs, expr : TokenStream) -> TokenStream {
    let value = { if let Some(convert) = &opts.convert {
        quote!{ ::core::convert::Into<#convert>::into(#expr) }
    } else {
        quote!{ #expr }
    } };

    match ((&opts.protocol, &opts.encode_with,)) {
        (Some(_), Some(_),) => { quote!{ compile_error!("value may not have both `encode_as` and `encode_with`"); } },

        (None, Some(_),) => { todo!("encode_with"); },

        (Some(protocol), None,) => { quote!{
            ::netzer::NetEncode::<#protocol>
                ::encode(#value, &mut netzer_derive_netencode_writer)?;
        } },

        (None, None,) => { quote!{
            ::netzer::NetEncode::<NetzerDeriveNetEncodeProtocol>
                ::encode(#value, &mut netzer_derive_netencode_writer)?;
        } }

    }
}
