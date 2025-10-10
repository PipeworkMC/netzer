use crate::{
    value::encode::derive_netencode_value,
    structs::{
        StructDeriveAttrArgs,
        StructFieldAttrArgs
    },
    util::{
        ident_or,
        finalise_encode
    }
};
use proc_macro2::{ Span, TokenStream };
use syn::{
    DeriveInput, DataStruct,
    Fields,
    WhereClause,
    Token,
    punctuated::Punctuated
};
use quote::quote;
use darling::{ FromDeriveInput, FromField };


pub(crate) fn derive_netencode_struct_encode(input : &DeriveInput, data : &DataStruct) -> TokenStream {
    let _args = { match (StructDeriveAttrArgs::from_derive_input(input)) {
        Ok(args) => args,
        Err(err) => { return err.write_errors(); }
    } };
    let mut where_clause = input.generics.where_clause.clone().unwrap_or_else(|| WhereClause {
        where_token : Token![where](Span::call_site()),
        predicates  : Punctuated::new()
    });

    let field_idents = data.fields.iter().enumerate().map(|(i, field,)| ident_or(i, field));
    let field_idents = quote!{ #( #field_idents , )* };
    let destructure = { match (data.fields) {
        Fields::Named(_)   => quote!{ { #field_idents } },
        Fields::Unnamed(_) => quote!{ ( #field_idents ) },
        Fields::Unit       => quote!{ },
    } };
    let encode_fields = derive_netencode_struct_fields(&data.fields, &mut where_clause);

    finalise_encode(
        &input.ident,
        quote!{
            let Self #destructure = &self;
            #encode_fields
        },
        input.generics.clone(),
        where_clause
    )
}


pub(crate) fn derive_netencode_struct_fields(fields : &Fields, where_clause : &mut WhereClause) -> TokenStream {
    let mut encodes = quote!{ };
    for (i, field,) in fields.into_iter().enumerate() {
        let args = { match (StructFieldAttrArgs::from_field(field)) {
            Ok(args) => args,
            Err(err) => { return err.write_errors(); }
        } };

        let ident = ident_or(i, field);
        encodes.extend(derive_netencode_value(
            &args.value,
            None,
            &field.ty,
            quote!{ #ident },
            where_clause
        ));
    }
    encodes
}
