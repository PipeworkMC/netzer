use syn::{ Type, Expr };
use darling::FromMeta;


pub(crate) mod encode;


#[derive(Debug, FromMeta)]
pub(crate) struct ValueAttrArgs {
    #[darling(default)]
    pub(crate) protocol    : Option<Type>,
    #[darling(default)]
    pub(crate) encode_with : Option<Expr>,
    // #[darling(default)]
    // pub(crate) decode_with : Option<Expr>,
    #[darling(default)]
    pub(crate) convert     : Option<Type>
}
