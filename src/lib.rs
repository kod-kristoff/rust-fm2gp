extern crate num_integer;
extern crate num_traits;

pub mod ops;

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

pub mod power {
    use crate::integer;
    use num_integer::Integer;
    use num_traits::identities::One;
    use num_traits::int::PrimInt;
    use std::ops::Mul;

    pub fn power_recursive<T, I>(x: T, n: I) -> T
    where
        T: Mul<Output = T> + One + Copy,
        I: Integer + PrimInt,
    {
        if n.is_zero() {
            T::one()
        } else if n.is_one() {
            x
        } else if n.is_odd() {
            x * power_recursive(x * x, integer::half(n))
        } else {
            power_recursive(x * x, integer::half(n))
        }
    }

    pub fn power_accumulate<T, I>(mut r: T, mut x: T, mut n: I, op: impl Fn(T, T) -> T) -> T
    where
        T: Copy,
        I: PrimInt + Integer,
    {
        if n.is_zero() {
            return r;
        }
        loop {
            if n.is_odd() {
                r = op(r, x);
                if n.is_one() {
                    return r;
                }
            }
            n = integer::half(n);
            x = op(x, x);
        }
    }

    pub fn power_semigroup<T, I>(mut x: T, mut n: I, op: impl Fn(T, T) -> T) -> T
    where
        T: Copy,
        I: Integer + PrimInt,
    {
        while n.is_even() {
            x = op(x, x);
            n = integer::half(n);
        }
        if n.is_one() {
            return x;
        }
        power_accumulate(x, op(x, x), integer::half(n - I::one()), op)
    }

    macro_rules! tests_impl {
        ($T:ty, $I:ty, $test_mod:ident) => {
            #[cfg(test)]
            mod $test_mod {
                use crate::ops;
                use power::power_accumulate;
                use power::power_recursive;
                use power::power_semigroup;

                #[test]
                fn power_recursive_tests() {
                    assert_eq!(power_recursive(2 as $T, 0 as $I), 1 as $T);
                    assert_eq!(power_recursive(2 as $T, 1 as $I), 2 as $T);
                    assert_eq!(power_recursive(2 as $T, 2 as $I), 4 as $T);
                    assert_eq!(power_recursive(2 as $T, 3 as $I), 8 as $T);
                    assert_eq!(power_recursive(2 as $T, 4 as $I), 16 as $T);
                }

                #[test]
                fn power_accumulate_mul() {
                    assert_eq!(
						power_accumulate(1 as $T, 2 as $T, 0 as $I, ops::mul),
						1 as $T
					);
                    assert_eq!(
						power_accumulate(1 as $T, 2 as $T, 1 as $I, ops::mul),
						2 as $T
					);
                    assert_eq!(
						power_accumulate(1 as $T, 2 as $T, 2 as $I, ops::mul),
						4 as $T
					);
                    assert_eq!(
						power_accumulate(1 as $T, 2 as $T, 3 as $I, ops::mul),
						8 as $T
					);
                    assert_eq!(
						power_accumulate(1 as $T, 2 as $T, 4 as $I, ops::mul),
						16 as $T
					);
                }

                #[test]
                fn power_semigroup_mul() {
                    assert_eq!(power_semigroup(3 as $T, 1 as $I, ops::mul), 3 as $T);
                    assert_eq!(power_semigroup(3 as $T, 2 as $I, ops::mul), 9 as $T);
                    assert_eq!(power_semigroup(3 as $T, 3 as $I, ops::mul), 27 as $T);
                    assert_eq!(power_semigroup(3 as $T, 4 as $I, ops::mul), 81 as $T);
                }

                #[test]
                fn power_semigroup_add() {
                    assert_eq!(power_semigroup(3 as $T, 1 as $I, ops::add), 3 as $T);
                    assert_eq!(power_semigroup(3 as $T, 2 as $I, ops::add), 6 as $T);
                    assert_eq!(power_semigroup(3 as $T, 3 as $I, ops::add), 9 as $T);
                    assert_eq!(power_semigroup(3 as $T, 4 as $I, ops::add), 12 as $T);
                }

                #[test]
                fn power_semigroup_lambda_add() {
                    assert_eq!(power_semigroup(3 as $T, 1 as $I, |a, b| a + b), 3 as $T);
                    assert_eq!(power_semigroup(3 as $T, 2 as $I, |a, b| a + b), 6 as $T);
                    assert_eq!(power_semigroup(3 as $T, 3 as $I, |a, b| a + b), 9 as $T);
                    assert_eq!(power_semigroup(3 as $T, 4 as $I, |a, b| a + b), 12 as $T);
                }

                #[test]
                fn power_semigroup_lambda_mul() {
                    assert_eq!(power_semigroup(3 as $T, 1 as $I, |a, b| a * b), 3 as $T);
                    assert_eq!(power_semigroup(3 as $T, 2 as $I, |a, b| a * b), 9 as $T);
                    assert_eq!(power_semigroup(3 as $T, 3 as $I, |a, b| a * b), 27 as $T);
                    assert_eq!(power_semigroup(3 as $T, 4 as $I, |a, b| a * b), 81 as $T);
                }
            }
        };
    }

    tests_impl!(f32, i8, tests_f32_i8);
    tests_impl!(f64, u32, tests_f64_u32);
    tests_impl!(i16, i8, tests_i16_i8);
    tests_impl!(i32, u16, tests_i32_u16);
    tests_impl!(u64, i32, tests_u64_i32);
    tests_impl!(u32, u64, tests_u32_u64);
}
