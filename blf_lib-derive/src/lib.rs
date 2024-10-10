extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

extern crate self as blf_lib_derive;

use proc_macro::TokenStream;

mod helpers;
mod macros;
mod bincode_packed;

#[proc_macro_derive(BlfChunk, attributes(Signature, Version, PackedEncode, BigEndian, LittleEndian, Size))]
pub fn blf_chunk(input: TokenStream) -> TokenStream {
    macros::blf_chunk::blf_chunk_macro(input)
}

#[proc_macro_derive(BlfFile)]
pub fn blf_file(input: TokenStream) -> TokenStream {
    macros::blf_file::blf_file_macro(input)
}

#[proc_macro_derive(BytePackedEncodeedSerializable, attributes(BigEndian, LittleEndian, PackedEncode, Signature))]
pub fn byte_packed_serializable(input: TokenStream) -> TokenStream {
    macros::byte_packed_serializable::byte_packed_serializable_macro(input)
}

#[proc_macro_derive(TestSize, attributes(Size, PackedEncode))]
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