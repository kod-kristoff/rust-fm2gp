extern crate num_integer;
extern crate num_traits;

pub mod ops;
pub mod power;
pub mod traits;
pub mod matrix;
//pub use crate::ops::ops;

pub mod integer {
    //use std::ops::Shr;
    use num_traits::int::PrimInt;

    #[inline(always)]
    pub fn half<N>(n: N) -> N
    where
        N: PrimInt, //Integer + Shr<Output = N>
    {
        n.unsigned_shr(1)
    }

    #[cfg(test)]
    mod tests {
        use integer::half;

        #[test]
        fn test_half() {
            assert_eq!(half(2), 1);
            assert_eq!(half(3), 1);
            assert_eq!(half(7), 3);
        }
    }
}
