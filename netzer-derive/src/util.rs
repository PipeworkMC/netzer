use proc_macro2::TokenStream;
use syn::{
    Ident, Field,
    spanned::Spanned as _
};
use quote::quote;


pub(crate) fn ident_or(i : usize, field : &Field) -> Ident {
    field.ident.clone().unwrap_or_else(|| Ident::new(&format!("a{i}"), field.span()))
}


pub(crate) fn finalise_encode(ident : &Ident, function_body : TokenStream) -> TokenStream {
    quote!{
        impl<Inherit : ::netzer::NetFormat> ::netzer::NetEncode<Inherit> for #ident {
            #[allow(clippy::clone_on_copy, clippy::needless_borrow)]
            async fn encode<Writer : ::netzer::AsyncWrite>(&self, mut netzer_derive_netencode_writer : Writer) -> ::netzer::Result {
                #function_body
                Ok(())
            }
        }
    }.into()
}
