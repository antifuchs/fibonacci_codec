#![macro_use]

macro_rules! impl_fib_encode_for_integral_type {
    ($typename:ident, $typename_str:expr, $decoder_name:ident, $decode_name:ident, $table:expr, $tablelength:expr) => {
        pub(crate) mod $typename {
            use {Encode, bits_from_table};
            use decode::{DecodeError, decode_from};
            use bit_vec::BitVec;

            pub(crate) const TABLE: &'static [$typename; $tablelength] = &($table);

            impl Encode for $typename {
                fn fib_encode_mut(self, vec: &mut BitVec) {
                    bits_from_table(self, TABLE, vec);
                }
            }

            impl<'a> Encode for &'a [$typename] {
                fn fib_encode_mut(self, vec: &mut BitVec) {
                    for elt in self.iter() {
                        bits_from_table(*elt, TABLE, vec);
                    }
                }
            }

            #[doc = "Decodes a fibonacci-encoded bit-stream into `"]
            #[doc = $typename_str]
            #[doc = "` integers."]
            ///
            /// It wraps an iterator over `bool` and returns the
            /// result of a decode attempt.
            pub struct $decoder_name<I> { orig: I }

            impl<I> Iterator for $decoder_name<I>
            where
                I: Iterator<Item = bool>,
            {
                type Item = Result<$typename, DecodeError>;

                fn next(&mut self) -> Option<Self::Item> {
                    decode_from(&mut self.orig, TABLE)
                }
            }

            #[doc = "Wraps an iterator over booleans and decodes into `"]
            #[doc = $typename_str]
            #[doc = "` integers."]
            pub fn $decode_name<T, I>(collection: T) -> $decoder_name<I>
            where
                T: IntoIterator<Item = bool, IntoIter = I>,
                I: Iterator<Item = bool>,
            {
                $decoder_name { orig: collection.into_iter() }
            }
        }
        pub use $typename::*;
    }
}
