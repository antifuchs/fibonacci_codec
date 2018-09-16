use bit_vec::BitVec;
use num::CheckedSub;
use std::fmt::Debug;

/// Allows encoding single non-zero unsigned integers into bit vectors
/// using fibonacci coding.
pub trait EncodeOne
where
    Self: Sized + Debug + Send + Sync,
    Self::Base: Sized + Debug + Send + Sync,
{
    /// The base numeric type that this trait encodes. E.g., for
    /// `NonZeroU8`, this is `u8`.
    type Base;

    /// Fibonacci-encodes an integer into a bit vector and returns the
    /// resulting vector.
    fn fib_encode(self) -> BitVec {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec);
        vec
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    /// # Errors
    /// Returns an error when attempting to encode 0.
    fn fib_encode_mut(self, vec: &mut BitVec);
}

/// Allows encoding enumerations of non-zero unsigned integers using
/// fibonacci coding.
///
/// This crate implements this trait for anything that is
/// `IntoIterator` with non-zero unsigned integer elements, including
/// other iterators.
pub trait Encode<S>
where
    Self: Sized,
    S: Debug,
{
    /// Fibonacci-encodes an iterator of integers into bits and
    /// returns the resulting bit vector.
    fn fib_encode(self) -> BitVec {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec);
        vec
    }

    /// Fibonacci-encodes an iterator yielding integers onto the end
    /// of an existing bit vector, until the iterator is exhausted. It
    /// extends the bit vector by the number of bits required to hold
    /// the entire output.
    fn fib_encode_mut(self, vec: &mut BitVec);
}

fn fibonacci_bit_value<T>(table: &'static [T], n: T) -> usize
where
    T: CheckedSub + Ord + 'static,
{
    match table.binary_search(&n) {
        Ok(pos) => pos,
        Err(insert_pos) => {
            if insert_pos == 0 {
                panic!("encountered a zero trying to fibonacci-encode a value");
            }
            insert_pos - 1
        }
    }
}

#[inline]
pub(crate) fn bits_from_table<T>(n: T, table: &'static [T], result: &mut BitVec)
where
    T: CheckedSub + Ord + Debug + Copy + 'static,
{
    let mut current = n;
    let split_pos = fibonacci_bit_value(table, n);

    let mut i = result.len() + split_pos + 1;
    result.grow(split_pos + 2, false);
    result.set(i, true);
    for elt in table.split_at(split_pos + 1).0.iter().rev() {
        i -= 1;
        let bit = if elt > &current {
            false
        } else {
            let next = match current.checked_sub(elt) {
                Some(next) => next,
                None => {
                    // We encountered an underflow. This is a bug, and
                    // I have no idea how it could even occur in real
                    // life. A panic seems appropriate:
                    panic!("underflow in substraction while encoding {:?}", n);
                }
            };
            current = next;
            true
        };
        result.set(i, bit);
    }
}
