extern crate bit_vec;
extern crate fibonacci_codec;

use bit_vec::BitVec;
use fibonacci_codec::Encode;

fn to_bits(slice: BitVec) -> Vec<u8> {
    slice.iter().map(|b| if b { 1 } else { 0 } as u8).collect()
}

#[test]
fn test_factor_wikipedia() {
    // Some of the examples on https://en.wikipedia.org/wiki/Fibonacci_coding:
    assert_eq!(vec![0, 1, 1], to_bits((2 as u8).fib_encode()));
    assert_eq!(vec![1, 0, 1, 1], to_bits((4 as u8).fib_encode()));
    assert_eq!(vec![0, 0, 1, 1], to_bits((3 as u8).fib_encode()));
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 1], to_bits((13 as u8).fib_encode()));
    assert_eq!(vec![1, 0, 0, 0, 0, 1, 1], to_bits((14 as u8).fib_encode()));
    assert_eq!(
        vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 1],
        to_bits((65 as u8).fib_encode())
    );
}

fn check_slice_u8(numbers: Vec<u8>) -> Vec<u8> {
    to_bits(numbers.fib_encode())
}
fn check_slice_u16(numbers: Vec<u16>) -> Vec<u8> {
    to_bits(numbers.fib_encode())
}
fn check_slice_u32(numbers: Vec<u32>) -> Vec<u8> {
    to_bits(numbers.fib_encode())
}
fn check_slice_u64(numbers: Vec<u64>) -> Vec<u8> {
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
        vec![0, 1, 1, // 1
             1, 0, 0, 0, 0, 1, 1, // 14
             0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u8(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u16(vec![1, 1]));
    assert_eq!(
        vec![0, 1, 1, // 1
             1, 0, 0, 0, 0, 1, 1, // 14
             0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u16(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u32(vec![1, 1]));
    assert_eq!(
        vec![0, 1, 1, // 1
             1, 0, 0, 0, 0, 1, 1, // 14
             0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u32(vec![2, 14, 65])
    );
    assert_eq!(vec![1, 1, 1, 1], check_slice_u64(vec![1, 1]));
    assert_eq!(
        vec![0, 1, 1, // 1
             1, 0, 0, 0, 0, 1, 1, // 14
             0, 1, 0, 0, 1, 0, 0, 0, 1, 1, // 65
        ],
        check_slice_u64(vec![2, 14, 65])
    );
}
