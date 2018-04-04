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
//! # use fibonacci_codec::Encode;
//! let numbers: Vec<u16> = vec![1, 50, 3003];
//! let encoded = &numbers.fib_encode();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//!
//! ## Encoding the value zero:
//! ``` rust
//! # use fibonacci_codec::Encode;
//! let numbers: Vec<u16> = vec![0, 49, 3002];
//! let adjusted: Vec<u32> = numbers.iter().map(|n| *n as u32 + 1).collect();
//! let encoded = &adjusted.fib_encode();
//! // code words: "11" (1), "001001011" (50), "000010010000100011" (3003)
//! // These encoded words take up 4 bytes instead of 6 (3*16 bits)!
//! assert_eq!(encoded.to_bytes(), [0b11001001, 0b01100001, 0b00100001, 0b00011000]);
//! ```
//!
//! # References:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Fibonacci_coding)
//! * [Fraenkel, Aviezri S.; Klein, Shmuel T. (1996). "Robust universal complete codes for transmission and compression"](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.37.3064)

extern crate bit_vec;
extern crate num;

use num::{CheckedAdd, CheckedSub, One, Zero};
use std::fmt::Debug;
use bit_vec::BitVec;
use std::iter::IntoIterator;
use std::marker::PhantomData;

mod macros;
mod tables;

/// Allows encoding unsigned integers (> 0) with fibonacci coding.
pub trait Encode
where
    Self: Sized,
{
    /// Fibonacci-encodes an integer into a bit vector and returns the resulting vector.
    /// # Panics
    /// When encoding zero.
    fn fib_encode(self) -> BitVec {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec);
        vec
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    /// # Panics
    /// When encoding zero.
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
fn multiplier<T>(bit: bool) -> T
where
    T: Zero + One,
{
    if bit {
        T::one()
    } else {
        T::zero()
    }
}

fn is_terminator(bit: bool, last: bool) -> bool {
    bit && last
}

/// An iterator that decodes Fibonacci-encoded bits into numbers.
pub struct Decoder<I, Result> {
    orig: I,
    _phantom: PhantomData<Result>,
}

#[inline]
pub(crate) fn decode_from<I, T>(iterator: &mut I, table: &'static [T]) -> Option<T>
where
    I: Iterator<Item = bool>,
    T: CheckedAdd + PartialOrd + Debug + Copy + Zero + One,
{
    let mut i = 0;
    let mut accumulator: T = T::zero();
    let mut last = false;
    while let Some(elt) = iterator.next() {
        if is_terminator(elt, last) {
            return Some(accumulator);
        }
        let digit = multiplier::<T>(elt) * table[i];
        if let Some(new_acc) = accumulator.checked_add(&digit) {
            accumulator = new_acc;
        } else {
            // overflow - return the faulty number.
            return Some(accumulator);
        }
        i += 1;
        last = elt;
    }
    // Done with this stream:
    None
}

pub fn decode_fib<Result, T, I>(collection: T) -> Decoder<I, Result>
where
    T: IntoIterator<Item = bool, IntoIter = I>,
    I: Iterator<Item = bool>,
    Result: CheckedAdd + PartialOrd + Debug + Copy + Zero + One,
{
    Decoder {
        orig: collection.into_iter(),
        _phantom: PhantomData,
    }
}
