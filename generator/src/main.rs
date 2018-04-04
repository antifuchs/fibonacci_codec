extern crate fibonacci;
extern crate num;

use fibonacci::Fibonacci;
use std::fs::File;
use num::{CheckedAdd, One, Zero};
use std::fmt::Debug;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;

fn generate<T>() -> Vec<T>
where
    T: Zero + One + Debug + CheckedAdd + Clone,
{
    let numbers: Vec<T> = Fibonacci::default().collect();
    numbers
}

const PREAMBLE: &'static str = r#"// Generated with "cargo run" in ../generator/
#![cfg_attr(rustfmt, rustfmt_skip)]

"#;

fn write_out<T>(out: &mut Write, t_name: &str)
where
    T: Zero + One + Debug + CheckedAdd + Clone,
{
    let ints = generate::<T>();
    write!(
        out,
        "impl_fib_encode_for_integral_type!({}, {:?}, Decode{}, decode_{}, {:?}, {});\n",
        t_name,
        t_name,
        t_name.to_uppercase(),
        t_name,
        ints,
        ints.len()
    ).unwrap();
}

fn main() {
    let output = Path::new("../src/tables.rs");
    let mut out = BufWriter::new(File::create(output).unwrap());
    write!(&mut out, "{}", PREAMBLE).unwrap();

    write_out::<u8>(&mut out, "u8");
    write_out::<u16>(&mut out, "u16");
    write_out::<u32>(&mut out, "u32");
    write_out::<u64>(&mut out, "u64");
}
