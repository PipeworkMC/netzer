use crate::value::ValueAttrArgs;
use proc_macro2::TokenStream;
use syn::Type;
use quote::quote;


pub(crate) fn derive_netencode_value(
    opts : &ValueAttrArgs,
    repr : Option<&Type>,
    ty   : &Type,
    expr : TokenStream
) -> TokenStream {
    let value = { if let Some(convert) = &opts.convert {
        quote!{ ::core::convert::Into::<#convert>::into(#expr.clone()) }
    } else if let Some(repr) = repr {
        quote!{ ::core::convert::Into::<#repr>::into(#expr.clone()) }
    } else {
        quote!{ #expr }
    } };

    match ((&opts.protocol, &opts.encode_with,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `encode_as` and `encode_with`"); }; },

        (None, Some(function),) => quote!{
            ::netzer::EncodeWith::<_, _>::encode(
                &mut #function,
                &#value,
                &mut netzer_derive_netencode_writer
            ).await?;
        },

        (Some(protocol), None,) => quote!{
            ::netzer::NetEncode::<#protocol>
                ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
        },

        (None, None,) => quote!{
            ::netzer::NetEncode::<NetzerDeriveNetEncodeProtocol>
                ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
        }

    }
}
