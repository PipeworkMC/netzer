use crate::value::ValueAttrArgs;
use syn::Ident;
use darling::{ FromDeriveInput, FromVariant };


pub(crate) mod encode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct EnumDeriveAttrArgs {
    #[darling(default)]
    ordinal : bool,
    #[darling(default)]
    nominal : bool,

    #[darling(flatten)]
    value : ValueAttrArgs,

    #[darling(default)]
    encode_error : Option<Ident>,
    // #[darling(default)]
    // decode_error : Option<Ident>
}

#[derive(Debug, FromVariant)]
#[darling(attributes(netzer))]
struct EnumVariantAttrArgs {
    #[darling(default)]
    rename : Option<String>
}
