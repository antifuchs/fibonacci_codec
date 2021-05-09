use bit_vec::BitVec;
use failure::Fail;
use num::CheckedSub;
use std::fmt::{Debug, Display, Error, Formatter};

/// Indicates that encoding a number failed.
#[derive(Debug, PartialEq)]
pub enum EncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    /// Indicates an attempt to encode the number `0`, which can't be
    /// represented in fibonacci encoding.
    ValueTooSmall(T),

    /// A bug in fibonacci_codec in which encoding the contained
    /// number resulted in an attempt to subtract a larger fibonacci
    /// number than the number to encode.
    Underflow(T),
}

impl<T> Display for EncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            EncodeError::ValueTooSmall(ref n) => write!(f, "value {:?} is too small to encode", n),
            EncodeError::Underflow(ref n) => {
                write!(f, "underflow occurred, could not encode {:?}", n)
            }
        }
    }
}

impl<T> Fail for EncodeError<T> where T: Debug + Send + Sync + 'static {}

/// Indicates that encoding an iterator failed at an element.
#[derive(Debug, PartialEq)]
pub struct ElementEncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    /// The element where encoding of iterator elements failed.
    pub index: usize,

    /// The error encountered when encoding an element on the iterator.
    pub error: EncodeError<T>,
}

impl<T> Fail for ElementEncodeError<T> where T: Debug + Send + Sync + 'static {}

impl<T> Display for ElementEncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "could not encode iterator element {:?}: {}",
            self.index, self.error
        )
    }
}

/// Allows encoding single primitive integers (> 0) using fibonacci
/// coding.
pub trait EncodeOne
where
    Self: Sized + Debug + Send + Sync,
{
    /// Fibonacci-encodes an integer into a bit vector and returns the
    /// resulting vector.
    /// # Errors
    /// Returns an error when attempting to encode 0.
    fn fib_encode(self) -> Result<BitVec, EncodeError<Self>> {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec)?;
        Ok(vec)
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    /// # Errors
    /// Returns an error when attempting to encode 0.
    fn fib_encode_mut(self, vec: &mut BitVec) -> Result<(), EncodeError<Self>>;
}

/// Allows encoding enumerations of unsigned integers (> 0) using
/// fibonacci coding.
///
/// This crate implements this trait for anything that is
/// `IntoIterator` with primitive unsigned integer elements.
///
/// ## A note about zero
/// The number `0` can't be encoded using fibonacci coding. If you
/// need to encode a zero, you can use `.map(|x| x+1)` before encoding
/// and invert this when decoding.
pub trait Encode<T>
where
    Self: Sized + Debug + Send + Sync,
    T: Debug + Send + Sync,
{
    /// Fibonacci-encodes an iterator of integers into bits and
    /// returns the resulting bit vector.
    fn fib_encode(self) -> Result<BitVec, ElementEncodeError<T>> {
        let mut vec = BitVec::default();
        self.fib_encode_mut(&mut vec)?;
        Ok(vec)
    }

    /// Fibonacci-encodes an iterator yielding integers onto the end
    /// of an existing bit vector, until the iterator is exhausted. It
    /// extends the bit vector by the number of bits required to hold
    /// the entire output.
    ///
    /// # Error handling
    /// When encountering an encoding error at any element,
    /// `fib_encode_mut` returns an error indicating at which element
    /// the error occurred. It leaves the previous, correctly-encoded
    /// values' bits in the result bit vector.
    fn fib_encode_mut(self, vec: &mut BitVec) -> Result<(), ElementEncodeError<T>>;
}

#[inline]
pub(crate) fn bits_from_table<T>(
    n: T,
    table: &'static [T],
    result: &mut BitVec,
) -> Result<(), EncodeError<T>>
where
    T: CheckedSub + PartialOrd + Ord + Debug + Copy + Send + Sync + 'static,
{
    let mut current = n;
    let split_pos = match table.binary_search(&n) {
        Ok(n) => n,
        Err(n) if n > 0 => n - 1,
        Err(_) => return Err(EncodeError::ValueTooSmall::<T>(n)),
    };

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
                    // life. However, let's clean up and return a
                    // reasonable error:
                    result.truncate(split_pos + 2);
                    return Err(EncodeError::Underflow(n));
                }
            };
            current = next;
            true
        };
        result.set(i, bit);
    }
    Ok(())
}
