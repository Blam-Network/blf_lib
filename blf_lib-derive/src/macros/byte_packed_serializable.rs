use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Meta, Ident, parenthesized, LitInt, Token, LitStr};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Group};
use crate::bincode_packed::derive_lib::{derive_decode_inner, derive_encode_inner};
use crate::helpers::{DeriveInputHelpers};

pub fn byte_packed_serializable_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let mut big_endian = cfg!(target_endian = "big");

    let big_endian_attribute = input.get_attribute("BigEndian");
    let little_endian_attribute = input.get_attribute("LittleEndian");

    if big_endian_attribute.is_none() && little_endian_attribute.is_none() {
        panic!("Please provide an Endian attribute for BytePackedSerializable.")
    }

    if big_endian_attribute.is_some() && little_endian_attribute.is_some() {
        panic!("Please provide only one Endian attribute.")
    }

    if big_endian_attribute.is_some() {
        big_endian = true;
    }

    if little_endian_attribute.is_some() {
        big_endian = false;
    }

    let endian_string = if big_endian { "big" } else { "little" };
    let with_endian = format_ident!("with_{}_endian", endian_string);

    let mut alignment: usize = 4; // not sure if this is actually the default

    let pack_attribute = input.get_required_attribute("Pack");

    match &pack_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let parsed_ints: Punctuated<LitInt, Comma> = list.parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated)
                .unwrap();

            let pack_int_literal = parsed_ints.first().unwrap();

            alignment = pack_int_literal.base10_parse().expect("Pack value is invalid");
        }
        _ => {
            panic!("Unsupported attribute type for Version. Please use the #[Pack(4)] syntax.");
        }
    }

    let mut signature_string: Option<String> = None;
    let signature_attribute = input.get_attribute("Signature");
    if signature_attribute.is_some() {
        match &signature_attribute.unwrap().meta {
            // Consider switching to a NameValue attribute.
            Meta::List(list) => {
                // We can drop the Punctuated, because we only deal with single-values.
                let parsed_strings: Punctuated<LitStr, Comma> = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                    .unwrap();
                let signature_string_literal = parsed_strings.first().unwrap();

                signature_string = Some(signature_string_literal.value());
            }
            _ => {
                panic!("Unsupported attribute type for Signnature. Please use the #[Signature(\"athr\")] syntax.");
            }
        }
    }

    let encode_packed_tokens: TokenStream2 = derive_encode_inner(token_stream.clone(), alignment).expect("Failed to generate encoder!").into();
    let decode_packed_tokens: TokenStream2 = derive_decode_inner(token_stream.clone(), alignment).expect("Failed to generate decoder!").into();

    // hacky
    let eof_update = if signature_string.unwrap_or_default() == "_eof" {
        (quote! {
            self.update_eof(&previously_written);
        })
    } else { quote! {}};

    match input.data {
        Data::Struct(_s) => {
            quote! {
                use bincode::de::read::Reader;
                #encode_packed_tokens
                #decode_packed_tokens
                impl blf_lib_derivable::blf::chunks::SerializableBlfChunk for #name {
                    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
                        #eof_update

                        let config = bincode::config::standard()
                            .with_fixed_int_encoding()
                            .#with_endian();

                        let encode_to_vec = |chunk: &Self| bincode::encode_to_vec(chunk, config).expect("Failed to encode #(name)!");
                        encode_to_vec(self)
                    }

                    // TODO: Rewrite to consider byte-order.
                    // Probably need to fetch all fields and write in sequence...
                    // And many implement a writable trait...
                    fn decode_body(&mut self, buffer: &[u8]) {
                        let config = bincode::config::standard()
                            .with_fixed_int_encoding()
                            .#with_endian();


                        bincode::decode_from_slice(buffer, config).unwrap().0
                    }
                }
            }
        }
        _ => { panic!("#[derive(BytePackedSerializable)] is only defined for structs!")}
    }.into()
}