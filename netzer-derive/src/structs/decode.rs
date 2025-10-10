use crate::{
    value::decode::derive_netdecode_value,
    structs::{
        StructDeriveAttrArgs,
        StructFieldAttrArgs
    },
    util::{
        ident_or,
        finalise_decode
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


pub(crate) fn derive_netdecode_struct(input : &DeriveInput, data : &DataStruct) -> TokenStream {
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
    let restructure = { match (data.fields) {
        Fields::Named(_)   => quote!{ { #field_idents } },
        Fields::Unnamed(_) => quote!{ ( #field_idents ) },
        Fields::Unit       => quote!{ },
    } };
    let decode_fields = derive_netdecode_struct_fields(&data.fields, &mut where_clause);

    finalise_decode(
        &input.ident,
        quote!{
            #decode_fields
            Ok(Self #restructure)
        },
        input.generics.clone(),
        where_clause
    )
}


pub(crate) fn derive_netdecode_struct_fields(fields : &Fields, where_clause : &mut WhereClause) -> TokenStream {
    let mut decodes = quote!{ };
    for (i, field,) in fields.into_iter().enumerate() {
        let args = { match (StructFieldAttrArgs::from_field(field)) {
            Ok(args) => args,
            Err(err) => { return err.write_errors(); }
        } };

        let ident  = ident_or(i, field);
        let decode = derive_netdecode_value(
            &args.value,
            None,
            &field.ty,
            where_clause
        );
        decodes.extend(quote!{ let #ident = #decode; });
    }
    decodes
}
