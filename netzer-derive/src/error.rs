use proc_macro2::{ Span, TokenStream };
use syn::Ident;


pub(crate) struct DeriveNetEncodeErrorDecl {
    pub(crate) ident    : Option<Ident>,
    pub(crate) variants : Vec<(Ident, TokenStream,)>
}
impl DeriveNetEncodeErrorDecl {

    pub(crate) fn empty() -> Self { Self {
        ident    : None,
        variants : Vec::new()
    } }

    fn new(error : Option<&Ident>, fallback : &Ident, suffix : &str) -> Self { Self {
        ident    : Some(error.cloned().unwrap_or_else(|| Ident::new(&format!("{fallback}{suffix}"), Span::call_site()))),
        variants : Vec::new()
    } }
    pub(crate) fn new_encode(error : Option<&Ident>, fallback : &Ident) -> Self {
        Self::new(error, fallback, "EncodeError")
    }
    // pub(crate) fn new_decode(error : Option<&Ident>, fallback : &Ident) -> Self {
    //     Self::new(error, fallback, "DecodeError")
    // }

}
