use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitInt, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn test_size_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();
    let test_name = format_ident!("sizeof_{}", name);
    let test_mod_name = format_ident!("derive_test_size_{}", name);


    let expected_size: usize;
    let size_attribute = input.get_required_attribute("Size");

    match &size_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let parsed_ints: Punctuated<LitInt, Comma> = list.parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated)
                .unwrap();

            let size_literal = parsed_ints.first().unwrap();

            if size_literal.to_string().starts_with("0x") {

                expected_size = usize::from_str_radix(&size_literal.to_string()[2..], 16).unwrap();

                // panic!("HEX {} EXPECTED {}", size_literal.to_string(), expected_size);

            } else {
                expected_size = size_literal.base10_parse().expect("Size value is invalid");
            }
        }
        _ => {
            panic!("Unsupported attribute type for Size. Please use the #[Size(0x44)] syntax.");
        }
    }

    match input.data {
        Data::Struct(_body) => {
            (quote! {
                #[cfg(test)]
                mod #test_mod_name {
                    use super::*;

                    #[test]
                    fn #test_name() {
                        let m: #name = Default::default();

                        let mut total_size: usize = 0;

                        let mut writer = std::io::Cursor::new(Vec::new());
                        <std::io::Cursor<std::vec::Vec<u8>> as binrw::BinWriterExt>::write_ne(&mut writer, &m).unwrap();
                        let written = writer.get_ref().clone();
                        let total_size = written.len();

                        assert_eq!(#expected_size, total_size, "Expected size = {} ({:X}), Actual size = {} ({:X})", #expected_size, #expected_size, total_size, total_size);
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply TestSize derive to a non-struct!")
    }
}