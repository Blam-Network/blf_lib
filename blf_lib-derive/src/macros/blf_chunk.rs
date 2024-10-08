use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitFloat, LitStr, Meta, Token, Ident};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

fn assert_c_repr(input: &DeriveInput) {
    let repr_attribute = input.get_attribute("repr").expect("Please make sure the BLF chunk has a #[repr(C !");

    match &repr_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let first_ident = list.clone().tokens.into_iter().next();
            if first_ident.unwrap().to_string().to_lowercase() != "c" {
                panic!("BLF Chunk has non-c alignment!");
            }
        }
        _ => {
            panic!("non-list repr provided!");
        }
    }
}

pub fn blf_chunk_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    assert_c_repr(&input);

    let signature_string: String;
    let version_float: f32;

    let signature_attribute = input.get_required_attribute("Signature");
    let version_attribute = input.get_required_attribute("Version");

    match &signature_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_strings: Punctuated<LitStr, Comma> = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                .unwrap();
            let signature_string_literal = parsed_strings.first().unwrap();

            signature_string = signature_string_literal.value();
        }
        _ => {
            panic!("Unsupported attribute type for Signnature. Please use the #[Signature(\"athr\")] syntax.");
        }
    }

    match &version_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let parsed_floats: Punctuated<LitFloat, Comma> = list.parse_args_with(Punctuated::<LitFloat, Token![,]>::parse_terminated)
                .unwrap();

            let version_float_literal = parsed_floats.first().unwrap();

            version_float = version_float_literal.base10_parse().expect("Version float value is invalid");
        }
        _ => {
            panic!("Unsupported attribute type for Version. Please use the #[Version(1.2)] syntax.");
        }
    }

    let mut version = [0u16; 2];
    let version_int = (version_float * 10.0) as u32;
    version[0] = (version_int / 10) as u16;
    version[1] = (version_int % 10) as u16;

    assert_eq!(signature_string.len(), 4, "Signature provided with invalid character length! {signature_string}");
    let bytes = signature_string.as_bytes();
    assert_eq!(bytes.len(), 4, "Signature provided with invalid byte length! {signature_string}");

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::DynamicBlfChunk for #name {}
                impl blf_lib_derivable::blf::chunks::BlfChunk for #name {
                    fn get_signature() -> blf_lib_derivable::types::chunk_signature::chunk_signature {
                        blf_lib_derivable::types::chunk_signature::chunk_signature::new([#((#bytes) as c_char), *])
                    }

                    fn get_version() -> [u16; 2] {
                        [#(#version), *]
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}