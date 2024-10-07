extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod helpers;
mod macros;

#[proc_macro_derive(BlfChunk, attributes(Signature, Version))]
pub fn blf_chunk(input: TokenStream) -> TokenStream {
    macros::blf_chunk::blf_chunk_macro(input)
}

#[proc_macro_derive(UnpackedSerializable)]
pub fn unpacked_serializable(input: TokenStream) -> TokenStream {
    macros::unpacked_serializable::unpacked_serializable_macro(input)
}


#[proc_macro_derive(TestSize, attributes(Size))]
pub fn test_size(input: TokenStream) -> TokenStream {
    macros::test_size::test_size_macro(input)
}