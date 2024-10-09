use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

pub fn blf_file_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();

    match input.data {
        Data::Struct(body) => {
            let writes = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());
                let field_type_ident = field.ty.to_token_stream().into_iter().next().expect("Failed to get field type.");

                // hacky
                let eof_update = if field_type_ident.to_string() == "s_blf_chunk_end_of_file" {
                    (quote! {
                        if #field_type_ident::get_signature().to_string() == "_eof" {
                            self.#field_name.file_size = u32::try_from(data.len()).unwrap();
                        }
                    })
                } else { quote! {}};


                quote! {
                    #eof_update
                    data.append(&mut self.#field_name.write());
                }
            });


            (quote! {
                use std::fs::File;
                use std::io::Write;
                use blf_lib::blf::chunks::{Serializable,BlfChunk};
                impl blf_lib::blf::BlfFile for #name {
                    fn write(&mut self, path: &str) {
                        let mut data: Vec<u8> = Vec::new();
                        #(#writes)*

                        let mut file = File::create(path)
                            .unwrap()
                            .write_all(&data);
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply TestSize derive to a non-struct!")
    }
}