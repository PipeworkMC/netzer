use crate::value::ValueAttrArgs;
use proc_macro2::{ Span, TokenStream };
use syn::{
    Type,
    WhereClause, WherePredicate, PredicateType,
    TypeParamBound, TraitBound, TraitBoundModifier,
    Path, PathSegment, PathArguments, AngleBracketedGenericArguments, GenericArgument, TypePath,
    Ident, Token,
    punctuated::Punctuated,
    spanned::Spanned as _
};
use quote::{ quote, quote_spanned };


pub(crate) fn derive_netencode_value(
    opts         : &ValueAttrArgs,
    repr         : Option<&Type>,
    ty           : &Type,
    expr         : TokenStream,
    where_clause : &mut WhereClause
) -> TokenStream {
    let (value, bounded_ty,) = { match ((&opts.convert, &opts.try_convert,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `convert` and `try_convert`"); }; },

        (Some(spanned), None,) => {
            let convert = &**spanned;
            (
                quote_spanned!{ spanned.span() => ::core::convert::Into::<#convert>::into(#expr.clone()) },
                convert,
            )
        },

        (None, Some(spanned),) => {
            let try_convert = &**spanned;
            (
                quote_spanned!{ spanned.span() => ::core::convert::TryInto::<#try_convert>::try_into(#expr.clone())? },
                try_convert,
            )
        },

        (None, None,) => {
            if let Some(repr) = repr { (
                quote_spanned!{ repr.span() => ::core::convert::Into::<#repr>::into(#expr.clone()) },
                repr,
            ) } else { (
                quote!{ #expr },
                ty,
            ) }
        }

    } };

    let encode;
    let bound;
    match ((&opts.format, &opts.encode_with,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `format` and `format_with`"); }; },

        (None, Some(spanned),) => {
            let encode_with = &**spanned;
            encode = quote_spanned!{ spanned.span() =>
                ::netzer::EncodeWith::<_, _>::encode(
                    &mut #encode_with,
                    &#value,
                    &mut netzer_derive_netencode_writer
                ).await?;
            };
            bound = None;
        },

        (Some(spanned), None,) => {
            let format = &**spanned;
            encode = quote_spanned!{ spanned.span() =>
                ::netzer::NetEncode::<#format>
                    ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
            };
            bound = Some(TypeParamBound::Trait(TraitBound {
                paren_token : None,
                modifier    : TraitBoundModifier::None,
                lifetimes   : None,
                path        : Path {
                    leading_colon : Some(Token![::](Span::call_site())),
                    segments      : Punctuated::from_iter([
                        PathSegment {
                            ident     : Ident::new("netzer", Span::call_site()),
                            arguments : PathArguments::None,
                        },
                        PathSegment {
                            ident     : Ident::new("NetEncode", Span::call_site()),
                            arguments : PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token : Some(Token![::](Span::call_site())),
                                lt_token     : Token![<](Span::call_site()),
                                args         : Punctuated::from_iter([ GenericArgument::Type(format.clone()) ]),
                                gt_token     : Token![>](Span::call_site())
                            })
                        }
                    ])
                }
            }));
        },

        (None, None,) => {
            encode = quote!{
                ::netzer::NetEncode::<Inherit>
                    ::encode(&#value, &mut netzer_derive_netencode_writer).await?;
            };
            bound = Some(TypeParamBound::Trait(TraitBound {
                paren_token : None,
                modifier    : TraitBoundModifier::None,
                lifetimes   : None,
                path        : Path {
                    leading_colon : Some(Token![::](Span::call_site())),
                    segments      : Punctuated::from_iter([
                        PathSegment {
                            ident     : Ident::new("netzer", Span::call_site()),
                            arguments : PathArguments::None,
                        },
                        PathSegment {
                            ident     : Ident::new("NetEncode", Span::call_site()),
                            arguments : PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token : Some(Token![::](Span::call_site())),
                                lt_token     : Token![<](Span::call_site()),
                                args         : Punctuated::from_iter([ GenericArgument::Type(Type::Path(TypePath {
                                    qself : None,
                                    path  : Path {
                                        leading_colon : None,
                                        segments      : Punctuated::from_iter([ PathSegment {
                                            ident     : Ident::new("Inherit", Span::call_site()),
                                            arguments : PathArguments::None
                                        } ])
                                    }
                                })) ]),
                                gt_token     : Token![>](Span::call_site())
                            })
                        }
                    ])
                }
            }));
        }

    }

    if let Some(bound) = bound {
        where_clause.predicates.push(WherePredicate::Type(PredicateType {
            lifetimes   : None,
            bounded_ty  : bounded_ty.clone(),
            colon_token : Token![:](Span::call_site()),
            bounds      : Punctuated::from_iter([ bound ])
        }));
    }
    encode
}
