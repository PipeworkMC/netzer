use crate::value::ValueAttrArgs;
use darling::{
    FromDeriveInput, FromVariant,
    util::{
        SpannedValue,
        Flag
    }
};


pub(crate) mod encode;
pub(crate) mod decode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct EnumDeriveAttrArgs {
    #[darling(default)]
    ordinal : Flag,
    #[darling(default)]
    nominal : Flag,

    #[darling(flatten)]
    value : ValueAttrArgs
}

#[derive(Debug, FromVariant)]
#[darling(attributes(netzer))]
struct EnumVariantAttrArgs {
    #[darling(default)]
    rename : Option<SpannedValue<String>>
}
