use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Ident, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn chunk_factory_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let title_string: String;
    let build_string: String;
    let chunk_idents: Vec<Ident>;

    let title_attribute = input.get_attribute("Title");
    let build_attribute = input.get_attribute("Build");
    let chunks_attribute = input.get_attribute("Chunks");

    match &title_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_strings: Punctuated<LitStr, Comma> = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                .unwrap();
            let signature_string_literal = parsed_strings.first().unwrap();

            title_string = signature_string_literal.value();
        }
        _ => {
            panic!("Unsupported attribute type for Title. Please use the #[Title(\"Halo 3\")] syntax.");
        }
    }

    match &build_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_strings: Punctuated<LitStr, Comma> = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                .unwrap();
            let signature_string_literal = parsed_strings.first().unwrap();

            build_string = signature_string_literal.value();
        }
        _ => {
            panic!("Unsupported attribute type for Build. Please use the #[Build(\"12070.08.09.05.2031.halo3_ship\")] syntax.");
        }
    }

    match &chunks_attribute.meta {
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_idents: Punctuated<Ident, Comma> = list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap();

            chunk_idents = parsed_idents.iter().map(|ident| ident.clone()).collect();
        }
        _ => {
            panic!("Unsupported attribute type for Build. Please use the #[Build(\"12070.08.09.05.2031.halo3_ship\")] syntax.");
        }
    }

    let if_statements = chunk_idents.iter().map(|chunk_ident| {
        quote! {
            if signature == &#chunk_ident::get_signature() {
                return Ok(Box::new(#chunk_ident::decode(buffer)));
            }
        }
    });

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::ChunkFactory for #name {
                    fn get_build_string() -> &'static str {
                        #build_string
                    }

                    fn get_title() -> &'static str {
                        #title_string
                    }

                    fn decode_chunk(&self, signature: &[c_char; 4], major_version: u16, minor_version: u16, buffer: &[u8]) -> Result<Box<dyn blf_lib_derivable::blf::chunks::DynamicBlfChunk>, &'static str> {
                        #(#if_statements)*

                        Err("Chunk not found!")
                    }
                }
            }
        }
        _ => { panic!("#[derive(ChunkFactory)] is only defined for structs!")}
    }.into()
}