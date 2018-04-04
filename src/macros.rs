#![macro_use]

macro_rules! impl_fib_encode_for_integral_type {
    ($typename:ident, $table:expr, $tablelength:expr) => {
        pub(crate) mod $typename {
            use {Encode, Decoder, bits_from_table, decode_from};
            use bit_vec::BitVec;

            const TABLE: &'static [$typename; $tablelength] = &($table);
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
            impl<I> Iterator for Decoder<I, $typename>
            where
                I: Iterator<Item = bool>,
            {
                type Item = $typename;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    decode_from(&mut self.orig, TABLE)
                }
            }
        }
    }
}
