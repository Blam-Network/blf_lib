use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, LitInt, Meta, Token};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helpers::DeriveInputHelpers;

pub fn test_size_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();
    let test_name = format_ident!("sizeof_{}", name);


    let expected_size: usize;

    let size_attribute = input.get_required_attribute("Size");
    let pack_attribute = input.get_required_attribute("PackedSerialize");

    let alignment: usize;
    match &pack_attribute.meta {
        Meta::List(list) => {
            let mut iterator = list.clone().tokens.into_iter();
            alignment = iterator.next().unwrap().to_string().parse::<usize>().expect("Invalid pack value provided.");
        }
        _ => {
            panic!("Unsupported attribute type for PackedEncode. Please use the #[PackedEncode(4)] syntax.");
        }
    }

    match &size_attribute.meta {
        // Consider switching to a NameValue attribute.
        Meta::List(list) => {
            let parsed_ints: Punctuated<LitInt, Comma> = list.parse_args_with(Punctuated::<LitInt, Token![,]>::parse_terminated)
                .unwrap();

            let size_literal = parsed_ints.first().unwrap();

            expected_size = size_literal.base10_parse().expect("Size value is invalid");
        }
        _ => {
            panic!("Unsupported attribute type for Size. Please use the #[Size(0x44)] syntax.");
        }
    }

    match input.data {
        Data::Struct(body) => {
            let adds = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());
                quote! {
                    let p = unsafe {
                        core::ptr::addr_of!((*(&m as *const _ as *const #name)).#field_name)
                    };
                    let value_size = size_of_raw(p);
                    let pad_size = (#alignment - (size_of_raw(p) % #alignment)) % #alignment;
                    total_size += value_size + pad_size;
                }
            });


            (quote! {
                #[cfg(test)]
                mod derive_test_size {
                    use super::*;

                    const fn size_of_raw<T>(_: *const T) -> usize {
                        core::mem::size_of::<T>()
                    }

                    #[test]
                    fn #test_name() {
                        let m = core::mem::MaybeUninit::<#name>::uninit();

                        let mut total_size: usize = 0;

                        #(#adds)*

                        assert_eq!(total_size, #expected_size);
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply TestSize derive to a non-struct!")
    }
}