use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Meta};
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derive::helpers::DeriveInputHelpers;

pub fn packed_serialize_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let mut endian = None;
    let mut packing = None;

    let pack_attribute = input.get_attribute("PackedSerialize");

    if pack_attribute.is_some() {
        match &pack_attribute.unwrap().meta {
            Meta::List(list) => {
                let mut iterator = list.clone().tokens.into_iter();
                packing = Some(iterator.next().unwrap().to_string().parse::<usize>().expect("Invalid pack value provided."));
                iterator.next(); // comma
                let endian_ident = iterator.next().expect("Please provide an endian argument as BigEndian or LittleEndian").to_string();
                if endian_ident == "BigEndian" { endian = Some(Endianness::Big) } else if endian_ident == "LittleEndian" { endian = Some(Endianness::Little); } else { panic!("Invalid Endian, Please provide an Endian argument as BigEndian or LittleEndian"); }
            }
            _ => {
                panic!("Unsupported attribute type for PackedSerialize. Please use the #[PackedSerialize(4)] syntax.");
            }
        }
    }

    match input.data {
        Data::Struct(body) => {
            let encode_statements = body.fields.iter().map(|field| {
                let field = &field.ident;

                quote! {
                    buffer.append(&mut self.#field.encode_packed(endian, packing));
                }
            });

            let decode_statements = body.fields.iter().map(|field| {
                let field = &field.ident;
                quote! {
                    #field: blf_lib::io::packed_decoding::PackedDecoder::decode_packed(cursor, #endian.or_else(endian), #packing.or_else(packing))?,
                }
            });

            (quote! {
                impl blf_lib::io::packed_encoding::PackedEncoder for #name {
                    fn encode_packed(
                        &self,
                        endian: blf_lib::io::endian::Endianness,
                        packing: blf_lib::io::packing::Packing
                    ) -> Vec<u8> {
                        let mut buffer = Vec::<u8>::new();
                        #(#encode_statements)*
                        buffer
                    }
                }
                impl blf_lib::io::packed_decoding::PackedDecoder for #name {
                    fn decode_packed(
                        cursor: &mut std::io::Cursor<&[u8]>,
                        endian: blf_lib::io::endian::Endianness,
                        packing: blf_lib::io::packing::Packing
                    ) -> core::result::Result<#name, String> {
                        Ok(Self {
                            #(#decode_statements)*
                        })
                    }
                }
            }).into()
        }
        _ => { panic!("#[derive(PackedSerialize)] is only defined for structs!")}
    }
}
