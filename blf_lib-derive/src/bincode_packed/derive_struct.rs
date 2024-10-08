use std::fmt::format;
use crate::bincode_packed::attribute::{ContainerAttributes, FieldAttributes};
use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;

pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}

impl DeriveStruct {
    pub fn generate_encode(self, generator: &mut Generator, alignment: usize) -> Result<()> {
        let crate_name = &self.attributes.crate_name;
        generator
            .impl_for(&format!("{}::Encode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) =
                    (self.attributes.encode_bounds.as_ref()).or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::Encode", crate_name))
                            .unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("encode")
            .with_generic_deps("__E", [format!("{}::enc::Encoder", crate_name)])
            .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
            .with_arg("encoder", "&mut __E")
            .with_return_type(format!(
                "core::result::Result<(), {}::error::EncodeError>",
                crate_name
            ))
            .body(|fn_body| {
                if let Some(fields) = self.fields.as_ref() {
                    for field in fields.names() {
                        let attributes = field
                            .attributes()
                            .get_attribute::<FieldAttributes>()?
                            .unwrap_or_default();

                        fn_body.push_parsed(format!(
                            "let packed_backing = self.{field};",
                        ))?;

                        fn_body.push_parsed(format!(
                            "let pad_size = ({alignment} - (std::mem::size_of_val(&packed_backing) % {alignment})) % {alignment};",
                        ))?;

                        // fn_body.push_parsed(format!(
                        //     "panic!(\"Backing Size {{}} Alignment {{}}, PAD {{pad_size}}\", std::mem::size_of_val(&packed_backing), {});",
                        //     alignment
                        // ))?;

                        if attributes.with_serde {
                            fn_body.push_parsed(format!(
                                "{0}::Encode::encode(&{0}::serde::Compat(&packed_backing), encoder)?;",
                                crate_name
                            ))?;
                        } else {
                            fn_body.push_parsed(format!(
                                "{}::Encode::encode(&packed_backing, encoder)?;",
                                crate_name
                            ))?;
                        }

                        fn_body.push_parsed(format!(
                            "for i in 0..pad_size {{ {}::Encode::encode(&[0 as u8], encoder)?; }}",
                            crate_name
                        ))?;
                    }
                }
                fn_body.push_parsed("Ok(())")?;
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator, alignment: usize) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode
        let crate_name = &self.attributes.crate_name;

        generator
            .impl_for(format!("{}::Decode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) = (self.attributes.decode_bounds.as_ref()).or(self.attributes.bounds.as_ref()) {
                    where_constraints.clear();
                    where_constraints.push_parsed_constraint(bounds).map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints.push_constraint(g, format!("{}::Decode", crate_name)).unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("decode")
            .with_generic_deps("__D", [format!("{}::de::Decoder", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!("core::result::Result<Self, {}::error::DecodeError>", crate_name))
            .body(|fn_body| {
                fn_body.push_parsed(format!(
                    "let m = core::mem::MaybeUninit::<Self>::uninit();\
                    const fn size_of_raw<T>(_: *const T) -> usize {{\
                        core::mem::size_of::<T>()\
                    }}"
                ));
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        // Fields
                        // {
                        //      a: bincode::Decode::decode(decoder)?,
                        //      b: bincode::Decode::decode(decoder)?,
                        //      ...
                        // }
                        if let Some(fields) = self.fields.as_ref() {
                            for field in fields.names() {
                                let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                // if attributes.with_serde {
                                //     struct_body
                                //         .push_parsed(format!(
                                //             "{1}: {{ \
                                //                 let p = unsafe {{
                                //                     core::ptr::addr_of!((*(&m as *const _ as *const $t)).{1})
                                //                 }}
                                //                 let pad_size = {alignment} - (size_of_raw(p) % {alignment}); \
                                //                 let value = (<{0}::serde::Compat<_> as {0}::Decode>::decode(decoder)?).0;\
                                //                 for i in 0..pad_size {{ (<{0}::serde::Compat<_> as {0}::Decode>::decode(decoder)?); }}\
                                //                 value\
                                //             }},",
                                //             crate_name,
                                //             field
                                //         ))?;
                                // } else {
                                    struct_body
                                        .push_parsed(format!(
                                            "{1}: {{\
                                                let p = unsafe {{
                                                    core::ptr::addr_of!((*(&m as *const _ as *const Self)).{1})
                                                }};
                                                let pad_size = ({alignment} - (size_of_raw(p) % {alignment})) % {alignment}; \
                                                let value = {0}::Decode::decode(decoder)?;\
                                                decoder.reader().consume(pad_size);\
                                                value\
                                            }},",
                                            crate_name,
                                            field
                                        ))?;
                                // }
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        self.generate_borrow_decode(generator)?;
        Ok(())
    }

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode
        let crate_name = self.attributes.crate_name;

        generator
            .impl_for_with_lifetimes(format!("{}::BorrowDecode", crate_name), ["__de"])
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) = (self.attributes.borrow_decode_bounds.as_ref()).or(self.attributes.bounds.as_ref()) {
                    where_constraints.clear();
                    where_constraints.push_parsed_constraint(bounds).map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints.push_constraint(g, format!("{}::de::BorrowDecode<'__de>", crate_name)).unwrap();
                    }
                    for lt in generics.iter_lifetimes() {
                        where_constraints.push_parsed_constraint(format!("'__de: '{}", lt.ident))?;
                    }
                }
                Ok(())
            })?
            .generate_fn("borrow_decode")
            .with_generic_deps("__D", [format!("{}::de::BorrowDecoder<'__de>", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!("core::result::Result<Self, {}::error::DecodeError>", crate_name))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        if let Some(fields) = self.fields.as_ref() {
                            for field in fields.names() {
                                let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                                if attributes.with_serde {
                                    struct_body
                                        .push_parsed(format!(
                                            "{1}: (<{0}::serde::BorrowCompat<_> as {0}::BorrowDecode>::borrow_decode(decoder)?).0,",
                                            crate_name,
                                            field
                                        ))?;
                                } else {
                                    struct_body
                                        .push_parsed(format!(
                                            "{1}: {0}::BorrowDecode::borrow_decode(decoder)?,",
                                            crate_name,
                                            field
                                        ))?;
                                }
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }
}