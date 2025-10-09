use crate::{
    value::ValueAttrArgs,
    error::DeriveNetEncodeErrorDecl
};
use proc_macro2::TokenStream;
use syn::{ Ident, Type };
use quote::quote;


pub(crate) fn derive_netencode_value(
    opts          : &ValueAttrArgs,
    repr          : Option<&Type>,
    ty            : &Type,
    expr          : TokenStream,
    error_variant : Ident,
    error_decl    : &mut DeriveNetEncodeErrorDecl
) -> TokenStream {
    let value = { if let Some(convert) = &opts.convert {
        quote!{ ::core::convert::Into::<#convert>::into(#expr) }
    } else if let Some(repr) = repr {
        quote!{ ::core::convert::Into::<#repr>::into(#expr) }
    } else {
        quote!{ #expr }
    } };

    let error_ident = error_decl.ident.as_ref().unwrap();

    let encode;
    let error_ty;
    match ((&opts.protocol, &opts.encode_with,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `encode_as` and `encode_with`"); }; },

        (None, Some(function),) => {
            encode = quote!{
                ::netzer::EncodeWith::<#ty, NetzerDeriveNetEncodeWrite>::encode(
                    &mut #function,
                    &#value,
                    &mut netzer_derive_netencode_writer
                ).await.map_err(#error_ident::#error_variant)?;
            };
            error_ty = quote!{ ::netzer::EncodeWith::<#ty, NetzerDeriveNetEncodeWrite>::Error };
        },

        (Some(protocol), None,) => {
            encode = quote!{
                ::netzer::NetEncode::<#protocol>
                    ::encode(&#value, &mut netzer_derive_netencode_writer).await.map_err(#error_ident::#error_variant)?;
            };
            error_ty = quote!{ ::netzer::NetEncode::<#protocol>::Error };
        },

        (None, None,) => {
            encode = quote!{
                ::netzer::NetEncode::<NetzerDeriveNetEncodeProtocol>
                    ::encode(&#value, &mut netzer_derive_netencode_writer).await.map_err(#error_ident::#error_variant)?;
            };
            error_ty = quote!{ ::netzer::NetEncode::<NetzerDeriveNetEncodeProtocol>::Error };
        }

    };
    error_decl.variants.push((error_variant, error_ty,));

    encode
}
