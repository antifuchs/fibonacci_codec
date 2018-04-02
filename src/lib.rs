extern crate bit_vec;
extern crate num;

use num::CheckedSub;
use std::fmt::Debug;
use bit_vec::BitVec;

mod tables;

/// Allows encoding unsigned integers with fibonacci coding.
///
///
pub trait FibEncode
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

impl<'a> FibEncode for &'a [u8] {
    fn fib_encode_mut(self, vec: &mut BitVec) {
        for elt in self.iter() {
            elt.fib_encode_mut(vec);
        }
    }
}

impl<'a> FibEncode for &'a [u16] {
    fn fib_encode_mut(self, vec: &mut BitVec) {
        for elt in self.iter() {
            elt.fib_encode_mut(vec);
        }
    }
}

impl<'a> FibEncode for &'a [u32] {
    fn fib_encode_mut(self, vec: &mut BitVec) {
        for elt in self.iter() {
            elt.fib_encode_mut(vec);
        }
    }
}

impl<'a> FibEncode for &'a [u64] {
    fn fib_encode_mut(self, vec: &mut BitVec) {
        for elt in self.iter() {
            elt.fib_encode_mut(vec);
        }
    }
}
