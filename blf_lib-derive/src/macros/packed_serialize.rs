use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Data, DeriveInput};
use virtue::generate::Generator;
use virtue::parse::{Body, Fields, Parse};

pub fn packed_serialize_macro(token_stream: TokenStream) -> TokenStream {
    let input = token_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);


    let encode_packed_tokens: TokenStream2 = get_encode_tokens(token_stream).into();
    // let decode_packed_tokens: TokenStream2 = derive_decode_inner(token_stream.clone(), alignment).expect("Failed to generate decoder!").into();

    match input.data {
        Data::Struct(_s) => {
            quote! {
                // use bincode::de::read::Reader as DeriveReader;
                // use std::ops::Deref as DeriveDeref;
                #encode_packed_tokens
                // #decode_packed_tokens
            }
        }
        _ => { panic!("#[derive(PackedSerialize)] is only defined for structs!")}
    }.into()
}

pub fn get_encode_tokens(input: TokenStream) -> TokenStream {
    let parse = Parse::new(input).unwrap();
    let (mut generator, _attributes, body) = parse.into_generator();

    match body {
        Body::Struct(body) => {
            generate_encode(body.fields, &mut generator);
        }
        _ => panic!("Tried to apply packed-bincode derive to a non-struct!")
    }

    generator.finish().unwrap()
}

pub fn generate_encode(fields: Option<Fields>, generator: &mut Generator) {
    generator
        .impl_for("blf_lib::io::packed_encoding::PackedEncoder")
        .generate_fn("encode_packed")
        .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
        .with_arg("endian", "blf_lib::io::endian::Endianness")
        .with_arg("packing", "blf_lib::io::packing::Packing")
        .with_return_type(
            "Vec<u8>",
        )
        .body(|fn_body| {
            fn_body.push_parsed("use blf_lib::io::packed_encoding::PackedEncoder;".to_string())?;
            fn_body.push_parsed("let mut buffer = Vec::<u8>::new();".to_string())?;
            if let Some(fields) = fields.as_ref() {
                for field in fields.names() {
                    fn_body.push_parsed(format!(
                        "buffer.append(&mut self.{field}.encode_packed(endian, packing));",
                    ))?;
                }
            }
            fn_body.push_parsed("buffer")?;
            Ok(())
        }).unwrap();
}
