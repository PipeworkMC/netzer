use crate::{
    value::encode::derive_netencode_value,
    error::DeriveNetEncodeErrorDecl,
    util::ident_or
};
use proc_macro2::TokenStream;
use syn::{
    DeriveInput, DataStruct,
    Fields
};
use quote::quote;
use darling::{ FromDeriveInput, FromField };


pub(crate) fn derive_netencode_struct_encode(input : &DeriveInput, data : &DataStruct) -> (TokenStream, DeriveNetEncodeErrorDecl,) {
    let mut error_decl = DeriveNetEncodeErrorDecl::default();

    let _args = { match (super::StructDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return (err.write_errors(), error_decl,); }
    } };

    let field_idents = data.fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
    let field_idents = quote!{ #( #field_idents , )* };
    let destructure = { match (data.fields) {
        Fields::Named(_)   => quote!{ { #field_idents } },
        Fields::Unnamed(_) => quote!{ ( #field_idents ) },
        Fields::Unit       => quote!{ },
    } };
    let encode_fields = derive_netencode_struct_fields(&data.fields);
    (quote!{
        let Self #destructure = self;
        #encode_fields
    }, error_decl,)
}


pub(crate) fn derive_netencode_struct_fields(fields : &Fields) -> TokenStream {
    let mut encodes = quote!{ };
    for (i, field,) in fields.into_iter().enumerate() {
        let args = { match (super::StructFieldAttrArgs::from_field(field)) {
            Ok(args) => args,
            Err(err) => { return err.write_errors(); }
        } };

        let ident = ident_or(i, field);
        encodes.extend(derive_netencode_value(&args.value, None, quote!{ #ident }));
    }
    encodes
}
