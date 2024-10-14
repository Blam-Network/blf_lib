extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

extern crate self as blf_lib_derive;

use proc_macro::TokenStream;

mod helpers;
mod macros;

#[proc_macro_derive(BlfChunk, attributes(Signature, Version, PackedSerialize, Size))]
pub fn blf_chunk(input: TokenStream) -> TokenStream {
    macros::blf_chunk::blf_chunk_macro(input)
}

#[proc_macro_derive(TitleAndBuild, attributes(Title, Build))]
pub fn title_and_build(input: TokenStream) -> TokenStream {
    macros::title_and_build::title_and_build_macro(input)
}

#[proc_macro_derive(BlfFile)]
pub fn blf_file(input: TokenStream) -> TokenStream {
    macros::blf_file::blf_file_macro(input)
}

#[proc_macro_derive(PackedSerialize)]
pub fn packed_serialize(input: TokenStream) -> TokenStream {
    macros::packed_serialize::packed_serialize_macro(input)
}

#[proc_macro_derive(TestSize, attributes(Size, PackedSerialize))]
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