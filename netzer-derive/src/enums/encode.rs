use crate::{
    value::encode::derive_netencode_value,
    structs::encode::derive_netencode_struct_fields,
    enums::{
        EnumDeriveAttrArgs,
        EnumVariantAttrArgs
    },
    util::{
        ident_or,
        finalise_encode
    }
};
use proc_macro2::{ Span, TokenStream };
use syn::{
    DeriveInput, DataEnum,
    Variant, Fields,
    Attribute, Meta, MetaList, MacroDelimiter,
    Path, PathSegment, PathArguments,
    Type, TypePath, TypeReference, Lifetime,
    WhereClause,
    Ident, Token,
    punctuated::Punctuated,
    spanned::Spanned as _
};
use quote::{ quote, quote_spanned };
use darling::{
    FromDeriveInput, FromVariant,
    util::SpannedValue
};


pub(crate) fn derive_netencode_enum_encode(input : &DeriveInput, data : &DataEnum) -> TokenStream {
    let args = { match (EnumDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return err.write_errors(); }
    } };
    let mut where_clause = input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
        where_token : Token![where](Span::call_site()),
        predicates  : Punctuated::new()
    });

    let mut match_body = quote!{ };
    match ((args.ordinal.is_present(), args.nominal.is_present(),)) {
        (false, false,) => { return quote!{ compile_error!("enum must have `#[netzer(ordinal)]` or `#[netzer(nominal)]`"); }; },
        (true, true,) => { return quote!{ compile_error!("enum can not be encoded as both `ordinal` and `nominal`"); }; },

        (true, false,) => { // Ordinal
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
                return quote!{ compile_error!("ordinal-encoded enum must have `#[netzer(convert = \"...\")]` or `#[repr(...)]`"); };
            }

            for variant @ Variant { ident, fields, discriminant, .. } in &data.variants {
                let variant_args = { match (EnumVariantAttrArgs::from_variant(variant)) {
                    Ok(variant_args) => variant_args,
                    Err(err) => { return err.write_errors(); }
                } };

                if let Some(rename) = &variant_args.rename {
                    return quote_spanned!{ rename.span() => compile_error!("variant in ordinal-encoded enum can not be renamed"); };
                }
                let Some((_, discriminant,)) = discriminant
                    else { return quote_spanned!{ variant.span() => compile_error!("ordinal-encoded enum must have a discriminant"); }; };
                let ordinal_encode = derive_netencode_value(
                    &args.value,
                    repr.as_ref(),
                    repr.as_ref().unwrap_or(&Type::Path(TypePath {
                        qself : None,
                        path  : Path {
                            leading_colon : Some(Token![::](Span::call_site())),
                            segments      : Punctuated::from_iter([
                                PathSegment {
                                    ident     : Ident::new("core", Span::call_site()),
                                    arguments : PathArguments::None
                                },
                                PathSegment {
                                    ident     : Ident::new("str", Span::call_site()),
                                    arguments : PathArguments::None
                                }
                            ])
                        }
                    })),
                    quote!{ #discriminant },
                    &mut where_clause
                );

                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let destructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let encode_fields = derive_netencode_struct_fields(fields, &mut where_clause);
                match_body.extend(quote!{ Self::#ident #destructure => {
                    #ordinal_encode
                    #encode_fields
                }, });
            }
        },

        (false, true,) => { // Nominal
            for variant @ Variant { ident, fields, .. } in &data.variants {
                let variant_args = { match (EnumVariantAttrArgs::from_variant(variant)) {
                    Ok(variant_args) => variant_args,
                    Err(err) => { return err.write_errors(); }
                } };

                let name_spanned = variant_args.rename.unwrap_or_else(|| SpannedValue::new(ident.to_string(), ident.span()));
                let name         = &*name_spanned;
                let name_encode = derive_netencode_value(
                    &args.value,
                    None,
                    &Type::Reference(TypeReference {
                        and_token  : Token![&](Span::call_site()),
                        lifetime   : Some(Lifetime {
                            apostrophe : Span::call_site(),
                            ident      : Ident::new("static", Span::call_site())
                        }),
                        mutability : None,
                        elem       : Box::new(Type::Path(TypePath {
                            qself : None,
                            path  : Path {
                                leading_colon : Some(Token![::](Span::call_site())),
                                segments      : Punctuated::from_iter([
                                    PathSegment {
                                        ident     : Ident::new("core", Span::call_site()),
                                        arguments : PathArguments::None
                                    },
                                    PathSegment {
                                        ident     : Ident::new("str", Span::call_site()),
                                        arguments : PathArguments::None
                                    }
                                ])
                            }
                        }))
                    }),
                    quote_spanned!{ name_spanned.span() => #name },
                    &mut where_clause
                );

                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let destructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let encode_fields = derive_netencode_struct_fields(fields, &mut where_clause);
                match_body.extend(quote!{ Self::#ident #destructure => {
                    #name_encode
                    #encode_fields
                }, });
            }
        }

    }

    finalise_encode(
        &input.ident,
        quote!{ match (self) { #match_body } },
        input.generics.clone(),
        where_clause
    )
}
