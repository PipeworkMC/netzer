use crate::{
    value::decode::derive_netdecode_value,
    structs::decode::derive_netdecode_struct_fields,
    enums::{
        EnumDeriveAttrArgs,
        EnumVariantAttrArgs
    },
    util::{
        ident_or,
        finalise_decode
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


pub(crate) fn derive_netdecode_enum(input : &DeriveInput, data : &DataEnum) -> TokenStream {
    let args = { match (EnumDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return err.write_errors(); }
    } };
    let mut where_clause = input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
        where_token : Token![where](Span::call_site()),
        predicates  : Punctuated::new()
    });

    let mut function_body = quote!{ };
    match ((args.ordinal.is_present(), args.nominal.is_present(), args.untagged.is_present(),)) {
        (false, false,false,) => { return quote!{ compile_error!("enum must have `#[netzer(ordinal)]` or `#[netzer(nominal)]`"); }; },

        (true, false, false,) => { // Ordinal
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
                return quote!{ compile_error!("ordinal-decoded enum must have `#[netzer(convert = \"...\")]` or `#[repr(...)]`"); };
            }

            let ordinal_type = Type::Path(TypePath {
                qself : None,
                path  : Path {
                    leading_colon : Some(Token![::](Span::call_site())),
                    segments      : Punctuated::from_iter([
                        PathSegment {
                            ident     : Ident::new("netzer", Span::call_site()),
                            arguments : PathArguments::None
                        },
                        PathSegment {
                            ident     : Ident::new("__private", Span::call_site()),
                            arguments : PathArguments::None
                        },
                        PathSegment {
                            ident     : Ident::new("usize", Span::call_site()),
                            arguments : PathArguments::None
                        }
                    ])
                }
            });
            let ordinal_type = repr.as_ref().unwrap_or(&ordinal_type);
            let ordinal_decode = derive_netdecode_value(
                &args.value,
                repr.as_ref(),
                ordinal_type,
                &mut where_clause
            );
            function_body.extend(quote!{
                let netzer_derive_netdecode_enum_ordinal : #ordinal_type = #ordinal_decode;
            });

            for variant @ Variant { ident, fields, discriminant, .. } in &data.variants {
                let variant_args = { match (EnumVariantAttrArgs::from_variant(variant)) {
                    Ok(variant_args) => variant_args,
                    Err(err) => { return err.write_errors(); }
                } };

                if let Some(rename) = &variant_args.rename {
                    return quote_spanned!{ rename.span() => compile_error!("variant in ordinal-decoded enum can not be renamed"); };
                }
                let Some((_, discriminant,)) = discriminant
                    else { return quote_spanned!{ variant.span() => compile_error!("ordinal-decoded enum must have a discriminant"); }; };

                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let restructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let decode_fields = derive_netdecode_struct_fields(fields, &mut where_clause);
                function_body.extend(quote!{
                    if (netzer_derive_netdecode_enum_ordinal == #discriminant) {
                        #decode_fields
                        return Ok(Self::#ident #restructure);
                    }
                });
            }

            function_body.extend(quote!{
                Err(::netzer::BadEnumOrdinal(netzer_derive_netdecode_enum_ordinal).into())
            })
        },

        (false, true, false) => { // Nominal
            let name_decode = derive_netdecode_value(
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
                                    ident     : Ident::new("netzer", Span::call_site()),
                                    arguments : PathArguments::None
                                },
                                PathSegment {
                                    ident     : Ident::new("__private", Span::call_site()),
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
                &mut where_clause
            );
            function_body.extend(quote!{
                let netzer_derive_netdecode_enum_name : String = #name_decode;
            });

            for variant @ Variant { ident, fields, .. } in &data.variants {
                let variant_args = { match (EnumVariantAttrArgs::from_variant(variant)) {
                    Ok(variant_args) => variant_args,
                    Err(err) => { return err.write_errors(); }
                } };

                let name_spanned = variant_args.rename.unwrap_or_else(|| SpannedValue::new(ident.to_string(), ident.span()));
                let name         = &*name_spanned;

                let field_idents = fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
                let field_idents = quote!{ #( #field_idents , )* };
                let restructure = { match (fields) {
                    Fields::Named(_)   => quote!{ { #field_idents } },
                    Fields::Unnamed(_) => quote!{ ( #field_idents ) },
                    Fields::Unit       => quote!{ },
                } };
                let decode_fields = derive_netdecode_struct_fields(fields, &mut where_clause);
                function_body.extend(quote!{
                    if (netzer_derive_netdecode_enum_name == #name) {
                        #decode_fields
                        return Ok(Self::#ident #restructure);
                    }
                });
            }

            function_body.extend(quote!{
                Err(::netzer::BadEnumName(netzer_derive_netdecode_enum_name).into())
            })
        },

        (false, false, true,) => { // Untagged
            return quote!{ compile_error!("derived NetDecode enum can not be decoded `untagged`"); };
        },

        (_, _, _,) => { return quote!{ compile_error!("enum may only be decoded as `ordinal` or `nominal`"); }; },
    }

    finalise_decode(
        &input.ident,
        function_body,
        input.generics.clone(),
        where_clause
    )
}
