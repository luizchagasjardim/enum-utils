use proc_macro::TokenStream;
use syn::parse_macro_input;

mod compilation_error;

mod enum_information;
use enum_information::EnumInformation;

mod variant_helpers;

/// Generates all utilities defined in this crate
#[proc_macro_derive(EnumUtils)]
pub fn enum_utils(input: TokenStream) -> TokenStream {
    vec![
        is(input.clone()),
        //inner(input.clone()),
        //map(input),
    ]
    .into_iter()
    .collect()
}

/// Generates the following associated functions:
/// is_\*(), analogous to is_ok()/is_err() from Result.
#[proc_macro_derive(Is)]
pub fn is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let enum_information = match EnumInformation::new(&input) {
        Ok(enum_information) => enum_information,
        Err(compilation_error) => {
            return compilation_error.into();
        }
    };
    let functions = enum_information.generate_is_functions();
    enum_information.implement(functions).into()
}

/// Generates the following associated functions for variants that have inner values:
/// is_*_and(), analogous to is_ok_and()/is_err_and() from Result.
/// *(), analogous to ok()/err() from Result.
/// inspect\_\*(), analogous to inspect()/inspect_err() from Result.
/// expect\_\*(), analogous to expect()/expect_err() from Result.
/// unwrap\_\*(), analogous to unwrap()/unwrap_err() from Result.
/// unwrap\_\*_or_default(), analogous to unwrap_or_default() from Result.
/// unwrap\_\*_unchecked(), analogous to unwrap_unchecked()/unwrap_err_unchecked() from Result.
/// contains\_\*(), analogous to contains()/contains_err() from Result.
#[proc_macro_derive(Inner)]
pub fn inner(_input: TokenStream) -> TokenStream {
    todo!()
}

/// Generates the following associated functions for variants that have inner values:
/// map_*(), analogous to map()/map_err() from Result.
/// map_\*_or(), analogous to map_or()/map_err_or() from Result.
#[proc_macro_derive(Map)]
pub fn map(_input: TokenStream) -> TokenStream {
    todo!()
}
