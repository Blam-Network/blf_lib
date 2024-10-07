extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::ffi::c_char;
use syn::{parse_macro_input, Data, DeriveInput, LitFloat, LitInt, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_derive(BlfChunk, attributes(Signature, Version))]
pub fn blf_chunk(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let signature_string: String;
    let version_float: f32;

    let signature_attribute = input.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "Signature"
    ).nth(0).expect("Signature attribute required for deriving BlfChunk!");

    let version_attribute = input.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "Version"
    ).nth(0).expect("Version attribute required for deriving BlfChunk!");

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

    // There's probably a better way of doing this.
    // We spread these out for the quote below.
    let char1 = bytes[0] as c_char;
    let char2 = bytes[1] as c_char;
    let char3 = bytes[2] as c_char;
    let char4 = bytes[3] as c_char;

    let version_major = version[0];
    let version_minor = version[1];

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::BlfChunk for #name {
                    fn get_signature() -> [c_char; 4] {
                         [(#char1), (#char2), (#char3), (#char4)]
                    }

                    fn get_version() -> [u16; 2] {
                        [(#version_major), (#version_minor)]
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}


#[proc_macro_derive(UnpackedSerializable)]
pub fn unpacked_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_string = name.to_string();

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::Serializable for #name {
                    fn encode(&self, buffer: &[u8]) {
                         unimplemented!();
                    }

                    fn decode(buffer: &[u8]) -> Self {
                        let name = #name_string;
                        let mut value: #name = unsafe { std::mem::zeroed() };

                        let value_size = std::mem::size_of::<#name>();
                        unsafe {
                            let value_slice = std::slice::from_raw_parts_mut(&mut value as *mut _ as *mut u8, value_size);
                            if (buffer.len() > value_size) {
                                let difference = buffer.len() - value_size;
                                panic!("Attempted to unpack {name} from a buffer that is {difference} bytes too big!")
                            }
                            if (buffer.len() < value_size) {
                                let difference = value_size - buffer.len() ;
                                panic!("Attempted to unpack {name} from a buffer that is {difference} bytes too small!")
                            }
                            value_slice.copy_from_slice(buffer);
                        }

                        value
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}


#[proc_macro_derive(TestSize, attributes(Size))]
pub fn test_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let test_name = format_ident!("sizeof_{}", name);

    let expected_size: usize;

    let size_attribute = input.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "Size"
    ).nth(0).expect("Size attribute required for deriving TestSize!");

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