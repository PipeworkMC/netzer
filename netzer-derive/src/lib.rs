use proc_macro::TokenStream;
use syn::{ Expr, Ident, Type };
use darling::{ FromDeriveInput, FromField };


mod encode;


#[derive(Debug, FromDeriveInput)]
#[darling(attributes(netzer))]
struct DeriveAttrArgs {
    #[darling(default)]
    error : Option<Ident>
}

#[derive(Debug, FromField)]
#[darling(attributes(netzer))]
struct FieldAttrArgs {
    #[darling(default)]
    encode_as : Option<Type>,
    #[darling(default)]
    encode_with : Option<Expr>,
    #[darling(default)]
    into : Option<Type>
}


#[proc_macro_derive(NetEncode, attributes(netzer))]
#[inline(always)]
pub fn derive_netencode(item : TokenStream) -> TokenStream {
    encode::derive_netencode(item)
}
