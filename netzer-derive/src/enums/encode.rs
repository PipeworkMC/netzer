use crate::{
    value::encode::derive_netencode_value,
    structs::encode::derive_netencode_struct_fields,
    util::ident_or
};
use proc_macro2::TokenStream;
use syn::{
    DeriveInput, DataEnum,
    Variant, Fields,
    spanned::Spanned
};
use quote::{ quote, quote_spanned };
use darling::FromDeriveInput;


pub(crate) fn derive_netencode_enum_encode(input : &DeriveInput, data : &DataEnum) -> TokenStream {
    let args = { match (super::EnumDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return err.write_errors(); }
    } };

    let mut match_body = quote!{ };
    match ((args.ordinal, args.nominal,)) {
        (false, false,) => { return quote!{ compile_error!("enum must be encoded as `ordinal` or `nominal`"); }; },
        (true, true,) => { return quote!{ compile_error!("enum can not be encoded as both `ordinal` and `nominal`"); }; },

        (true, false,) => {
            for variant @ Variant { ident, fields, discriminant, .. } in &data.variants {
                let Some((_, discriminant,)) = discriminant
                    else { return quote_spanned!{ variant.span() => compile_error!("`ordinal` encoded enum must have a discriminant"); }; };
                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let destructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let ordinal_encode = derive_netencode_value(&args.value, quote!{ #discriminant });
                let encode_fields = derive_netencode_struct_fields(fields);
                match_body.extend(quote!{ Self::#ident #destructure => {
                    #ordinal_encode
                    #encode_fields
                }, });
            }
        },

        (false, true,) => { todo!("nominal"); }

    }
    quote!{ match (self) { #match_body } }
}
