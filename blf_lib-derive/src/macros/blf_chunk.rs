use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitFloat, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;
use proc_macro2::TokenStream as TokenStream2;
use crate::macros::test_size::test_size_macro;

pub fn blf_chunk_macro(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let signature_string: String;
    let version_float: f32;

    let signature_attribute = input.get_required_attribute("Signature");
    let version_attribute = input.get_required_attribute("Version");
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
            panic!("Unsupported attribute type for Signature. Please use the #[Signature(\"athr\")] syntax.");
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

    let test_size_tokens: TokenStream2 = if size_attribute.is_some() { test_size_macro(tokens.clone()).into() } else { quote! {} };
    // let serializable_chunk_tokens: TokenStream2 = generate_serializable_chunk(tokens.clone(), &signature_string).into();

    match input.data {
        Data::Struct(_s) => {
            quote! {
                // #serializable_chunk_tokens
                // #test_size_tokens

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

// pub fn generate_serializable_chunk(input: TokenStream, signature_string: &String) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident.clone();
//
//     let update_eof_tokens = if signature_string == "_eof" { quote! {
//         self.update_eof(previously_written);
//     }} else { quote! {} };
//
//     match input.data {
//         Data::Struct(..) => {
//             (quote! {
//                 impl blf_lib::blf::chunks::SerializableBlfChunk for #name {
//                     fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
//                         #update_eof_tokens
//
//                         let mut writer = std::io::Cursor::new(Vec::<u8>::new());
//                         <std::io::Cursor<std::vec::Vec<u8>> as binrw::BinWriterExt>::write_ne(&mut writer, &self).unwrap();
//                         writer.get_ref().clone()
//                     }
//
//                     fn decode_body(&mut self, buffer: &[u8]) {
//                         let mut reader = std::io::Cursor::new(buffer);
//                         self.clone_from(&<std::io::Cursor<[u8]> as binrw::BinReaderExt>::read_ne(&mut reader).unwrap());
//                     }
//                 }
//             }).into()
//         }
//         _ => panic!("Tried to apply PackedSerialize derive to a non-struct!")
//     }
// }