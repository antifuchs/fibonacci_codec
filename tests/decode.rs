extern crate bit_vec;
extern crate fibonacci_codec;
extern crate num;

// use bit_vec::BitVec;
use fibonacci_codec::{DecodeError, EncodeSlice, fib_decode_u64, fib_decode_u8};
use std::iter::Iterator;

#[test]
fn test_roundtrip_u64() {
    let input: Vec<u64> = vec![2, 14, 65];
    let encoded = input.fib_encode().unwrap();
    let decoded: Vec<u64> = fib_decode_u64(encoded).map(|x| x.unwrap()).collect();
    assert_eq!(input, decoded);
}

#[test]
fn test_roundtrip_u8() {
    let input: Vec<u8> = vec![2, 14, 65];
    let encoded = input.fib_encode().unwrap();
    let decoded: Vec<u8> = fib_decode_u8(encoded).map(|x| x.unwrap()).collect();
    assert_eq!(input, decoded);
}

#[test]
fn test_overflow_from_fib_elt() {
    let input: Vec<u64> = vec![23894089128374];
    let encoded = input.fib_encode().unwrap();
    let decoded: Vec<Result<u8, DecodeError>> = fib_decode_u8(encoded).collect();
    assert_eq!(
        Err(DecodeError::FibonacciElementOverflow { bit_pos: 12 }),
        decoded[0]
    );
}

#[test]
fn test_overflow_from_addition() {
    let input: Vec<u64> = vec![256];
    let encoded = input.fib_encode().unwrap();
    let decoded: Vec<Result<u8, DecodeError>> = fib_decode_u8(encoded).collect();
    assert_eq!(
        Err(DecodeError::ConstructionOverflow { bit_pos: 11 }),
        decoded[0]
    );
}
