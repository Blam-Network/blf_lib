use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Meta, Ident, parenthesized};
use syn::token::Group;
use crate::bincode_packed::derive_lib::{derive_decode_inner, derive_encode_inner};
use crate::helpers::{DeriveInputHelpers};

pub fn unpacked_serializable_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let name_string = name.to_string();

    let mut big_endian = cfg!(target_endian = "big");

    let big_endian_attribute = input.get_attribute("BigEndian");
    let little_endian_attribute = input.get_attribute("LittleEndian");

    if big_endian_attribute.is_none() && little_endian_attribute.is_none() {
        panic!("Please provide an Endian attribute for UnpackedSerializable.")
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

    let mut alignment: usize;

    let repr_attribute = input.get_attribute("repr");

    if repr_attribute.is_some() {
        match &repr_attribute.unwrap().meta {
            // Consider switching to a NameValue attribute.
            Meta::List(list) => {
                // map tokens to string
                // panic!("{}", list.tokens.to_string());
                // c...
                let mut iterator = list.clone().tokens.into_iter();
                let first_ident = iterator.next();
                if first_ident.unwrap().to_string().to_lowercase() != "c" {
                    panic!("non-c structs aren't supported by UnpackedSerializable");
                }

                // comma...
                iterator.next();
                // packed or align
                let second_ident = iterator.next().unwrap();

                if second_ident.to_string() == "packed" {
                    alignment = 1
                } else if second_ident.to_string() == "align" {
                    let alignment_string = iterator.next().expect("Bad alignment on struct!").to_string();
                    let alignment_string = alignment_string.replace("(", "");
                    let alignment_string = alignment_string.replace(")", "");
                    alignment = alignment_string.parse::<usize>().expect("Unrecognized alignment provided.");
                } else {
                    panic!("{:?}", second_ident)
                }
            }
            _ => {
                panic!("non-list repr provided!");
            }
        }
    } else {
        panic!("repr attribute is required for unpacked serialization");
    }

    let encode_packed_tokens: TokenStream2 = derive_encode_inner(token_stream.clone(), alignment).expect("Failed to generate encoder!").into();
    let decode_packed_tokens: TokenStream2 = derive_decode_inner(token_stream.clone(), alignment).expect("Failed to generate decoder!").into();

    match input.data {
        Data::Struct(_s) => {
            quote! {
                use bincode::de::read::Reader;
                #encode_packed_tokens
                #decode_packed_tokens
                impl blf_lib_derivable::blf::chunks::Serializable for #name {
                    fn encode_chunk(&self) -> Vec<u8> {
                        let config = bincode::config::standard()
                            .with_fixed_int_encoding()
                            .#with_endian();

                        bincode::encode_to_vec(&self, config).expect("Failed to encode #(name)!")
                    }

                    // TODO: Rewrite to consider byte-order.
                    // Probably need to fetch all fields and write in sequence...
                    // And many implement a writable trait...
                    fn decode_chunk(buffer: &[u8]) -> Self {
                        let config = bincode::config::standard()
                            .with_fixed_int_encoding()
                            .#with_endian();


                        bincode::decode_from_slice(buffer, config).unwrap().0
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}