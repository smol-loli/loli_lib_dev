#![allow(unused_imports)]
#![allow(dead_code)]


use crypto::ripemd160::Ripemd160;
use crypto::sha3::Sha3;
use crypto::whirlpool::Whirlpool;

// Module containing generic functions at some point (currently none)
pub mod lstd {
    /// Just adds together two numbers lol
    /// Thanks for mrMiiao for making it polymorphic!
    ///
    pub fn add<T: core::ops::Add<Output = T>>(left: T, right: T) -> T {
        left + right
    }
}

/// Module containing all implemented hashing methods
///
/// # Notice
/// As of now all implemented hashing methods aren't made by me (as this is just a fun project to learn the basics of Rust). All hashing methods were used from the rust-crypto crate made by DaGenix. All credit goes to him!
///
/// DaGenix (<https://crates.io/users/DaGenix>)
///
/// rust-crypto (<https://crates.io/crates/rust-crypto>)
///
/// # Available hashing methods
/// SHA3-512
///
/// Whirlpool
///
/// RIPEMD-160
///

pub mod hashing {
    use crypto::digest::Digest;
    use crypto::ripemd160::Ripemd160;
    use crypto::sha3::Sha3;
    use crypto::whirlpool::Whirlpool;

    /// Hash a &str input (gets converted to bytes before being hashed) using SHA3-512
    ///
    /// # Example
    /// ```
    /// use loli_lib_dev::hashing::hash_sha3_512;
    ///
    /// let str_to_hash = "12345abcde";
    /// let hashed = hash_sha3_512(str_to_hash);
    /// assert_eq!(str_to_hash, "4a223fa925a250eae6701f132f153ec0eeb869bc4a2aec386e2c929527290d3553fd1f9bbd41be9039f77a5a902548991c2976c30dca84df9ec8427ad4aa4949");
    ///
    /// ```
    ///
    pub fn hash_sha3_512(hashing_input: &str) -> String {

        // Convert input string to bytes
        let b_input = hashing_input.as_bytes();

        // Initialize Hasher
        let mut hasher = Sha3::sha3_512();
        hasher.input(b_input);
        let hex_output = hasher.result_str();

        return hex_output;
    }

    /// Hash a &str input (gets converted to bytes before being hashed) using Whirlpool
    ///
    /// # Example
    /// ```
    /// use loli_lib_dev::hashing::hash_whirlpool;
    ///
    /// let str_to_hash = "12345abcde";
    /// let hashed = hash_whirlpool(str_to_hash);
    /// assert_eq!(str_to_hash, "c9b9baeb725f00211f3807752fcfad344927be6ac1588996518193f6c003946118d1bd2dd2f67ba06f211eee26fe2fc162d4b4fe4748b9c9beed81fe865cc409");
    ///
    /// ```
    ///
    pub fn hash_whirlpool(hashing_input: &str) -> String {

        // Convert input string to bytes
        let b_input = hashing_input.as_bytes();

        // Initialize Hasher
        let mut hasher = Whirlpool::new();
        hasher.input(b_input);
        let hex_output = hasher.result_str();

        return hex_output;
    }

    /// Hash a &str input (gets converted to bytes before being hashed) using RIPEMD-160
    ///
    /// # Example
    /// ```
    /// use loli_lib_dev::hashing::hash_ripemd160;
    ///
    /// let str_to_hash = "12345abcde";
    /// let hashed = hash_ripemd160(str_to_hash);
    /// assert_eq!(str_to_hash, "7a6ec22a4902a79d6635f86445980ef4c13254c5");
    ///
    /// ```
    ///
    pub fn hash_ripemd160(hashing_input: &str) -> String {

        // Convert input string to bytes
        let b_input = hashing_input.as_bytes();

        // Initialize Hasher
        let mut hasher = Ripemd160::new();
        hasher.input(b_input);
        let hex_output = hasher.result_str();

        return hex_output;
    }
}


#[cfg(test)]// 
mod tests {
    use crate::hashing::{hash_ripemd160, hash_sha3_512, hash_whirlpool};
    use crate::lstd::{add};
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_hash_sha3_512() {
        let result = hash_sha3_512("12345abcde");
        assert_eq!(result, "4a223fa925a250eae6701f132f153ec0eeb869bc4a2aec386e2c929527290d3553fd1f9bbd41be9039f77a5a902548991c2976c30dca84df9ec8427ad4aa4949");
    }

    #[test]
    fn it_works_hash_whirlpool() {
        let result = hash_whirlpool("12345abcde");
        assert_eq!(result, "c9b9baeb725f00211f3807752fcfad344927be6ac1588996518193f6c003946118d1bd2dd2f67ba06f211eee26fe2fc162d4b4fe4748b9c9beed81fe865cc409");
    }

    #[test]
    fn it_works_hash_ripemd160() {
        let result = hash_ripemd160("12345abcde");
        assert_eq!(result, "7a6ec22a4902a79d6635f86445980ef4c13254c5");
    }
}
