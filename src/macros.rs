#![macro_use]

macro_rules! impl_fib_encode_for_integral_type {
    ($typename:ident, $table:expr, $tablelength:expr) => {
        mod $typename {
            use {FibEncode, bits_from_table};
            use bit_vec::BitVec;

            const TABLE: &'static [$typename; $tablelength] = &($table);
            impl FibEncode for $typename {
                fn fib_encode_mut(self, vec: &mut BitVec) {
                    bits_from_table(self, TABLE, vec);
                }
            }
            impl<'a> FibEncode for &'a [$typename] {
                fn fib_encode_mut(self, vec: &mut BitVec) {
                    for elt in self.iter() {
                        bits_from_table(*elt, TABLE, vec);
                    }
                }
            }
        }
    }
}
