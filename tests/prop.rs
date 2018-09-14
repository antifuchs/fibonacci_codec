extern crate bit_vec;
extern crate fibonacci_codec;
extern crate nonzero_ext;
extern crate num;

#[macro_use]
extern crate proptest;

use std::num::NonZeroU64;

use fibonacci_codec::{fib_decode_u16, fib_decode_u32, fib_decode_u64, fib_decode_u8};
use fibonacci_codec::{Encode, EncodeOne};

use nonzero_ext::NonZeroAble;
use proptest::prelude::*;

proptest! {
   #[test]
    fn roundtrips(input in any::<u64>().prop_filter("Values must be >0".to_owned(),
                                                    |v| v.as_nonzero().is_some())) {
        let bits = input.as_nonzero().unwrap().fib_encode().expect("Expected an Ok result");
        if input <= 0xff {
            let decoded: Vec<u8> = fib_decode_u8(&bits).filter_map(|x| x.ok()).collect();
            prop_assert_eq!(decoded, vec![input as u8], "Decoding as u8");
        }
        if input <= 0xffff {
            let decoded: Vec<u16> = fib_decode_u16(&bits).filter_map(|x| x.ok()).collect();
            prop_assert_eq!(decoded, vec![input as u16], "Decoding as u16");
        }
        if input <= 0xffffffff {
            let decoded: Vec<u32> = fib_decode_u32(&bits).filter_map(|x| x.ok()).collect();
            prop_assert_eq!(decoded, vec![input as u32], "Decoding as u32");
        }
        let decoded: Vec<u64> = fib_decode_u64(&bits).filter_map(|x| x.ok()).collect();
        prop_assert_eq!(decoded, vec![input as u64], "Decoding as u64");
    }

    #[test]
    fn roundtrips_array(ref input in
                        proptest::collection::vec(any::<u64>().prop_filter("Values must be >0".to_owned(),
                                                                           |v| v.as_nonzero().is_some()),
                                                  1..100)) {
        let input = input.into_iter().filter_map(|i| i.as_nonzero()).collect::<Vec<NonZeroU64>>();
        let bits = input.clone().fib_encode().expect("Expected an Ok result");
        let decoded: Vec<u64> = fib_decode_u64(bits).filter_map(|x| x.ok()).collect();
        prop_assert_eq!(decoded,
                        input
                        .into_iter()
                        .map(|n| n.get())
                        .collect::<Vec<u64>>());
    }
}
