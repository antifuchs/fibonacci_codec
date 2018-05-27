#[macro_use]
extern crate criterion;
extern crate bit_vec;
extern crate fibonacci_codec;

use bit_vec::BitVec;
use fibonacci_codec::*;

use criterion::Criterion;

#[derive(Debug)]
enum Width {
    U8,
    U16,
    U32,
    U64,
}

impl Width {
    #[inline]
    fn sample(&self) -> u64 {
        match *self {
            Width::U8 => 0xf0,
            Width::U16 => 0xfff0,
            Width::U32 => 0xfffffff0,
            Width::U64 => 0xfffffffffffffff0,
        }
    }

    fn decode(&self, bits: &BitVec) -> usize {
        match *self {
            Width::U8 => fib_decode_u8(bits).count(),
            Width::U16 => fib_decode_u16(bits).count(),
            Width::U32 => fib_decode_u32(bits).count(),
            Width::U64 => fib_decode_u64(bits).count(),
        }
    }
}

fn encode_1_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "encode_1",
        |b, &n| b.iter(|| n.sample().fib_encode().expect("should be encodable")),
        &[Width::U8, Width::U16, Width::U32, Width::U64],
    );
}

fn encode_multiple_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "encode_multiple",
        |b, &&n| {
            b.iter(|| {
                let v = vec![n as u64; 20];
                v.fib_encode().expect("should be encodable")
            })
        },
        &[0xf0, 0xfff0, 0xfffffff0 as u64, 0xfffffffffffffff0 as u64],
    );
}

fn decode_1_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "decode_1",
        |b, &d| {
            let input = d.sample();
            let bits = input.fib_encode().unwrap();
            b.iter(move || d.decode(&bits));
        },
        &[Width::U8, Width::U16, Width::U32, Width::U64],
    );
}

fn decode_multiple_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "decode_multiple",
        |b, &d| {
            let input = vec![d.sample() as u8; 20];
            let bits = input.fib_encode().unwrap();
            b.iter(move || assert_eq!(20, d.decode(&bits)));
        },
        &[Width::U8, Width::U16, Width::U32, Width::U64],
    );
}
criterion_group!(
    benches,
    encode_1_benchmark,
    encode_multiple_benchmark,
    decode_1_benchmark,
    decode_multiple_benchmark,
);
criterion_main!(benches);
