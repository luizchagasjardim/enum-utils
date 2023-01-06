use convert_case::*;
use proc_macro2::*;
use quote::*;
use syn::{spanned::Spanned, *};

pub trait VariantHelpers {
    fn expand_fields(&self) -> TokenStream;
    fn generate_is_function(&self, enum_identifier: &Ident) -> TokenStream;
}

impl VariantHelpers for Variant {
    fn expand_fields(&self) -> TokenStream {
        match &self.fields {
            Fields::Named(_) => quote_spanned! { self.span()=> {..} },
            Fields::Unnamed(_) => quote_spanned! { self.span()=> (..) },
            Fields::Unit => quote_spanned! { self.span()=> },
        }
    }
    fn generate_is_function(&self, enum_identifier: &Ident) -> TokenStream {
        let variant_identifier = &self.ident;

        let mut function_identifier =
            format_ident!("is_{}", variant_identifier.to_string().to_case(Case::Snake));
        function_identifier.set_span(variant_identifier.span());

        let variant_fields = self.expand_fields();
        quote_spanned! {self.span()=>
            #[allow(dead_code)]
            fn #function_identifier(&self) -> bool {
                match self {
                    #enum_identifier::#variant_identifier #variant_fields => true,
                    _ => false,
                }
            }
        }
    }
}
