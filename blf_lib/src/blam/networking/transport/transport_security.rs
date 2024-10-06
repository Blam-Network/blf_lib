// This module is based on ManagedDonkey's transport_security module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/networking/transport/transport_security.cpp
#![allow(dead_code)]

pub struct s_transport_unique_identifier
{
    part0: u32,
    part4: [u16; 2],
}

pub struct s_transport_secure_address {
    identifier: s_transport_unique_identifier,
    part8: [u8; 8],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizeof_s_transport_unique_identifier() {
        assert_eq!(size_of::<s_transport_unique_identifier>(), 0x8);
    }

    #[test]
    fn sizeof_s_transport_secure_address() {
        assert_eq!(size_of::<s_transport_secure_address>(), 0x10);
    }
}
