use num::CheckedSub;
use std::fmt::{Debug, Display, Error, Formatter};
use bit_vec::BitVec;
use failure::Fail;

#[derive(Debug, PartialEq)]
pub enum EncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    ValueTooSmall(T),
    Underflow(T),
}

impl<T> Display for EncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            EncodeError::ValueTooSmall(n) => write!(f, "value {:?} is too small to encode", n),
            EncodeError::Underflow(n) => write!(f, "underflow occurred, could not encode {:?}", n),
        }
    }
}

impl<T> Fail for EncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
}

#[derive(Debug, PartialEq)]
pub struct SliceEncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    pub index: usize,
    pub error: EncodeError<T>,
}

impl<T> Fail for SliceEncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
}

impl<T> Display for SliceEncodeError<T>
where
    T: Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "could not encode slice element {:?}: {}",
            self.index, self.error
        )
    }
}

/// Allows encoding unsigned integers (> 0) with fibonacci coding.
pub trait Encode
where
    Self: Sized + Debug + Send + Sync,
{
    /// Fibonacci-encodes an integer into a bit vector and returns the resulting vector.
    fn fib_encode(self) -> Result<BitVec, EncodeError<Self>> {
        let mut vec = BitVec::default();
        try!(self.fib_encode_mut(&mut vec));
        Ok(vec)
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    /// # Panics
    /// When encoding zero.
    fn fib_encode_mut(self, vec: &mut BitVec) -> Result<(), EncodeError<Self>>;
}

/// Allows encoding slices of unsigned integers (> 0) with fibonacci coding.
pub trait EncodeSlice<T>
where
    Self: Sized + Debug + Send + Sync,
    T: Debug + Send + Sync,
{
    /// Fibonacci-encodes a slice of integers into a bit vector and
    /// returns the resulting vector of bits.
    fn fib_encode(self) -> Result<BitVec, SliceEncodeError<T>> {
        let mut vec = BitVec::default();
        try!(self.fib_encode_mut(&mut vec));
        Ok(vec)
    }

    /// Fibonacci-encodes an integer onto the end of an existing bit
    /// vector. It extends the bit vector by the numer of bits
    /// required to hold the output.
    fn fib_encode_mut(self, vec: &mut BitVec) -> Result<(), SliceEncodeError<T>>;
}

#[inline]
pub(crate) fn bits_from_table<T>(
    n: T,
    table: &'static [T],
    result: &mut BitVec,
) -> Result<(), EncodeError<T>>
where
    T: CheckedSub + PartialOrd + Debug + Copy + Send + Sync + 'static,
{
    let mut current = n;
    let split_pos = table
        .iter()
        .rposition(|elt| *elt <= n)
        .ok_or(EncodeError::ValueTooSmall::<T>(n))?;

    let mut i = result.len() + split_pos + 1;
    result.grow(split_pos + 2, false);
    result.set(i, true);
    for elt in table.split_at(split_pos + 1).0.iter().rev() {
        i -= 1;
        result.set(
            i,
            if elt <= &current {
                let next = current.checked_sub(elt).ok_or(EncodeError::Underflow(n))?;
                current = next;
                true
            } else {
                false
            },
        );
    }
    Ok(())
}
