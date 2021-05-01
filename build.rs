extern crate num;

use num::{CheckedAdd, Integer, One};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::{fmt::Debug, iter::successors};

fn generate<T>() -> Vec<T>
where
    T: Integer + One + CheckedAdd + Debug + Copy,
{
    successors(Some((T::one(), T::one())), |(prev, cur)| {
        prev.checked_add(cur).map(|next| (*cur, next))
    })
    .map(|(_, cur)| cur)
    .collect()
}

const PREAMBLE: &str = r#""#;

fn write_out<T>(out: &mut dyn Write, t_name: &str)
where
    T: Integer + One + CheckedAdd + Debug + Copy,
{
    let ints = generate::<T>();
    writeln!(
        out,
        "impl_fib_encode_for_integral_type!({}, {:?}, DecodeIter, fib_decode_{}, {:?});",
        t_name, t_name, t_name, ints,
    )
    .unwrap();
}

fn write_decode_wrapper(out: &mut dyn Write, t_names: Vec<&'_ str>) -> Result<(), std::io::Error> {
    out.write_all(b"\npub(crate) mod funcs {\n")?;
    for typename in t_names.iter() {
        writeln!(
            out,
            "    impl_decoder!({}, {:?}, fib_decode_{});",
            typename, typename, typename
        )?;
    }
    out.write_all(b"}\n")?;
    Ok(())
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let output = Path::new(&out_dir).join("int.rs");
    let mut out = BufWriter::new(File::create(&output).unwrap());
    write!(&mut out, "{}", PREAMBLE).unwrap();

    write_out::<u8>(&mut out, "u8");
    write_out::<u16>(&mut out, "u16");
    write_out::<u32>(&mut out, "u32");
    write_out::<u64>(&mut out, "u64");
    write_decode_wrapper(&mut out, vec!["u8", "u16", "u32", "u64"]).unwrap();
    out.flush().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
