use crate::value::ValueAttrArgs;
use darling::{
    FromDeriveInput, FromVariant,
    util::SpannedValue
};


pub(crate) mod encode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct EnumDeriveAttrArgs {
    #[darling(default)]
    ordinal : SpannedValue<bool>,
    #[darling(default)]
    nominal : SpannedValue<bool>,

    #[darling(flatten)]
    value : ValueAttrArgs
}

#[derive(Debug, FromVariant)]
#[darling(attributes(netzer))]
struct EnumVariantAttrArgs {
    #[darling(default)]
    rename : Option<SpannedValue<String>>
}
