extern crate bit_vec;
extern crate fibonacci_codec;

use bit_vec::BitVec;
use fibonacci_codec::{Encode, EncodeOne};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

fn to_bits(slice: BitVec) -> Vec<u8> {
    slice.iter().map(|b| if b { 1 } else { 0 } as u8).collect()
}

#[test]
fn test_factor_wikipedia() {
    // Some of the examples on https://en.wikipedia.org/wiki/Fibonacci_coding:
    assert_eq!(
        vec![0, 1, 1],
        to_bits((NonZeroU8::new(2).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![1, 0, 1, 1],
        to_bits((NonZeroU8::new(4).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![0, 0, 1, 1],
        to_bits((NonZeroU8::new(3).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![0, 0, 0, 0, 0, 1, 1],
        to_bits((NonZeroU8::new(13).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![1, 0, 0, 0, 0, 1, 1],
        to_bits((NonZeroU8::new(14).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 1],
        to_bits((NonZeroU8::new(65).expect("must be nonzero")).fib_encode())
    );
    assert_eq!(
        vec![0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1],
        to_bits((NonZeroU16::new(3003).expect("must be nonzero")).fib_encode())
    );
}

fn check_slice_u8(numbers: Vec<u8>) -> Vec<u8> {
    let numbers: Vec<NonZeroU8> = numbers
        .into_iter()
        .map(|n| NonZeroU8::new(n).expect("Must be non-zero"))
        .collect();
    to_bits(numbers.fib_encode())
}
fn check_slice_u16(numbers: Vec<u16>) -> Vec<u8> {
    let numbers: Vec<NonZeroU16> = numbers
        .into_iter()
        .map(|n| NonZeroU16::new(n).expect("must be nonzero"))
        .collect();
    to_bits(numbers.fib_encode())
}
fn check_slice_u32(numbers: Vec<u32>) -> Vec<u8> {
    let numbers: Vec<NonZeroU32> = numbers
        .into_iter()
        .map(|n| NonZeroU32::new(n).expect("must be nonzero"))
        .collect();
    to_bits(numbers.fib_encode())
}
fn check_slice_u64(numbers: Vec<u64>) -> Vec<u8> {
    let numbers: Vec<NonZeroU64> = numbers
        .into_iter()
        .map(|n| NonZeroU64::new(n).expect("must be nonzero"))
        .collect();
    to_bits(numbers.fib_encode())
}

#[test]
fn test_slices_singles() {
    assert_eq!(vec![0, 1, 1], check_slice_u8(vec![2]));
    assert_eq!(vec![0, 1, 1], check_slice_u16(vec![2]));
    assert_eq!(vec![0, 1, 1], check_slice_u32(vec![2]));
    assert_eq!(vec![0, 1, 1], check_slice_u64(vec![2]));
}

#[test]
fn test_slices_multi() {
    assert_eq!(vec![1, 1, 1, 1], check_slice_u8(vec![1, 1]));
    assert_eq!(
        vec![
            0, 1, 1, // 1
            1, 0, 0, 0, 0, 1, 1, // 14
            0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u8(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u16(vec![1, 1]));
    assert_eq!(
        vec![
            0, 1, 1, // 1
            1, 0, 0, 0, 0, 1, 1, // 14
            0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u16(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u32(vec![1, 1]));
    assert_eq!(
        vec![
            0, 1, 1, // 1
            1, 0, 0, 0, 0, 1, 1, // 14
            0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u32(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u64(vec![1, 1]));
    assert_eq!(
        vec![
            0, 1, 1, // 1
            1, 0, 0, 0, 0, 1, 1, // 14
            0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u64(vec![2, 14, 65])
    );
}
