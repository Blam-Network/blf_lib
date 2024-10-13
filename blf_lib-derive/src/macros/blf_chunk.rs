use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitFloat, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;
use crate::macros::byte_packed_serializable::byte_packed_serializable_macro;
use proc_macro2::TokenStream as TokenStream2;
use blf_lib_derive::macros::packed_serialize::packed_serialize_macro;
use crate::macros::test_size::test_size_macro;

pub fn blf_chunk_macro(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let signature_string: String;
    let version_float: f32;

    let signature_attribute = input.get_required_attribute("Signature");
    let version_attribute = input.get_required_attribute("Version");
    let pack_attribute = input.get_attribute("PackedSerialize");
    let size_attribute = input.get_attribute("Size");

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

    assert_eq!(signature_string.len(), 4, "Signature provided with invalid character length! {signature_string}");
    let bytes = signature_string.as_bytes();
    assert_eq!(bytes.len(), 4, "Signature provided with invalid byte length! {signature_string}");

    // old, to be removed
    let serializable_tokens: TokenStream2 = if pack_attribute.is_some() { byte_packed_serializable_macro(tokens.clone()).into() } else { quote! {} };
    let test_size_tokens: TokenStream2 = if size_attribute.is_some() { test_size_macro(tokens.clone()).into() } else { quote! {} };
    let serialize_tokens: TokenStream2 = if pack_attribute.is_some() { packed_serialize_macro(tokens.clone()).into() } else { quote! {} };


    match input.data {
        Data::Struct(_s) => {
            quote! {
                #serializable_tokens
                #serialize_tokens
                #test_size_tokens

                impl blf_lib_derivable::blf::chunks::DynamicBlfChunk for #name {
                    fn signature(&self) -> blf_lib_derivable::types::chunk_signature::chunk_signature {
                        blf_lib_derivable::types::chunk_signature::chunk_signature::new([#((#bytes) as c_char), *])
                    }

                    fn version(&self) -> blf_lib_derivable::types::chunk_version::chunk_version {
                        blf_lib_derivable::types::chunk_version::chunk_version::new(#version_float)
                    }
                }
                impl blf_lib_derivable::blf::chunks::BlfChunk for #name {
                    fn get_signature() -> blf_lib_derivable::types::chunk_signature::chunk_signature {
                        blf_lib_derivable::types::chunk_signature::chunk_signature::new([#((#bytes) as c_char), *])
                    }

                    fn get_version() -> blf_lib_derivable::types::chunk_version::chunk_version {
                        blf_lib_derivable::types::chunk_version::chunk_version::new(#version_float)
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}