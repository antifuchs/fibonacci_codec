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
//! means that the integer zero can not be encoded. This crate uses
//! the `NonZeroU*` types as inputs. I recommend using the traits in
//! [nonzero_ext](https://crates.io/crates/nonzero_ext) to help with
//! conversions from primitive types.
//!
//! # Examples
//!
//! ## Encoding a slice of numbers:
//! ``` rust
//! # use std::num::NonZeroU16;
//! use fibonacci_codec::Encode;
//!
//! let numbers: Vec<u16> = vec![1, 50, 3003];
//! let numbers: Vec<NonZeroU16> = numbers.iter().filter_map(|n| NonZeroU16::new(*n)).collect();
//! let encoded = &numbers.fib_encode().unwrap();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//!
//! # References:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_coding)
//! * [Fraenkel, Aviezri S.; Klein, Shmuel T. (1996). "Robust universal complete codes for transmission and compression"](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.37.3064)

extern crate bit_vec;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate num;

// Macros need to be defined first:
mod macros;

mod decode;
mod encode;
pub mod int;

pub use decode::DecodeError;
pub use encode::*;
pub use int::funcs::*;
