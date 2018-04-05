# Fibonacci coding for primitive integers in Rust

This crate implements the [Fibonacci
coding](https://en.wikipedia.org/wiki/Fibonacci_coding) technique for
storing integers as variable bit length code words. It implements an
encoder consuming an interator over various primitive unsigned integer
types (`u8` through `u64`), and a decoder to reverse the process.

## Restrictions

Due to the way the coding scheme works, the number `0` can't be
encoded.
