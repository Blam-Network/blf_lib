use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn blf_file_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();

    match input.data {
        Data::Struct(body) => {
            let writes = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());

                quote! {
                    data.append(&mut self.#field_name.write(&data));
                }
            });

            let reads = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());

                quote! {
                    <File as std::io::Read>::read_exact(&mut file, &mut headerBytes).unwrap();
                    header = derive_s_blf_header::decode(&headerBytes);

                    if header.signature == blf_file.#field_name.signature() && header.version == blf_file.#field_name.version() {
                        let mut body_bytes = vec![0u8; (header.chunk_size as usize) - derive_s_blf_header::size()];
                        <File as std::io::Read>::read_exact(&mut file, body_bytes.as_mut_slice()).unwrap();
                        blf_file.#field_name.decode_body(body_bytes.as_slice());
                    }
                    else {
                        panic!("{} Chunk not found!", blf_file.#field_name.signature().to_string());
                    }
                }
            });


            (quote! {
                use std::fs::File;
                use blf_lib::blf::chunks::ReadableBlfChunk as DeriveReadableBlfChunk;
                use blf_lib::blf::chunks::SerializableBlfChunk as DeriveSerializableBlfChunk;
                use blf_lib::blf::chunks::DynamicBlfChunk as DeriveDynamicBlfChunk;
                use blf_lib::blf::chunks::BlfChunk;
                use blf_lib::blf::s_blf_header as derive_s_blf_header;
                impl blf_lib::blf::BlfFile for #name {
                    fn write(&mut self, path: &String) {
                        let mut data: Vec<u8> = Vec::new();
                        #(#writes)*

                        let mut file = File::create(path)
                            .unwrap();

                        <File as std::io::Write>::write_all(&mut file, &data).unwrap();
                    }

                    fn read(path: &String) -> Self {
                        let mut headerBytes = [0u8; derive_s_blf_header::size()];
                        let mut header: derive_s_blf_header;
                        let mut file = File::open(path).unwrap();

                        let mut blf_file = Self::default();

                        #(#reads)*

                        blf_file
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply BlfFile derive to a non-struct!")
    }
}