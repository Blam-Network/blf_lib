use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Meta};
use crate::helpers::DeriveInputHelpers;
use proc_macro2::TokenStream as TokenStream2;
use crate::macros::test_size::test_size_macro;

pub fn blf_chunk_macro(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let signature_string: String;
    let version_float: f32;

    let header_attribute = input.get_required_attribute("Header");
    let size_attribute = input.get_attribute("Size");

    match &header_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let mut iterator = list.clone().tokens.into_iter();
            signature_string = iterator.next().unwrap().to_string().replace("\"", "");
            iterator.next(); // comma
            version_float = iterator.next().expect("Please provide the chunk version number.").to_string().parse::<f32>().expect("Invalid chunk version provided.");
        }
        _ => {
            panic!("Unsupported attribute type for Header. Please use the #[Header(\"athr\")] syntax.");
        }
    }

    assert_eq!(signature_string.len(), 4, "Signature provided with invalid character length! {signature_string}");
    let bytes = signature_string.as_bytes();
    assert_eq!(bytes.len(), 4, "Signature provided with invalid byte length! {signature_string}");

    // let test_size_tokens: TokenStream2 = if size_attribute.is_some() { test_size_macro(tokens.clone()).into() } else { quote! {} };

    match input.data {
        Data::Struct(_s) => {
            quote! {
                #test_size_tokens

                impl blf_lib_derivable::blf::chunks::DynamicBlfChunk for #name {
                    fn signature(&self) -> blf_lib_derivable::types::chunk_signature::chunk_signature {
                        blf_lib_derivable::types::chunk_signature::chunk_signature::new([#((#bytes) as std::ffi::c_char), *])
                    }

                    fn version(&self) -> blf_lib_derivable::types::chunk_version::chunk_version {
                        blf_lib_derivable::types::chunk_version::chunk_version::new(#version_float)
                    }
                }
                impl blf_lib_derivable::blf::chunks::BlfChunk for #name {
                    fn get_signature() -> blf_lib_derivable::types::chunk_signature::chunk_signature {
                        blf_lib_derivable::types::chunk_signature::chunk_signature::new([#((#bytes) as std::ffi::c_char), *])
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
