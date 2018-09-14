#![macro_use]

macro_rules! impl_fib_encode_for_integral_type {
    (
        $typename:ident,
        $safe_typename:ident,
        $typename_str:expr,
        $decoder_name:ident,
        $decode_name:ident,
        $table:expr,
        $tablelength:expr
    ) => {
        #[doc = "Functions and iterators to decode `"]
        #[doc = $typename_str]
        #[doc = "` integers."]
        pub mod $typename {
            use bit_vec::BitVec;
            use decode::{decode_from, DecodeError};
            use encode::{bits_from_table, ElementEncodeError, Encode, EncodeError, EncodeOne};
            use std::fmt::Debug;
            use std::num::$safe_typename;

            pub(crate) const TABLE: &'static [$typename; $tablelength] = &($table);

            impl EncodeOne for $safe_typename {
                type Base = $typename;

                fn fib_encode_mut(self, vec: &mut BitVec) -> Result<(), EncodeError<$typename>> {
                    bits_from_table(self.get(), TABLE, vec)
                }
            }

            impl<'a, T> Encode<$typename, $safe_typename> for T
            where
                T: IntoIterator<Item = &'a $safe_typename> + Debug + Send + Sync,
            {
                fn fib_encode_mut(
                    self,
                    vec: &mut BitVec,
                ) -> Result<(), ElementEncodeError<$typename>> {
                    for (i, elt) in self.into_iter().enumerate() {
                        match bits_from_table(elt.get(), TABLE, vec) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(ElementEncodeError { index: i, error: e });
                            }
                        }
                    }
                    Ok(())
                }
            }

            #[doc = "An iterator that yields fibonacci-decoded `"]
            #[doc = $typename_str ]
            #[doc = "` integers."]
            pub struct $decoder_name<I> {
                pub(crate) orig: I,
            }

            impl<I: Iterator<Item = bool>> Iterator for $decoder_name<I> {
                #[doc = "This iterator yields `Ok("]
                #[doc = $typename_str]
                #[doc = ")` when a number could be decoded successfully and returns an error"]
                #[doc = "otherwise."]
                type Item = Result<$typename, DecodeError>;

                fn next(&mut self) -> Option<Self::Item> {
                    decode_from(&mut self.orig, TABLE)
                }
            }
        }
    };
}

macro_rules! impl_decoder {
    ($typename:ident, $typename_str:expr, $decode_name:ident) => {
        #[doc = "Returns an iterator that consumes bits (`bool`) and fibonacci-decodes them"]
        #[doc = "into `"]
        #[doc = $typename_str]
        #[doc = "` integers."]
        pub fn $decode_name<T, I>(collection: T) -> super::$typename::DecodeIter<I>
        where
            T: IntoIterator<Item = bool, IntoIter = I>,
            I: Iterator<Item = bool>,
        {
            super::$typename::DecodeIter {
                orig: collection.into_iter(),
            }
        }
    };
}
