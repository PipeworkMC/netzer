use syn::{
    Ident, Field,
    spanned::Spanned
};


pub(crate) fn ident_or(i : usize, field : &Field) -> Ident {
    field.ident.clone().unwrap_or_else(|| Ident::new(&format!("a{i}"), field.span()))
}
