extern crate bit_vec;
extern crate fib_encode;

use bit_vec::BitVec;
use fib_encode::FibEncode;

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
