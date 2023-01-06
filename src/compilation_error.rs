use proc_macro::TokenStream as VanillaTokenStream;
use proc_macro2::*;
use syn::Error;

pub enum CompilationError {
    OnlyAvailableForEnum,
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CompilationError::OnlyAvailableForEnum => {
                write!(formatter, "Only available for enum types")
            }
        }
    }
}

impl Into<TokenStream> for CompilationError {
    fn into(self) -> TokenStream {
        Error::new(Span::call_site(), self.to_string()).to_compile_error()
    }
}

impl Into<VanillaTokenStream> for CompilationError {
    fn into(self) -> VanillaTokenStream {
        <CompilationError as Into<TokenStream>>::into(self).into()
    }
}
