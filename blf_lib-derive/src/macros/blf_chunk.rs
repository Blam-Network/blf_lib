use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitFloat, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;
use crate::macros::byte_packed_serializable::byte_packed_serializable_macro;
use proc_macro2::TokenStream as TokenStream2;
use virtue::parse::{Body, Parse};
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use blf_lib_derivable::types::chunk_signature::chunk_signature;
use blf_lib_derive::macros::packed_serialize::packed_serialize_macro;
use crate::macros::test_size::test_size_macro;

pub fn blf_chunk_macro(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let signature_string: String;
    let version_float: f32;
    let mut big_endian = false;
    let mut packing: usize = 1;

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

    if pack_attribute.is_some() {
        match &pack_attribute.unwrap().meta {
            Meta::List(list) => {
                let mut iterator = list.clone().tokens.into_iter();
                packing = iterator.next().unwrap().to_string().parse::<usize>().expect("Invalid pack value provided.");
                iterator.next(); // comma
                let endian_ident = iterator.next().expect("Please provide an endian argument as BigEndian or LittleEndian").to_string();
                if endian_ident == "BigEndian" { big_endian = true; } else if endian_ident == "LittleEndian" { big_endian = false; } else { panic!("Invalid Endian, Please provide an Endian argument as BigEndian or LittleEndian"); }
            }
            _ => {
                panic!("Unsupported attribute type for PackedSerialize. Please use the #[PackedSerialize(4)] syntax.");
            }
        }
    }

    assert_eq!(signature_string.len(), 4, "Signature provided with invalid character length! {signature_string}");
    let bytes = signature_string.as_bytes();
    assert_eq!(bytes.len(), 4, "Signature provided with invalid byte length! {signature_string}");

    // old, to be removed
    let serializable_tokens: TokenStream2 = if pack_attribute.is_some() { byte_packed_serializable_macro(tokens.clone()).into() } else { quote! {} };
    let test_size_tokens: TokenStream2 = if size_attribute.is_some() { test_size_macro(tokens.clone()).into() } else { quote! {} };
    let serialize_tokens: TokenStream2 = if pack_attribute.is_some() { packed_serialize_macro(tokens.clone()).into() } else { quote! {} };
    let serializable_chunk_tokens: TokenStream2 = if pack_attribute.is_some() { generate_serializable_chunk(tokens.clone(), big_endian, packing, &signature_string).into() } else { quote! {} };

    match input.data {
        Data::Struct(_s) => {
            quote! {
                // #serializable_tokens
                #serializable_chunk_tokens
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

pub fn generate_serializable_chunk(input: TokenStream, big_endian: bool, packing: usize, signature_string: &String) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let update_eof_tokens = if signature_string == "_eof" { quote! {
        self.update_eof(previously_written);
    }} else { quote! {} };

    match input.data {
        Data::Struct(body) => {
            (quote! {
                impl blf_lib::blf::chunks::SerializableBlfChunk for #name {
                    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
                        #update_eof_tokens

                        <Self as blf_lib::io::packed_encoding::PackedEncoder>::encode_packed(
                            self,
                            blf_lib::io::endian::Endianness::new(#big_endian),
                            blf_lib::io::packing::Packing::new(#packing)
                        )
                    }

                    fn decode_body(&mut self, buffer: &[u8]) {
                        todo!();
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply PackedSerialize derive to a non-struct!")
    }
}