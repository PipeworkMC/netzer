use proc_macro2::Span;
use syn::{
    WherePredicate,
    Lifetime,
    Ident,
    GenericParam,
    TypeParamBound,
    BoundLifetimes,
    Path,
    CapturedParam,
    PathArguments,
    GenericArgument,
    Type,
    AngleBracketedGenericArguments,
    ReturnType,
    LifetimeParam,
    Token,
    punctuated::Punctuated
};


pub(super) fn make_where_predicate_lifetimes_explicit(predicate : &mut WherePredicate) {
    match (predicate) {

        WherePredicate::Lifetime(_) => {
            // make_lifetime_explicit(&mut predicate_lifetime.lifetime, &mut next_id, &mut lifetimes);
            // for bound in &mut predicate_lifetime.bounds {
            //     make_lifetime_explicit(bound, &mut next_id, &mut lifetimes);
            // }
        },

        WherePredicate::Type(predicate_type) => {
            let mut next_id   = 0;
            let mut lifetimes = Vec::new();

            make_type_lifetimes_explicit(&mut predicate_type.bounded_ty, &mut next_id, &mut lifetimes);
            if let Some(bound) = &mut predicate_type.lifetimes {
                make_bound_lifetimes_explicit(bound, &mut next_id, &mut lifetimes);
            }

            let for_lifetimes = predicate_type.lifetimes.get_or_insert_with(|| BoundLifetimes {
                for_token : Token![for](Span::call_site()),
                lt_token  : Token![<](Span::call_site()),
                lifetimes : Punctuated::new(),
                gt_token  : Token![>](Span::call_site())
            });
            for lifetime in lifetimes {
                for_lifetimes.lifetimes.push(GenericParam::Lifetime(LifetimeParam {
                    attrs       : Vec::new(),
                    lifetime    : Lifetime {
                        apostrophe : Span::call_site(),
                        ident      : lifetime
                    },
                    colon_token : None,
                    bounds      : Punctuated::new()
                }));
            }
        },

        _ => { }
    }
}


fn make_bound_lifetimes_explicit(bound : &mut BoundLifetimes, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    for param in &mut bound.lifetimes {
        match (param) {
            GenericParam::Lifetime(lifetime_param) => {
                make_lifetime_explicit(&mut lifetime_param.lifetime, next_id, lifetimes);
                for bound in &mut lifetime_param.bounds {
                    make_lifetime_explicit(bound, next_id, lifetimes);
                }
            },
            GenericParam::Type(type_param) => {
                for bound in &mut type_param.bounds {
                    make_type_param_bound_lifetimes_explicit(bound, next_id, lifetimes);
                }
            },
            GenericParam::Const(const_param) => {
                make_type_lifetimes_explicit(&mut const_param.ty, next_id, lifetimes);
            }
        }
    }
}


fn make_type_param_bound_lifetimes_explicit(bound : &mut TypeParamBound, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    match (bound) {
        TypeParamBound::Trait(trait_bound) => {
            if let Some(bound) = &mut trait_bound.lifetimes {
                make_bound_lifetimes_explicit(bound, next_id, lifetimes);
            }
            make_path_lifetimes_explicit(&mut trait_bound.path, next_id, lifetimes);
        },
        TypeParamBound::Lifetime(lifetime) => {
            make_lifetime_explicit(lifetime, next_id, lifetimes);
        },
        TypeParamBound::PreciseCapture(precise_capture) => {
            for param in &mut precise_capture.params {
                match (param) {
                    CapturedParam::Lifetime(lifetime) => {
                        make_lifetime_explicit(lifetime, next_id, lifetimes);
                    },
                    CapturedParam::Ident(_) => { },
                    _ => { }
                }
            }
        },
        TypeParamBound::Verbatim(_) => { },
        _ => { }
    }
}


fn make_path_lifetimes_explicit(path : &mut Path, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    for segment in &mut path.segments {
        match (&mut segment.arguments) {
            PathArguments::None => { },
            PathArguments::AngleBracketed(args) => {
                make_angle_bracketed_generic_arguments_lifetimes_explicit(args, next_id, lifetimes);
            },
            PathArguments::Parenthesized(args) => {
                for input in &mut args.inputs {
                    make_type_lifetimes_explicit(input, next_id, lifetimes);
                }
                make_return_type_lifetimes_explicit(&mut args.output, next_id, lifetimes);
            },
        }
    }
}


fn make_angle_bracketed_generic_arguments_lifetimes_explicit(args : &mut AngleBracketedGenericArguments, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    for arg in &mut args.args {
        match (arg) {
            GenericArgument::Lifetime(lifetime) => {
                make_lifetime_explicit(lifetime, next_id, lifetimes);
            },
            GenericArgument::Type(ty) => {
                make_type_lifetimes_explicit(ty, next_id, lifetimes);
            },
            GenericArgument::Const(_) => { },
            GenericArgument::AssocType(assoc_type) => {
                if let Some(generics) = &mut assoc_type.generics {
                    make_angle_bracketed_generic_arguments_lifetimes_explicit(generics, next_id, lifetimes);
                }
                make_type_lifetimes_explicit(&mut assoc_type.ty, next_id, lifetimes);
            },
            GenericArgument::AssocConst(assoc_const) => {
                if let Some(generics) = &mut assoc_const.generics {
                    make_angle_bracketed_generic_arguments_lifetimes_explicit(generics, next_id, lifetimes);
                }
            },
            GenericArgument::Constraint(constraint) => {
                if let Some(generics) = &mut constraint.generics {
                    make_angle_bracketed_generic_arguments_lifetimes_explicit(generics, next_id, lifetimes);
                }
                for bound in &mut constraint.bounds {
                    make_type_param_bound_lifetimes_explicit(bound, next_id, lifetimes);
                }
            },
            _ => { }
        }
    }
}


fn make_type_lifetimes_explicit(ty : &mut Type, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    match (ty) {
        Type::Array(type_array) => {
            make_type_lifetimes_explicit(&mut type_array.elem, next_id, lifetimes);
        },
        Type::BareFn(type_bare_fn) => {
            if let Some(bound) = &mut type_bare_fn.lifetimes {
                make_bound_lifetimes_explicit(bound, next_id, lifetimes);
            }
            for input in &mut type_bare_fn.inputs {
                make_type_lifetimes_explicit(&mut input.ty, next_id, lifetimes);
            }
            make_return_type_lifetimes_explicit(&mut type_bare_fn.output, next_id, lifetimes);

        },
        Type::Group(type_group) => {
            make_type_lifetimes_explicit(&mut type_group.elem, next_id, lifetimes);
        },
        Type::ImplTrait(type_impl_trait) => {
            for bound in &mut type_impl_trait.bounds {
                make_type_param_bound_lifetimes_explicit(bound, next_id, lifetimes);
            }
        },
        Type::Infer(_) => { },
        Type::Macro(_) => { },
        Type::Never(_) => { },
        Type::Paren(type_paren) => {
            make_type_lifetimes_explicit(&mut type_paren.elem, next_id, lifetimes);
        },
        Type::Path(type_path) => {
            if let Some(qself) = &mut type_path.qself {
                make_type_lifetimes_explicit(&mut qself.ty, next_id, lifetimes);
            }
            make_path_lifetimes_explicit(&mut type_path.path, next_id, lifetimes);
        },
        Type::Ptr(type_ptr) => {
            make_type_lifetimes_explicit(&mut type_ptr.elem, next_id, lifetimes);
        },
        Type::Reference(type_reference) => {
            make_option_lifetime_explicit(&mut type_reference.lifetime, next_id, lifetimes);
            make_type_lifetimes_explicit(&mut type_reference.elem, next_id, lifetimes);
        },
        Type::Slice(type_slice) => {
            make_type_lifetimes_explicit(&mut type_slice.elem, next_id, lifetimes);
        },
        Type::TraitObject(type_trait_object) => {
            for bound in &mut type_trait_object.bounds {
                make_type_param_bound_lifetimes_explicit(bound, next_id, lifetimes);
            }
        },
        Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                make_type_lifetimes_explicit(elem, next_id, lifetimes);
            }
        },
        Type::Verbatim(_) => { },
        _ => { }
    }
}


fn make_return_type_lifetimes_explicit(return_type : &mut ReturnType, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    match (return_type) {
        ReturnType::Default => { },
        ReturnType::Type(_, ty) => {
            make_type_lifetimes_explicit(ty, next_id, lifetimes);
        }
    }
}


fn make_option_lifetime_explicit(lifetime : &mut Option<Lifetime>, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    match (lifetime) {
        Some(lifetime) => {
            make_lifetime_explicit(lifetime, next_id, lifetimes);
        },
        None => {
            *lifetime = Some(Lifetime {
                apostrophe : Span::call_site(),
                ident      : next_lifetime_ident(next_id, lifetimes)
            });
        },
    }
}


fn make_lifetime_explicit(lifetime : &mut Lifetime, next_id : &mut usize, lifetimes : &mut Vec<Ident>) {
    if (lifetime.ident == "_") {
        let ident = next_lifetime_ident(next_id, lifetimes);
        lifetime.ident = ident.clone();
    }
}


fn next_lifetime_ident(next_id : &mut usize, lifetimes : &mut Vec<Ident>) -> Ident {
    let ident = Ident::new(&format!("netzer_derive_lifetime_{}", next_id), Span::call_site());
    *next_id += 1;
    lifetimes.push(ident.clone());
    ident
}
