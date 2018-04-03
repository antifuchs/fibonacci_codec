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
//! ## Examples
//! ``` rust
//! # use fibonacci_codec::Encode;
//! let numbers: Vec<u16> = vec![1, 50, 3003];
//! let encoded = &numbers.fib_encode();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//! ## References:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_coding)
//! * [Fraenkel, Aviezri S.; Klein, Shmuel T. (1996). "Robust universal complete codes for transmission and compression"](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.37.3064)

extern crate bit_vec;
extern crate num;

use num::CheckedSub;
use std::fmt::Debug;
use bit_vec::BitVec;

mod macros;
mod tables;

/// Allows encoding unsigned integers (> 0) with fibonacci coding.
pub trait Encode
where
    Self: Sized,
{
    /// Fibonacci-encodes an integer into a bit vector and returns the resulting vector.
    fn fib_encode(self) -> BitVec {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec);
        vec
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    fn fib_encode_mut(self, vec: &mut BitVec);
}

#[inline]
pub(crate) fn bits_from_table<T>(n: T, table: &'static [T], result: &mut BitVec)
where
    T: CheckedSub + PartialOrd + Debug + Copy,
{
    let mut current = n;
    let split_pos = table
        .iter()
        .rposition(|elt| *elt <= n)
        .unwrap_or_else(|| panic!("BUG: Could not find a fibonacci number less than {:?}", n));

    let mut i = result.len() + split_pos + 1;
    result.grow(split_pos + 2, false);
    result.set(i, true);
    for elt in table.split_at(split_pos + 1).0.iter().rev() {
        i -= 1;
        result.set(
            i,
            if elt <= &current {
                let next = current.checked_sub(elt).unwrap_or_else(|| {
                    panic!(
                        "BUG: could not subtract {:?} from {:?} to encode {:?}",
                        elt, current, n
                    )
                });
                current = next;
                true
            } else {
                false
            },
        );
    }
}
