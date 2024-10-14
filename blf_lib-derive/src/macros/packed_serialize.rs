use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn packed_serialize_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

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
                    #field: blf_lib::io::packed_decoding::PackedDecoder::decode_packed(cursor, endian, packing),
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
                    ) -> Self {
                        Self {
                            #(#decode_statements)*
                        }
                    }
                }
            }).into()
        }
        _ => { panic!("#[derive(PackedSerialize)] is only defined for structs!")}
    }
}
