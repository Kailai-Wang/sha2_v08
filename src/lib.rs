#![cfg_attr(not(feature = "std"), no_std)]

extern crate sha2;
pub use sha2::{Digest, Sha256, Sha512};
