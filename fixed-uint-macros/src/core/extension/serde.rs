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

use core::constructor::UintConstructor;

impl UintConstructor {
    pub fn with_serde(&self) {
        self.with_serde_defun_pub();
    }

    fn with_serde_defun_pub(&self) {
        let name = &self.ts.name;
        let bytes_size = &self.ts.bytes_size;
        let part = quote!(
            impl serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                    let mut bytes = [0u8; #bytes_size];
                    let mut dst = [0u8; #bytes_size * 2 + 2];
                    dst[0] = b'0';
                    dst[1] = b'x';
                    self.into_big_endian(&mut bytes);
                    let non_zero = bytes.iter().position(|&b| b != 0);

                    if let Some(non_zero_idx) = non_zero {
                        let bytes = &bytes[non_zero_idx..];
                        faster_hex::hex_to(bytes, &mut dst[2..]);
                        serializer.serialize_str(unsafe {::std::str::from_utf8_unchecked(&dst[..(bytes.len() * 2 + 2)])})
                    } else {
                        serializer.serialize_str("0x0")
                    }
                }
            }


            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    let mut dst = [0u8; #bytes_size];

                    fn val<E>(ch: u8, idx: usize) -> Result<u8, E> where E: serde::de::Error {
                        match ch {
                            b'A'...b'F' => Ok(ch - b'A' + 10),
                            b'a'...b'f' => Ok(ch - b'a' + 10),
                            b'0'...b'9' => Ok(ch - b'0'),
                            _ => {
                                Err(E::custom(&format!("invalid hex character: {}, at {}", ch, idx)))
                            }
                        }
                    }

                    struct Visitor<'a> {
                        dst: &'a mut [u8],
                    }

                    impl<'a, 'b> serde::de::Visitor<'b> for Visitor<'a> {
                        type Value = ();

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            write!(formatter, "a 0x-prefixed hex string with {}", self.dst.len())
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                            if v.len() < 2  || &v[0..2] != "0x" {
                                return Err(E::custom("Invalid format"));
                            }

                            if v.len() > self.dst.len() * 2 + 2 {
                                return Err(E::invalid_length(v.len() - 2, &self))
                            }

                            for (idx, pair) in v.as_bytes()[2..].chunks(2).enumerate() {
                                self.dst[idx] = val(pair[0], 2 * idx)? << 4
                                    | val(pair[1], 2 * idx + 1)?;
                            }
                            Ok(())
                        }

                        fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
                            self.visit_str(&v)
                        }
                    }
                    deserializer.deserialize_str(Visitor { dst: &mut dst })?;


                    #name::from_big_endian(&dst).map_err(|e| serde::de::Error::custom(&format!("invalid hex bytes: {:?}", e)))
                }
            }
        );
        self.implt(part);
    }
}
