#[macro_use]
extern crate criterion;

use bit_vec::BitVec;
use fibonacci_codec::*;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

use criterion::{black_box, BatchSize, Criterion, ParameterizedBenchmark, Throughput};

#[derive(Debug)]
enum Width {
    U8,
    U16,
    U32,
    U64,
}

const ALL: &'static [Width; 4] = &[Width::U8, Width::U16, Width::U32, Width::U64];
const ELTS: usize = 500;

fn encode_multiple_benchmark(c: &mut Criterion) {
    macro_rules! logic {
        ($b:expr, $t:ty, $rng:expr) => {{
            $b.iter_batched(
                || {
                    let range = Uniform::new(1 as $t, <$t>::MAX);
                    range.sample_iter(&mut $rng).take(ELTS).collect::<Vec<$t>>()
                },
                |v| black_box(v.fib_encode().expect("should encode right")),
                BatchSize::LargeInput,
            )
        }};
    }

    let id = "encode_multiple";
    let bm = ParameterizedBenchmark::new(
        id,
        |b, ref n| {
            let mut thread_rng = thread_rng();
            match n {
                Width::U8 => logic!(b, u8, thread_rng),
                Width::U16 => logic!(b, u16, thread_rng),
                Width::U32 => logic!(b, u32, thread_rng),
                Width::U64 => logic!(b, u64, thread_rng),
            }
        },
        ALL,
    )
    .throughput(|_s| Throughput::Elements(ELTS as u64));
    c.bench(id, bm);
}

fn decode_multiple_benchmark(c: &mut Criterion) {
    macro_rules! logic {
        ($b:expr, $t:ty, $dec:expr, $rng:expr) => {{
            $b.iter_batched(
                || {
                    let range = Uniform::new(1 as $t, <$t>::MAX);
                    let v: Vec<$t> = range.sample_iter(&mut $rng).take(ELTS).collect();
                    v.fib_encode().expect("should encode right")
                },
                |bits| black_box($dec(&bits).count()),
                BatchSize::LargeInput,
            )
        }};
    }

    let id = "decode_multiple";
    let bm = ParameterizedBenchmark::new(
        id,
        |b, ref n| {
            let mut thread_rng = thread_rng();
            match n {
                Width::U8 => logic!(b, u8, fib_decode_u8, thread_rng),
                Width::U16 => logic!(b, u16, fib_decode_u16, thread_rng),
                Width::U32 => logic!(b, u32, fib_decode_u32, thread_rng),
                Width::U64 => logic!(b, u64, fib_decode_u64, thread_rng),
            }
        },
        ALL,
    )
    .throughput(|_s| Throughput::Elements(ELTS as u64));
    c.bench(id, bm);
}

fn encode_1_benchmark(c: &mut Criterion) {
    macro_rules! logic {
        ($b:expr, $t:ty, $rng:expr) => {{
            let range = Uniform::new(1 as $t, <$t>::MAX);
            $b.iter_batched(
                || range.sample(&mut $rng),
                |n| black_box(n.fib_encode().expect("should encode")),
                BatchSize::SmallInput,
            )
        }};
    }

    c.bench_function_over_inputs(
        "encode_1",
        |b, ref n| {
            let mut thread_rng = thread_rng();
            match n {
                Width::U8 => logic!(b, u8, thread_rng),
                Width::U16 => logic!(b, u16, thread_rng),
                Width::U32 => logic!(b, u32, thread_rng),
                Width::U64 => logic!(b, u64, thread_rng),
            }
        },
        ALL,
    );
}

fn decode_1_benchmark(c: &mut Criterion) {
    macro_rules! logic {
        ($b:expr, $t:ty, $dec:expr, $rng:expr) => {{
            let range = Uniform::new(1 as $t, <$t>::MAX);
            $b.iter_batched(
                || range.sample(&mut $rng).fib_encode().expect("should encode"),
                |bits| black_box($dec(bits).count()),
                BatchSize::SmallInput,
            )
        }};
    }

    c.bench_function_over_inputs(
        "decode_1",
        |b, ref n| {
            let mut thread_rng = thread_rng();
            match n {
                Width::U8 => logic!(b, u8, fib_decode_u8, thread_rng),
                Width::U16 => logic!(b, u16, fib_decode_u16, thread_rng),
                Width::U32 => logic!(b, u32, fib_decode_u32, thread_rng),
                Width::U64 => logic!(b, u64, fib_decode_u64, thread_rng),
            }
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
