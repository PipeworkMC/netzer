use crate::value::ValueAttrArgs;
use syn::Ident;
use darling::{ FromDeriveInput, FromField };


pub(crate) mod encode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct StructDeriveAttrArgs {
    #[darling(default)]
    encode_error : Option<Ident>,
    // #[darling(default)]
    // decode_error : Option<Ident>
}

#[derive(Debug, FromField)]
#[darling(attributes(netzer))]
struct StructFieldAttrArgs {
    #[darling(flatten)]
    value : ValueAttrArgs,

    #[darling(default)]
    error : Option<Ident>
}
