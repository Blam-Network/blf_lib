use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn version_factory_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let version_idents: Vec<Ident>;

    let versions_attribute = input.get_required_attribute("Versions");

    match &versions_attribute.meta {
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_idents: Punctuated<Ident, Comma> = list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap();

                version_idents = parsed_idents.iter().map(|ident| ident.clone()).collect();
        }
        _ => {
            panic!("Unsupported attribute type for Build. Please use the #[Versions(halo3::v12070.08.09.05.2031.halo3_ship)] syntax.");
        }
    }

    let if_statements = version_idents.iter().map(|version_ident| {
        quote! {
            if #version_ident::get_title() == title && #version_ident::get_build_string() == build {
                return Some(Box::from(#version_ident {}))
            }
        }
    });

    match input.data {
        Data::Struct(_s) => {
            quote! {
                use blf_lib::blf::chunks::TitleAndBuild;
                impl #name {
                    pub fn get_version(title: &str, build: &str) -> Option<Box<dyn blf_lib::blf::chunks::ChunkFactory>> {
                        #(#if_statements)*

                        None
                    }
                }
            }
        }
        _ => { panic!("#[derive(VersionFactory)] is only defined for structs!")}
    }.into()
}