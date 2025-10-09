use crate::{
    value::encode::derive_netencode_value,
    structs::encode::derive_netencode_struct_fields,
    error::DeriveNetEncodeErrorDecl,
    util::ident_or
};
use proc_macro2::TokenStream;
use syn::{
    DeriveInput, DataEnum,
    Variant, Fields,
    Attribute, Meta, MetaList, MacroDelimiter,
    Path, PathSegment, PathArguments,
    Type,
    spanned::Spanned as _,
};
use quote::{ quote, quote_spanned };
use darling::FromDeriveInput;


pub(crate) fn derive_netencode_enum_encode(input : &DeriveInput, data : &DataEnum) -> (TokenStream, DeriveNetEncodeErrorDecl,) {
    let mut error_decl = DeriveNetEncodeErrorDecl::default();

    let args = { match (super::EnumDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return (err.write_errors(), error_decl,); }
    } };

    let mut repr = None;
    for Attribute { meta, .. } in &input.attrs {
        if let Meta::List(MetaList { path : Path { leading_colon : None, segments }, delimiter : MacroDelimiter::Paren(_), tokens }) = meta
            && let Some(PathSegment { ident, arguments : PathArguments::None }) = segments.get(0)
            && (ident.to_string() == "repr")
            && let Ok(repr_ty) = syn::parse2::<Type>(tokens.clone())
        {
            repr = Some(repr_ty);
            break;
        }
    }
    if (repr.is_none() && args.value.convert.is_none()) {
        return (quote!{ compile_error!("enum must have `#[netzer(convert = \"...\")]` or `#[repr(...)]`"); }, error_decl,);
    }

    let mut match_body = quote!{ };
    match ((args.ordinal, args.nominal,)) {
        (false, false,) => { return (quote!{ compile_error!("enum must be encoded as `ordinal` or `nominal`"); }, error_decl); },
        (true, true,) => { return (quote!{ compile_error!("enum can not be encoded as both `ordinal` and `nominal`"); }, error_decl); },

        (true, false,) => {
            for variant @ Variant { ident, fields, discriminant, .. } in &data.variants {
                let Some((_, discriminant,)) = discriminant
                    else { return (quote_spanned!{ variant.span() => compile_error!("`ordinal` encoded enum must have a discriminant"); }, error_decl); };
                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let destructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let ordinal_encode = derive_netencode_value(&args.value, repr.as_ref(), quote!{ #discriminant });
                let encode_fields = derive_netencode_struct_fields(fields);
                match_body.extend(quote!{ Self::#ident #destructure => {
                    #ordinal_encode
                    #encode_fields
                }, });
            }
        },

        (false, true,) => { todo!("nominal"); }

    }
    (quote!{ match (self) { #match_body } }, error_decl,)
}
