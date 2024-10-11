use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitStr, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn title_and_build_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    let title_string: String;
    let build_string: String;

    let title_attribute = input.get_required_attribute("Title");
    let build_attribute = input.get_required_attribute("Build");

    match &title_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            // We can drop the Punctuated, because we only deal with single-values.
            let parsed_strings: Punctuated<LitStr, Comma> = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                .unwrap();
            let title_string_literal = parsed_strings.first().unwrap();

            title_string = title_string_literal.value();
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
            let build_string_literal = parsed_strings.first().unwrap();

            build_string = build_string_literal.value();
        }
        _ => {
            panic!("Unsupported attribute type for Build. Please use the #[Build(\"12070.08.09.05.2031.halo3_ship\")] syntax.");
        }
    }

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib::blf::chunks::TitleAndBuild for #name {
                    fn get_title() -> &'static str {
                        #title_string
                    }

                    fn get_build_string() -> &'static str {
                        #build_string
                    }
                }
                impl blf_lib::blf::chunks::DynTitleAndBuild for #name {
                    fn title(&self) -> String {
                        String::from(#title_string)
                    }

                    fn build_string(&self) -> String {
                        String::from(#build_string)
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}