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

    let title_attribute = input.get_required_attribute("Title");
    let build_attribute = input.get_required_attribute("Build");
    let chunks_attribute = input.get_required_attribute("Chunks");

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
            panic!("Unsupported attribute type for Build. Please use the #[Chunks(s_blf_chunk_author)] syntax.");
        }
    }

    let if_statements = chunk_idents.iter().map(|chunk_ident| {
        quote! {
            if signature == &#chunk_ident::get_signature() && version == #chunk_ident::get_version() {
                return Ok(Box::new(#chunk_ident::decode_chunk(buffer)));
            }
        }
    });

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::TitleAndBuild for #name {
                    fn get_build_string() -> &'static str {
                        #build_string
                    }

                    fn get_title() -> &'static str {
                        #title_string
                    }
                }
                impl blf_lib_derivable::blf::chunks::ChunkFactory for #name {
                    fn decode_chunk(&self, signature: &blf_lib_derivable::types::chunk_signature::chunk_signature, version: blf_lib_derivable::types::chunk_version::chunk_version, buffer: &[u8]) -> Result<Box<dyn blf_lib_derivable::blf::chunks::DynamicBlfChunk>, &'static str> {
                        #(#if_statements)*

                        Err("Chunk not found!")
                    }
                }
            }
        }
        _ => { panic!("#[derive(ChunkFactory)] is only defined for structs!")}
    }.into()
}