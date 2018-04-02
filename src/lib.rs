extern crate num;

use num::{CheckedSub, Integer, Unsigned, Zero};
use std::fmt::Debug;

mod tables;

pub(crate) trait EncodeImpl
where
    Self: Integer + Unsigned + CheckedSub + Zero + Sized + Clone + Debug,
{
    type TableType;
    const TABLE: Self::TableType;

    /// Returns
    fn fibonacci_encode<R: Iterator<Item = bool> + Sized>(n: Self) -> R {
        panic!("NYI: {:?}", n);
    }
}

#[inline]
fn indexes_from_table<T>(n: T, table: &'static [T]) -> (T, usize)
where
    T: PartialOrd + Debug + Copy,
{
    let posn = table.iter().rposition(|elt| n >= *elt);
    if let Some(i) = posn {
        (table[i], i)
    } else {
        panic!("Could not find {:?} in the table for u8", n)
    }
}

impl EncodeImpl for u8 {
    type TableType = &'static [u8];
    const TABLE: Self::TableType = tables::FIB_TABLE_U8;
}

#[test]
fn test_factor_smol() {
    // assert_eq!(vec![8, 6, 1], fib_position_encode(78 as u8)); // 78 = 55 + 21 + 2
    // assert_eq!(vec![9, 4, 0], fib_position_encode(98 as u8)); // 98 = 89 + 8 + 1
    // assert_eq!(vec![6, 4, 2, 0], fib_position_encode(33 as u8)); // 33 = 21 + 8 + 3 + 1
}
