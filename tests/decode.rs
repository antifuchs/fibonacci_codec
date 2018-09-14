extern crate bit_vec;
extern crate fibonacci_codec;
extern crate nonzero_ext;
extern crate num;

// use bit_vec::BitVec;
use fibonacci_codec::{fib_decode_u8, DecodeError, Encode};
use nonzero_ext::NonZeroAble;
use std::iter::Iterator;

fn a_vec<I>(v: Vec<I>) -> Vec<I::NonZero>
where
    I: NonZeroAble + Sized,
{
    v.into_iter()
        .filter_map(|n| n.as_nonzero())
        .collect::<Vec<I::NonZero>>()
}

macro_rules! test_for_roundtrip {
    ($name:ident, $input_type:ty, $decoder:path, $vec:expr) => {
        #[test]
        fn $name() {
            let input = a_vec::<$input_type>($vec);
            let encoded = input.iter().fib_encode().unwrap();
            let decoded: Vec<$input_type> = $decoder(encoded).map(|x| x.unwrap()).collect();
            assert_eq!(
                input
                    .into_iter()
                    .map(|n| n.get())
                    .collect::<Vec<$input_type>>(),
                decoded
            );
        }
    };
}

test_for_roundtrip!(
    test_roundtrip_u8,
    u8,
    fibonacci_codec::fib_decode_u8,
    vec![2, 14, 65]
);
test_for_roundtrip!(
    test_roundtrip_u16,
    u16,
    fibonacci_codec::fib_decode_u16,
    vec![2, 14, 65]
);
test_for_roundtrip!(
    test_roundtrip_u32,
    u32,
    fibonacci_codec::fib_decode_u32,
    vec![2, 14, 65]
);
test_for_roundtrip!(
    test_roundtrip_u64,
    u64,
    fibonacci_codec::fib_decode_u64,
    vec![2, 14, 65]
);
test_for_roundtrip!(
    test_roundtrip_u128,
    u128,
    fibonacci_codec::fib_decode_u128,
    vec![2, 14, 65]
);

#[test]
fn test_overflow_from_fib_elt() {
    let input = a_vec::<u64>(vec![23894089128374]);
    let encoded = input.clone().fib_encode().unwrap();
    let decoded: Vec<Result<u8, DecodeError>> = fib_decode_u8(encoded).collect();
    assert_eq!(
        Err(DecodeError::FibonacciElementOverflow { bit_pos: 12 }),
        decoded[0]
    );
}

#[test]
fn test_overflow_from_addition() {
    let input = a_vec::<u64>(vec![256]);
    let encoded = input.fib_encode().unwrap();
    let decoded: Vec<Result<u8, DecodeError>> = fib_decode_u8(encoded).collect();
    assert_eq!(
        Err(DecodeError::ConstructionOverflow { bit_pos: 11 }),
        decoded[0]
    );
}
