use proc_macro2::{ Span, TokenStream };
use syn::{
    Ident, Field,
    Generics, GenericParam,
    TypeParam, TypeParamBound, TraitBound, TraitBoundModifier,
    WhereClause,
    Path, PathSegment, PathArguments,
    Token,
    punctuated::Punctuated,
    spanned::Spanned as _
};
use quote::quote;


mod where_clause;


pub(crate) fn ident_or(i : usize, field : &Field) -> Ident {
    field.ident.clone().unwrap_or_else(|| Ident::new(&format!("a{i}"), field.span()))
}


pub(crate) fn finalise_encode(ident : &Ident, function_body : TokenStream, mut generics : Generics, added_where_clause : WhereClause) -> TokenStream {
    let where_clause = generics.where_clause.get_or_insert_with(|| WhereClause {
        where_token : Token![where](Span::call_site()),
        predicates  : Punctuated::new()
    });
    where_clause.predicates.extend(added_where_clause.predicates.iter().cloned().map(|mut predicate| {
        where_clause::make_where_predicate_lifetimes_explicit(&mut predicate);
        predicate
    }));

    let (_, type_generics, where_clause,) = generics.split_for_impl();
    let mut impl_generics = generics.params.clone();
    impl_generics.push(GenericParam::Type(TypeParam {
        attrs       : Vec::new(),
        ident       : Ident::new("Inherit", Span::call_site()),
        colon_token : Some(Token![:](Span::call_site())),
        bounds      : Punctuated::from_iter([
            TypeParamBound::Trait(TraitBound {
                paren_token : None,
                modifier    : TraitBoundModifier::None,
                lifetimes   : None,
                path        : Path {
                    leading_colon : Some(Token![::](Span::call_site())),
                    segments      : Punctuated::from_iter([
                        PathSegment {
                            ident     : Ident::new("netzer", Span::call_site()),
                            arguments : PathArguments::None
                        },
                        PathSegment {
                            ident     : Ident::new("NetFormat", Span::call_site()),
                            arguments : PathArguments::None
                        }
                    ])
                },
            })
        ]),
        eq_token    : None,
        default     : None
    }));

    quote!{
        impl<#impl_generics> ::netzer::NetEncode<Inherit> for #ident #type_generics #where_clause {
            #[allow(unused_variables, clippy::clone_on_copy, clippy::needless_borrow, clippy::unnecessary_cast)]
            async fn encode<Writer : ::netzer::AsyncWrite>(&self, mut netzer_derive_netencode_writer : Writer) -> ::netzer::Result {
                #function_body
                Ok(())
            }
        }
    }.into()
}


pub(crate) fn finalise_decode(ident : &Ident, function_body : TokenStream, mut generics : Generics, added_where_clause : WhereClause) -> TokenStream {
    let where_clause = generics.where_clause.get_or_insert_with(|| WhereClause {
        where_token : Token![where](Span::call_site()),
        predicates  : Punctuated::new()
    });
    where_clause.predicates.extend(added_where_clause.predicates.iter().cloned().map(|mut predicate| {
        where_clause::make_where_predicate_lifetimes_explicit(&mut predicate);
        predicate
    }));

    let (_, type_generics, where_clause,) = generics.split_for_impl();
    let mut impl_generics = generics.params.clone();
    impl_generics.push(GenericParam::Type(TypeParam {
        attrs       : Vec::new(),
        ident       : Ident::new("Inherit", Span::call_site()),
        colon_token : Some(Token![:](Span::call_site())),
        bounds      : Punctuated::from_iter([
            TypeParamBound::Trait(TraitBound {
                paren_token : None,
                modifier    : TraitBoundModifier::None,
                lifetimes   : None,
                path        : Path {
                    leading_colon : Some(Token![::](Span::call_site())),
                    segments      : Punctuated::from_iter([
                        PathSegment {
                            ident     : Ident::new("netzer", Span::call_site()),
                            arguments : PathArguments::None
                        },
                        PathSegment {
                            ident     : Ident::new("NetFormat", Span::call_site()),
                            arguments : PathArguments::None
                        }
                    ])
                },
            })
        ]),
        eq_token    : None,
        default     : None
    }));

    quote!{
        impl<#impl_generics> ::netzer::NetDecode<Inherit> for #ident #type_generics #where_clause {
            #[allow(clippy::clone_on_copy, clippy::needless_borrow)]
            async fn decode<Reader : ::netzer::AsyncRead>(mut netzer_derive_netdecode_reader : Reader) -> ::netzer::Result<Self> {
                #function_body
            }
        }
    }.into()
}
