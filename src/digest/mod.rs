//! Cryptographic hash functions primitives
//!
//! Via [Wikipedia](https://en.wikipedia.org/wiki/Cryptographic_hash_function):
//!
//! > The ideal cryptographic hash function has four main properties:
//! >
//! > - it is easy to compute the hash value for any given message
//! > - it is infeasible to generate a message from its hash
//! > - it is infeasible to modify a message without changing the hash
//! > - it is infeasible to find two different messages with the same hash.
//!
//! **WARNING**: If you want to use one of this functions as password hash then
//! you are evil human being and I really hope that I'm not using any of your services.

pub trait Digest {
    /// Update digest with data.
    fn update<T>(&mut self, input: T) where T: AsRef<[u8]>;

    /// Output size in bits
    fn output_bits() -> usize;
    /// Output size in bytes
    fn output_bytes() -> usize {
        (Self::output_bits() + 7) / 8
    }
    fn block_size() -> usize;

    /// Write resulting hash into `output`.
    ///
    /// `output` should be big enough to contain whole output.
    ///
    /// ## Panics
    ///
    /// If output length is less than `Digest::output_bytes`.
    fn result<T>(self, output: T) where T: AsMut<[u8]>;
}

#[cfg(feature = "md4")]pub mod md4;
#[cfg(feature = "md5")]pub mod md5;
#[cfg(feature = "ripemd")]pub mod ripemd;
#[cfg(feature = "sha1")]pub mod sha1;
#[cfg(feature = "sha2")]pub mod sha2;
#[cfg(feature = "sha3")]pub mod sha3;
#[cfg(feature = "tiger")]pub mod tiger;
#[cfg(feature = "whirlpool")]pub mod whirlpool;

#[cfg(test)]mod test;
