use proc_macro2::*;
use quote::*;
use std::result::Result;
use syn::*;

use crate::variant_helpers::VariantHelpers;

use crate::compilation_error::CompilationError;

pub struct EnumInformation<'a> {
    identifier: &'a Ident,
    data: &'a DataEnum,
    impl_generics: ImplGenerics<'a>,
    type_generics: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
}

impl<'a> EnumInformation<'a> {
    pub fn new(input: &'a DeriveInput) -> Result<Self, CompilationError> {
        let identifier = &input.ident;
        let Data::Enum(data) = &input.data else {
            return Err(CompilationError::OnlyAvailableForEnum);
        };
        let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
        Ok(Self {
            identifier,
            data,
            impl_generics,
            type_generics,
            where_clause,
        })
    }

    pub fn implement(&self, functions: TokenStream) -> TokenStream {
        let impl_generics = &self.impl_generics;
        let identifier = &self.identifier;
        let type_generics = &self.type_generics;
        let where_clause = &self.where_clause;
        quote! {
            impl #impl_generics #identifier #type_generics #where_clause {
                #functions
            }
        }
    }

    pub fn generate_is_functions(&self) -> TokenStream {
        self.data
            .variants
            .iter()
            .map(|variant| variant.generate_is_function(&self.identifier))
            .collect()
    }
}
