use crate::{
    value::encode::derive_netencode_value,
    structs::{
        StructDeriveAttrArgs,
        StructFieldAttrArgs
    },
    error::DeriveNetEncodeErrorDecl,
    util::ident_or
};
use proc_macro2::TokenStream;
use syn::{
    DeriveInput, DataStruct,
    Fields,
    Ident,
    spanned::Spanned as _
};
use quote::quote;
use darling::{ FromDeriveInput, FromField };
use convert_case::{ Case, Casing };


pub(crate) fn derive_netencode_struct_encode(input : &DeriveInput, data : &DataStruct) -> (TokenStream, DeriveNetEncodeErrorDecl,) {
    let args = { match (StructDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return (err.write_errors(), DeriveNetEncodeErrorDecl::empty(),); }
    } };

    let mut error_decl = DeriveNetEncodeErrorDecl::new_encode(args.encode_error.as_ref(), &input.ident);

    let field_idents = data.fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
    let field_idents = quote!{ #( #field_idents , )* };
    let destructure = { match (data.fields) {
        Fields::Named(_)   => quote!{ { #field_idents } },
        Fields::Unnamed(_) => quote!{ ( #field_idents ) },
        Fields::Unit       => quote!{ },
    } };
    let encode_fields = derive_netencode_struct_fields(&data.fields, String::new(), &mut error_decl);
    (quote!{
        let Self #destructure = &self;
        #encode_fields
    }, error_decl,)
}


pub(crate) fn derive_netencode_struct_fields(fields : &Fields, error_variant_prefix : String, error_decl : &mut DeriveNetEncodeErrorDecl) -> TokenStream {
    let mut encodes = quote!{ };
    for (i, field,) in fields.into_iter().enumerate() {
        let args = { match (StructFieldAttrArgs::from_field(field)) {
            Ok(args) => args,
            Err(err) => { return err.write_errors(); }
        } };

        let ident = ident_or(i, field);
        let error_variant = field.ident.as_ref().map_or_else(|| format!("E{i}"), |i| i.to_string().to_case(Case::Pascal));
        encodes.extend(derive_netencode_value(
            &args.value,
            None,
            &field.ty,
            quote!{ #ident },
            Ident::new(&format!("{error_variant_prefix}{error_variant}"), field.span()),
            error_decl
        ));
    }
    encodes
}
