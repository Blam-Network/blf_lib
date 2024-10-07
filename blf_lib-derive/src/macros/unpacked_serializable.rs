use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn unpacked_serializable_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_string = name.to_string();

    match input.data {
        Data::Struct(_s) => {
            quote! {
                impl blf_lib_derivable::blf::chunks::Serializable for #name {
                    fn encode(&self) -> &[u8] {
                         unimplemented!();
                    }

                    fn decode(buffer: &[u8]) -> Self {
                        let name = #name_string;
                        let mut value: #name = unsafe { std::mem::zeroed() };

                        let value_size = std::mem::size_of::<#name>();
                        unsafe {
                            let value_slice = std::slice::from_raw_parts_mut(&mut value as *mut _ as *mut u8, value_size);
                            if (buffer.len() > value_size) {
                                let difference = buffer.len() - value_size;
                                panic!("Attempted to unpack {name} from a buffer that is {difference} bytes too big!")
                            }
                            if (buffer.len() < value_size) {
                                let difference = value_size - buffer.len() ;
                                panic!("Attempted to unpack {name} from a buffer that is {difference} bytes too small!")
                            }
                            value_slice.copy_from_slice(buffer);
                        }

                        value
                    }
                }
            }
        }
        _ => { panic!("#[derive(BlfChunk)] is only defined for structs!")}
    }.into()
}