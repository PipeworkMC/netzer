use crate::value::ValueAttrArgs;
use proc_macro2::TokenStream;
use syn::{
    Type,
    spanned::Spanned
};
use quote::{ quote, quote_spanned };


pub(crate) fn derive_netencode_value(
    opts : &ValueAttrArgs,
    repr : Option<&Type>,
    expr : TokenStream
) -> TokenStream {
    let value = { match ((&opts.convert, &opts.try_convert,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `convert` and `try_convert`"); }; },

        (Some(spanned), None,) => {
            let convert = &**spanned;
            quote_spanned!{ spanned.span() => ::core::convert::Into::<#convert>::into(#expr.clone()) }
        },

        (None, Some(spanned),) => {
            let convert = &**spanned;
            quote_spanned!{ spanned.span() => ::core::convert::TryInto::<#convert>::try_into(#expr.clone())? }
        },

        (None, None,) => {
            if let Some(repr) = repr {
                quote_spanned!{ repr.span() => ::core::convert::Into::<#repr>::into(#expr.clone()) }
            } else {
                quote!{ #expr }
            }
        }

    } };

    match ((&opts.format, &opts.format_with,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `format` and `format_with`"); }; },

        (Some(spanned), None,) => {
            let format = &**spanned;
            quote_spanned!{ spanned.span() =>
                ::netzer::NetEncode::<#format>
                    ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
            }
        },

        (None, Some(spanned),) => {
            let function = &**spanned;
            quote_spanned!{ spanned.span() =>
                ::netzer::EncodeWith::<_, _>::encode(
                    &mut #function,
                    &#value,
                    &mut netzer_derive_netencode_writer
                ).await?;
            }
        },

        (None, None,) => quote!{
            ::netzer::NetEncode::<NetzerDeriveNetEncodeNetFormats>
                ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
        }

    }
}
