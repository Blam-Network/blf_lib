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

#[proc_macro_derive(ChunkFactory, attributes(Title, Build, Chunks))]
pub fn chunk_factory(input: TokenStream) -> TokenStream {
    macros::chunk_factory::chunk_factory_macro(input)
}

#[proc_macro_derive(VersionFactory, attributes(Versions))]
pub fn version_factory(input: TokenStream) -> TokenStream {
    macros::version_factory::version_factory_macro(input)
}