extern crate fibonacci;
extern crate num;

use fibonacci::Fibonacci;
use num::{CheckedAdd, One, Zero};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn generate<T>() -> Vec<T>
where
    T: Zero + One + Debug + CheckedAdd + Clone,
{
    let numbers: Vec<T> = Fibonacci::default().collect();
    numbers
}

const PREAMBLE: &'static str = r#"// Generated with "cargo run" in ../generator/
#![cfg_attr(rustfmt, rustfmt_skip)]
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

"#;

fn write_out<T>(out: &mut Write, t_name: &str, safe_t_name: &str)
where
    T: Zero + One + Debug + CheckedAdd + Clone,
{
    let ints = generate::<T>();
    write!(
        out,
        "impl_fib_encode_for_integral_type!({}, {}, {:?}, DecodeIter, fib_decode_{}, {:?}, {});\n",
        t_name,
        safe_t_name,
        t_name,
        t_name,
        ints,
        ints.len()
    ).unwrap();
}

fn write_decode_wrapper<'a>(out: &mut Write, t_names: Vec<&'a str>) -> Result<(), std::io::Error> {
    out.write_all(b"\npub(crate) mod funcs {\n")?;
    for typename in t_names.iter() {
        write!(
            out,
            "    impl_decoder!({}, {:?}, fib_decode_{});\n",
            typename, typename, typename
        )?;
    }
    out.write_all(b"}\n")?;
    Ok(())
}

fn main() {
    let output = Path::new("../src/int.rs");
    let mut out = BufWriter::new(File::create(output).unwrap());
    write!(&mut out, "{}", PREAMBLE).unwrap();

    write_out::<u8>(&mut out, "u8", "NonZeroU8");
    write_out::<u16>(&mut out, "u16", "NonZeroU16");
    write_out::<u32>(&mut out, "u32", "NonZeroU32");
    write_out::<u64>(&mut out, "u64", "NonZeroU64");
    write_out::<u128>(&mut out, "u128", "NonZeroU128");
    write_decode_wrapper(&mut out, vec!["u8", "u16", "u32", "u64", "u128"]).unwrap();
    out.flush().unwrap();
}
