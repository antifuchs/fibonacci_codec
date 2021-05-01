#[macro_use]
extern crate criterion;

use bit_vec::BitVec;
use fibonacci_codec::*;

use criterion::{black_box, Criterion, ParameterizedBenchmark, Throughput};

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

const ALL: &'static [Width; 4] = &[Width::U8, Width::U16, Width::U32, Width::U64];
const ELTS: usize = 20;

fn encode_multiple_benchmark(c: &mut Criterion) {
    let id = "encode_multiple";
    let bm = ParameterizedBenchmark::new(
        id,
        |b, ref n| {
            b.iter(|| {
                let v = vec![n.sample(); ELTS];
                v.fib_encode().expect("should be encodable")
            })
        },
        ALL,
    )
    .throughput(|_s| Throughput::Elements(ELTS as u64));
    c.bench(id, bm);
}

fn decode_multiple_benchmark(c: &mut Criterion) {
    let id = "decode_multiple";
    let bm = ParameterizedBenchmark::new(
        id,
        |b, ref d| {
            let input = vec![d.sample(); ELTS];
            let bits = input.fib_encode().unwrap();
            b.iter(move || assert_eq!(ELTS, d.decode(&bits)));
        },
        ALL,
    )
    .throughput(|_s| Throughput::Elements(ELTS as u64));
    c.bench(id, bm);
}

fn encode_1_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "encode_1",
        |b, ref n| b.iter(|| n.sample().fib_encode().expect("should be encodable")),
        ALL,
    );
}

fn decode_1_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "decode_1",
        |b, ref d| {
            let input = d.sample();
            let bits = input.fib_encode().unwrap();
            b.iter(move || black_box(d.decode(&bits)));
        },
        ALL,
    );
}

criterion_group!(
    benches,
    encode_multiple_benchmark,
    decode_multiple_benchmark,
    encode_1_benchmark,
    decode_1_benchmark,
);
criterion_main!(benches);
