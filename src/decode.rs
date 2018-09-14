use num::{CheckedAdd, One, Zero};
use std::fmt::Debug;

/// Returned if decoding a number fails. Usually indicates an overflow
/// of the number being decoded.
#[derive(Fail, Debug, PartialEq)]
pub enum DecodeError {
    /// Indicates that the decoded number depends on a fibonacci
    /// sequence element that doesn't fit the return type.
    #[fail(
        display = "fibonacci sequence element would overflow result type at bit position {:?}",
        bit_pos
    )]
    FibonacciElementOverflow { bit_pos: usize },

    /// Indicates that the decoded number does not fit into the given
    /// result type. This more than anything indicates that a bit flip
    /// has occurred, and the next number can't be trusted either.
    #[fail(
        display = "constructing number would overflow at bit position {:?}",
        bit_pos
    )]
    ConstructionOverflow { bit_pos: usize },
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

fn consume_overflow<I>(elt: bool, iterator: &mut I)
where
    I: Iterator<Item = bool>,
{
    let mut last = elt;
    for elt in iterator {
        if is_terminator(elt, last) {
            break;
        }
        last = elt;
    }
}

// Can't write the loop as `for elt in iterator` because we use the
// iterator again later:
#[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
#[inline]
pub(crate) fn decode_from<I, T>(
    iterator: &mut I,
    table: &'static [T],
) -> Option<Result<T, DecodeError>>
where
    I: Iterator<Item = bool>,
    T: CheckedAdd + PartialOrd + Debug + Copy + Zero + One,
{
    let mut i = 0;
    let mut accumulator: T = T::zero();
    let mut last = false;
    while let Some(elt) = iterator.next() {
        if is_terminator(elt, last) {
            return Some(Ok(accumulator));
        }

        if let Some(fib) = table.get(i) {
            let digit = multiplier::<T>(elt) * *fib;
            if let Some(new_acc) = accumulator.checked_add(&digit) {
                accumulator = new_acc;
            } else {
                consume_overflow(elt, iterator);
                return Some(Err(DecodeError::ConstructionOverflow { bit_pos: i }));
            }
        } else {
            consume_overflow(elt, iterator);
            return Some(Err(DecodeError::FibonacciElementOverflow { bit_pos: i }));
        }
        i += 1;
        last = elt;
    }
    // Done with this stream:
    None
}
