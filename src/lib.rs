extern crate bit_vec;
extern crate num;

use num::CheckedSub;
use std::fmt::Debug;
use bit_vec::BitVec;

mod tables;

/// Allows encoding unsigned integers with fibonacci coding.
///
///
pub trait FibEncode {
    /// Fibonacci-encodes an integer into a bit vector.
    fn fib_encode(self) -> BitVec;
}

#[inline]
pub(crate) fn bits_from_table<T>(n: T, table: &'static [T]) -> BitVec
where
    T: CheckedSub + PartialOrd + Debug + Copy,
{
    let mut current = n;
    let split_pos = table
        .iter()
        .rposition(|elt| *elt <= n)
        .unwrap_or_else(|| panic!("BUG: Could not find a fibonacci number less than {:?}", n));
    let mut result = BitVec::from_elem(split_pos + 2, false);

    result.set(split_pos + 1, true);
    let mut i = split_pos + 1;
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
    result
}
