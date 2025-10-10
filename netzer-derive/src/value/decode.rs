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


pub(crate) fn derive_netdecode_value(
    opts         : &ValueAttrArgs,
    repr         : Option<&Type>,
    ty           : &Type,
    where_clause : &mut WhereClause
) -> TokenStream {

    let bounded_ty = { match (&opts.convert) {
        Some(spanned) => spanned.as_ref(),
        None => { if let Some(repr) = repr { repr } else { ty } }
    } };

    let decode;
    let bound;
    match ((&opts.format, &opts.decode_with,)) {
        (Some(_), Some(_),) => { return quote!{ compile_error!("value may not have both `format` and `decode_with`"); }; },

        (None, Some(spanned),) => {
            let decode_with = &**spanned;
            decode = quote_spanned!{ spanned.span() =>
                ::netzer::DecodeWith::<#bounded_ty, _>::decode(
                    &mut #decode_with,
                    &mut netzer_derive_netdecode_reader
                ).await?
            };
            bound = None;
        },

        (Some(spanned), None,) => {
            let format = &**spanned;
            decode = quote_spanned!{ spanned.span() =>
                <#bounded_ty as ::netzer::NetDecode::<#format>>
                    ::decode(&mut netzer_derive_netdecode_reader).await?
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
                            ident     : Ident::new("NetDecode", Span::call_site()),
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
            decode = quote!{
                <#bounded_ty as ::netzer::NetDecode::<Inherit>>
                    ::decode(&mut netzer_derive_netdecode_reader).await?
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
                            ident     : Ident::new("NetDecode", Span::call_site()),
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

    let (from_trait, from_method, from_after,) = {
        if (opts.try_from.is_present()) {
            (quote!{ ::core::convert::TryFrom }, quote!{ try_from }, quote!{ ? },)
        } else {
            (quote!{ ::core::convert::From }, quote!{ from }, quote!{ },)
        }
    };

    let value = { match (&opts.convert) {
        Some(spanned) => {
            let convert = &**spanned;
            quote_spanned!{ spanned.span() => #from_trait::<#convert>::#from_method(#decode)#from_after }
        },
        None => {
            if let Some(repr) = repr {
                quote_spanned!{ repr.span() => #from_trait::<#repr>::#from_method(#decode)#from_after }
            } else {
                if (opts.try_from.is_present()) {
                    return quote_spanned!{ opts.try_from.span() => compile_error!("`value may not have #[netzer(try_from)]` without a conversion type"); };
                }
                decode
            }
        }
    } };

    if let Some(bound) = bound {
        where_clause.predicates.push(WherePredicate::Type(PredicateType {
            lifetimes   : None,
            bounded_ty  : bounded_ty.clone(),
            colon_token : Token![:](Span::call_site()),
            bounds      : Punctuated::from_iter([ bound ])
        }));
    }
    value
}
