use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Data, DeriveInput, Meta, Token, LitStr};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::bincode_packed::derive_lib::{derive_decode_inner, derive_encode_inner};
use crate::helpers::{DeriveInputHelpers};

pub fn byte_packed_serializable_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let big_endian;

    let alignment: usize;

    let pack_attribute = input.get_required_attribute("PackedEncode");


    match &pack_attribute.meta {
        Meta::List(list) => {
            let mut iterator = list.clone().tokens.into_iter();
            alignment = iterator.next().unwrap().to_string().parse::<usize>().expect("Invalid pack value provided.");
            iterator.next(); // comma
            let endian_ident = iterator.next().expect("Please provide an endian argument as BigEndian or LittleEndian").to_string();
            if endian_ident == "BigEndian" {big_endian = true;}
            else if endian_ident == "LittleEndian" {big_endian = false;}
            else { panic!("Invalid Endian, Please provide an Endian argument as BigEndian or LittleEndian");}
        }
        _ => {
            panic!("Unsupported attribute type for PackedEncode. Please use the #[PackedEncode(4)] syntax.");
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

    let endian_string = if big_endian { "big" } else { "little" };
    let with_endian = format_ident!("with_{}_endian", endian_string);

    // hacky
    let eof_update = if signature_string.unwrap_or_default() == "_eof" {
        quote! {
            self.update_eof(&previously_written);
        }
    } else { quote! {}};

    match input.data {
        Data::Struct(_s) => {
            quote! {
                use bincode::de::read::Reader as DeriveReader;
                use std::ops::Deref as DeriveDeref;
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

                    fn decode_body(&mut self, buffer: &[u8]) {
                        let config = bincode::config::standard()
                            .with_fixed_int_encoding()
                            .#with_endian();

                        let chunk: #name = bincode::decode_from_slice(buffer, config).unwrap().0;

                        self.clone_from(&chunk);
                    }
                }
            }
        }
        _ => { panic!("#[derive(BytePackedEncodeedSerializable)] is only defined for structs!")}
    }.into()
}