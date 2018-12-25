// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Use [`serde`] for serializing and deserializing.
//!
//! [`serde`]: https://crates.io/crates/serde

use crate::fixed_uint::UintConstructor;
use quote::quote;

impl UintConstructor {
    pub fn with_serde(&self) {
        self.with_serde_defun_pub();
    }

    fn with_serde_defun_pub(&self) {
        let name = &self.ts.name;
        let bytes_size = &self.ts.bytes_size;
        let part = quote!(
            #[cfg(feature = "support_serde")]
            impl serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    fn hex_to(src: &[u8], dst: &mut [u8]) -> usize {
                        fn hex(byte: u8) -> u8 {
                            static TABLE: &[u8] = b"0123456789abcdef";
                            TABLE[byte as usize]
                        }

                        let mut len = src.len() * 2;
                        let mut idx = 0;
                        let first_nibble = src[0] >> 4;
                        if first_nibble != 0 {
                            dst[idx] = hex(src[0] >> 4);
                            idx += 1;
                        } else {
                            len -= 1;
                        }
                        dst[idx] = hex(src[0] & 0xf);
                        idx += 1;

                        for (byte, slots) in src.iter().skip(1).zip(dst[idx..].chunks_mut(2)) {
                            slots[0] = hex(*byte >> 4);
                            slots[1] = hex(*byte & 0xf);
                        }
                        len
                    }

                    let mut bytes = [0u8; #bytes_size];
                    let mut dst = [0u8; #bytes_size * 2 + 2];
                    dst[0] = b'0';
                    dst[1] = b'x';
                    self.into_big_endian(&mut bytes)
                        .map_err(|e| serde::ser::Error::custom(&format!("{}", e)))?;

                    let non_zero = bytes.iter().position(|&b| b != 0);

                    if let Some(non_zero_idx) = non_zero {
                        let bytes = &bytes[non_zero_idx..];
                        let len = hex_to(bytes, &mut dst[2..]);
                        serializer.serialize_str(unsafe {
                            ::std::str::from_utf8_unchecked(&dst[..(len + 2)])
                        })
                    } else {
                        serializer.serialize_str("0x0")
                    }
                }
            }

            #[cfg(feature = "support_serde")]
            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'b> serde::de::Visitor<'b> for Visitor {
                        type Value = #name;

                        fn expecting(
                            &self,
                            formatter: &mut ::std::fmt::Formatter,
                        ) -> ::std::fmt::Result {
                            write!(
                                formatter,
                                "a 0x-prefixed hex string with at most {} digits",
                                #bytes_size * 2
                            )
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            if v.len() < 2 || &v[0..2] != "0x" {
                                return Err(E::custom(format_args!(
                                    "invalid format, expected {}",
                                    &self as &serde::de::Expected
                                )));
                            }

                            if v.len() > #bytes_size * 2 + 2 {
                                return Err(E::invalid_length(v.len() - 2, &self));
                            }

                            #name::from_hex_str(&v[2..]).map_err(|e| {
                                E::custom(format_args!(
                                    "invalid hex bytes: {:?}, expected {}",
                                    e, &self as &serde::de::Expected
                                ))
                            })
                        }

                        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            self.visit_str(&v)
                        }
                    }
                    deserializer.deserialize_str(Visitor)
                }
            }
        );
        self.implt(part);
    }
}
