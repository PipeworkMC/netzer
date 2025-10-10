use crate::value::ValueAttrArgs;
use darling::{ FromDeriveInput, FromField };


pub(crate) mod encode;
pub(crate) mod decode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct StructDeriveAttrArgs {
}

#[derive(Debug, FromField)]
#[darling(attributes(netzer))]
struct StructFieldAttrArgs {
    #[darling(flatten)]
    value : ValueAttrArgs
}
