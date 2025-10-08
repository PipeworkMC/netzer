use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;
use syn::{
    DeriveInput,
    parse_macro_input,
    Data, DataStruct, DataEnum, DataUnion,
    WhereClause, WherePredicate, PredicateType, TypeParamBound, TraitBound, TraitBoundModifier,
    Type, Path, TypePath, PathSegment, PathArguments, AngleBracketedGenericArguments, GenericArgument,
    Ident,
    punctuated::Punctuated,
    spanned::Spanned,
    Token
};
use quote::{ quote, quote_spanned };
use darling::{ FromDeriveInput, FromField };
use convert_case::{ Case, Casing };


use super::*;


pub(super) fn derive_netencode(item : TokenStream1) -> TokenStream1 {
    let span = Span::call_site();

    let input = parse_macro_input!(item as DeriveInput);
    let args = match (DeriveAttrArgs::from_derive_input(&input)) {
        Ok(args) => args,
        Err(err) => { return err.write_errors().into(); }
    };
    let DeriveInput { vis, ident, generics, data, .. } = input;
    let (impl_generics, type_generics, where_clause,) = generics.split_for_impl();
    let mut where_clause = where_clause.cloned().unwrap_or(WhereClause {
        where_token : Token![where](span),
        predicates  : Punctuated::new()
    });

    let mut error_body        = quote!{ };
    let     error_ident       = args.error.unwrap_or_else(|| Ident::new(&format!("{ident}DecodeError"), ident.span()));
    let mut error_has_generic = false;
    let mut error_type        = quote!{ #error_ident };
    let mut encode_body       = quote!{ };

    match (data) {

        Data::Struct(DataStruct { fields, .. }) => {
            for (i, (field, member,),) in fields.iter().zip(fields.members()).enumerate() {
                let args = match (FieldAttrArgs::from_field(field)) {
                    Ok(args) => args,
                    Err(err) => { return err.write_errors().into(); }
                };
                let field_access = match (&args.into) {
                    Some(into) => { quote!{ &Into::<#into>::into(self.#member) } },
                    None       => { quote!{ &self.#member } }
                };
                let field_error_ident = Ident::new(
                    &field.ident.as_ref().map_or_else(|| format!("E{i}"), |ident| ident.to_string().to_case(Case::Pascal)),
                    field.ident.as_ref().map_or(span, |ident| ident.span())
                );
                let field_ty = &field.ty;
                match ((args.encode_as, args.encode_with,)) {
                    (Some(_), Some(_),) => { return quote_spanned!{ member.span() => compile_error!("encode_as and encode_with are mutually exclusive"); }.into(); },
                    (None, None,) => {
                        error_body.extend(quote!{ #field_error_ident(<#field_ty as ::netzer::NetEncode<NetzerDeriveNetEncodeGenericsProtocol>>::Error) });
                        encode_body.extend(quote!{
                            ::netzer::NetEncode::<NetzerDeriveNetEncodeGenericsProtocol>::encode(#field_access, writer)
                                .map_err(#error_ident::#field_error_ident)?;
                        });
                        error_has_generic = true;
                    },
                    (Some(encode_as), None,) => {
                        error_body.extend(quote!{ #field_error_ident(<#field_ty as ::netzer::NetEncode<#encode_as>>::Error) });
                        encode_body.extend(quote!{
                            ::netzer::NetEncode::<#encode_as>::encode(#field_access, writer)
                                .map_err(#error_ident::#field_error_ident)?;
                        });
                    },
                    (None, Some(encode_with),) => {
                        encode_body.extend(quote!{ (#encode_with)(#field_access, writer)?; });
                    }
                }
                where_clause.predicates.push(WherePredicate::Type(PredicateType {
                    lifetimes   : None,
                    bounded_ty  : field.ty.clone(),
                    colon_token : Token![:](span),
                    bounds      : Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
                        paren_token : None,
                        modifier    : TraitBoundModifier::None,
                        lifetimes   : None,
                        path        : Path {
                            leading_colon : Some(Token![::](span)),
                            segments      : Punctuated::from_iter([PathSegment {
                                ident     : Ident::new("netzer", span),
                                arguments : PathArguments::None
                            }, PathSegment {
                                ident     : Ident::new("NetEncode", span),
                                arguments : PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    colon2_token : None,
                                    lt_token     : Token!{<}(span),
                                    args         : Punctuated::from_iter([GenericArgument::Type(Type::Path(TypePath {
                                        qself : None,
                                        path  : Path {
                                            leading_colon : None,
                                            segments      : Punctuated::from_iter([PathSegment {
                                                ident     : Ident::new("NetzerDeriveNetEncodeGenericsProtocol", span),
                                                arguments : PathArguments::None
                                            }])
                                        }
                                    }))]),
                                    gt_token     : Token![>](span)
                                })
                            }])
                        }
                    })])
                }));
            }
            if (error_has_generic) {
                error_type.extend(quote!{ #error_ident<NetzerDeriveNetEncodeGenericsProtocol> });
            }
            error_body = quote!{ #vis enum #error_type { #error_body } };
        },

        Data::Enum(DataEnum { enum_token, brace_token, variants }) => todo!(),

        Data::Union(DataUnion { union_token, fields }) => todo!(),

    }

    let mut out = quote!{
        impl<NetzerDeriveNetEncodeGenericsProtocol : ::netzer::Protocol>
            #impl_generics ::netzer::NetEncode<NetzerDeriveNetEncodeGenericsProtocol>
            for #ident #type_generics #where_clause
        {
            type Error = #error_type;
            fn encode<W : ::std::io::Write>(&self, writer : W) -> Result<(), Self::Error> {
                #encode_body
                Ok(())
            }
        }
    };
    out.extend(error_body);
    out.into()

}
