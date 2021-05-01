//! Traits and functions for encoding and decoding (slices of)
//! positive integers using Fibonacci Coding.
//!
//! Fibonacci coding is a method for representing a continuous stream
//! of (positive) integers using a variable number of bits -- e.g.,
//! instead of taking up 2*32 bits for storing two `u32` words, these
//! words will take up however many bits their fibonacci-encoded
//! representation takes up.
//!
//! Fibonacci coding yields better results for smaller numbers than
//! larger ones: The largest 32-bit words can take up to 47 bits, but
//! to encode the number `1`, you only need 2 bits.
//!
//! Fibonacci coding is self-synchronizing: If a single bit is
//! altered, a single number could be decoded as two (or two numbers
//! decoded as one), but the remaining numbers will be read correctly.
//!
//! ## Value range
//!
//! Fibonacci coding can represent any number that can be expressed as
//! addition of one or more Fibonacci numbers, so any integer greater
//! than 1, up to the range of the given machine integer type. This
//! means that the integer zero can not be encoded.
//!
//! If you should need to encode `0`, it is advisable to encode
//! numbers incremented by one (preventing overflow by upgrading to
//! the next-biggest integer type, or by not encoding the maximum
//! value), and to subtract one from the decoded result.
//! .
//!
//! # Examples
//!
//! ## Encoding a slice of numbers:
//! ``` rust
//! use fibonacci_codec::Encode;
//!
//! let numbers: Vec<u16> = vec![1, 50, 3003];
//! let encoded = &numbers.fib_encode().unwrap();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//!
//! ## Encoding the value zero:
//! ``` rust
//! use fibonacci_codec::Encode;
//!
//! let numbers: Vec<u16> = vec![0, 49, 3002];
//! let adjusted: Vec<u32> = numbers.iter().map(|n| *n as u32 + 1).collect();
//! let encoded = &adjusted.fib_encode().unwrap();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//!
//! # References:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_coding)
//! * [Fraenkel, Aviezri S.; Klein, Shmuel T. (1996). "Robust universal complete codes for transmission and compression"](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.37.3064)

#[macro_use]
extern crate failure_derive;

// Macros need to be defined first:
mod macros;

mod decode;
mod encode;
pub mod int {
    include!(concat!(env!("OUT_DIR"), "/int.rs"));
}

pub use crate::decode::DecodeError;
pub use crate::encode::*;
pub use crate::int::funcs::*;
