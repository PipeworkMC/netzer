use syn::{ Type, Expr };
use darling::{
    FromMeta,
    util::SpannedValue
};


pub(crate) mod encode;


#[derive(Debug, FromMeta)]
pub(crate) struct ValueAttrArgs {
    #[darling(default)]
    pub(crate) format      : Option<SpannedValue<Type>>,
    #[darling(default)]
    pub(crate) encode_with : Option<SpannedValue<Expr>>,
    #[darling(default)]
    pub(crate) decode_with : Option<SpannedValue<Expr>>,
    #[darling(default)]
    pub(crate) convert     : Option<SpannedValue<Type>>,
    #[darling(default)]
    pub(crate) try_convert : Option<SpannedValue<Type>>
}
