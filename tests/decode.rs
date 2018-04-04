extern crate bit_vec;
extern crate fibonacci_codec;
extern crate num;

// use bit_vec::BitVec;
use fibonacci_codec::{decode_fib, Encode};
use std::iter::Iterator;

#[test]
fn test_roundtrip_u64() {
    let input: Vec<u64> = vec![2, 14, 65];
    let encoded = input.fib_encode();
    let decoded: Vec<u64> = decode_fib::<u64, _, _>(encoded).collect();
    assert_eq!(input, decoded);
}

#[test]
fn test_roundtrip_u8() {
    let input: Vec<u8> = vec![2, 14, 65];
    let encoded = input.fib_encode();
    let decoded: Vec<u8> = decode_fib::<u8, _, _>(encoded).collect();
    assert_eq!(input, decoded);
}
