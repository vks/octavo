//! Highly modular & configurable hash & crypto library written in Rust
//!
//! # About
//!
//! Octavo is Rust implementation of cryptographic primitives and [Transport Layer Security][tls].
//! Our goal is to provide safe, fast, full-featured and configurable cryptographic library
//! for modern world.
//!
//! Except of Rust API we want expose also C API for usage with other languages and tools. Probably
//! there will also land thin [OpenSSL][openssl] compatibility cascade for use with existing code.
//!
//! [tls]: https://en.wikipedia.org/wiki/Transport_Layer_Security "Transport Layer Security"
//!
//! ## Legalities
//!
//! Please remember that export/import and/or use of strong cryptography software, providing
//! cryptography hooks, or even just communicating technical details about cryptography software is
//! illegal in some parts of the world. So when you import this package to your country,
//! re-distribute it from there or even just email technical suggestions or even source patches to
//! the authors or other people you are strongly advised to pay close attention to any laws or
//! regulations which apply to you. The authors of Octavo are not liable for any violations you
//! make here. So be careful, it is your responsibility.[^authors]
//!
//! [^authors]: Text of this paragraph is copied from [OpenSSL website][openssl].
//! [openssl]: http://www.openssl.org/ "OpenSSL - Cryptography and SSL/TLS Toolkit"
//!
//! # Examples
//!
//! Calculate SHA-512 sum:
//!
//! ```rust
//! extern crate octavo;
//!
//! use octavo::digest::Digest;
//! use octavo::digest::sha2::Sha512;
//!
//! fn main() {
//!     let data = "Hello World!";
//!     let result = {
//!         let mut sha = Sha512::default();
//!         sha.update(data);
//!
//!         let mut result = vec![0; Sha512::output_bytes()];
//!
//!         sha.result(&mut result[..]);
//!
//!         result
//!     };
//!
//!     for byte in result {
//!         print!("{:2x}", byte);
//!     }
//!     println!(" {}", data);
//! }
//! ```

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(not(feature = "lints"), allow(unknown_lints))]

#![deny(unreachable_code, while_true)]
#![cfg_attr(not(test), deny(trivial_casts))]
#![warn(unused_qualifications, missing_docs)]

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate openssl;

extern crate byteorder;
#[cfg(feature = "num")] extern crate num;
#[cfg(feature = "rand")] extern crate rand;

pub mod crypto;
pub mod digest;
pub mod mac;
pub mod kdf;

mod utils;
