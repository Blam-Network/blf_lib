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


            (quote! {
                use std::fs::File;
                use std::io::Write;
                use blf_lib::blf::chunks::SerializableBlfChunk as DeriveSerializableBlfChunk;
                use blf_lib::blf::chunks::BlfChunk;
                impl blf_lib::blf::BlfFile for #name {
                    fn write(&mut self, path: &str) {
                        let mut data: Vec<u8> = Vec::new();
                        #(#writes)*

                        let mut file = File::create(path)
                            .unwrap()
                            .write_all(&data);
                    }
                }
                impl #name {
                    fn before_write(&self, previously_written: &Vec<u8>) {}
                }
            }).into()
        }
        _ => panic!("Tried to apply TestSize derive to a non-struct!")
    }
}