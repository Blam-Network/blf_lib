use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, LitInt, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn test_size_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();
    let test_name = format_ident!("sizeof_{}", name);

    let expected_size: usize;

    let size_attribute = input.get_attribute("Size");


    match &size_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let parsed_ints: Punctuated<LitInt, Comma> = list.parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated)
                .unwrap();

            let size_literal = parsed_ints.first().unwrap();

            expected_size = size_literal.base10_parse().expect("Size value is invalid");
        }
        _ => {
            panic!("Unsupported attribute type for Size. Please use the #[Size(0x44)] syntax.");
        }
    }

    (quote! {
        #[cfg(test)]
        mod derive_test_size {
            use super::*;
            #[test]
            fn #test_name() {
                assert_eq!(size_of::<#name>(), #expected_size);
            }
        }
    }).into()
}