extern crate num_integer;
extern crate num_traits;

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
    use num_traits::identities::One;
    use num_traits::int::PrimInt;
    use num_integer::Integer;
    use std::ops::Mul;
    use crate::integer;

    pub fn power_recursive<T, I>(x: T, n: I) -> T
        where
            T: Mul<Output = T> + One + Copy,
            I: Integer + PrimInt
    {
        if n.is_zero() {
            T::one()
        } else if n.is_one() {
            x
        } else {
            if n.is_odd() {
                x * power_recursive(x * x, integer::half(n))
            } else {
                power_recursive(x * x, integer::half(n))
            }
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
        I: Integer + PrimInt
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

    #[cfg(test)]
    mod tests {
        use power::power_accumulate;
        use power::power_recursive;
        use power::power_semigroup;
        use std::ops::Mul;

        fn mul<T>(x: T, y: T) -> T
        where
            T: Copy + Mul<Output = T>,
        {
            x * y
        }

        #[test]
        fn power_recursive_i32_n0() {
            assert_eq!(power_recursive(2i32, 0), 1);
            assert_eq!(power_recursive(2, 1), 2);
            assert_eq!(power_recursive(2, 2), 4);
            assert_eq!(power_recursive(2, 3), 8);
            assert_eq!(power_recursive(2, 4), 16);
        }

        #[test]
        fn power_accumulate_i32_n0() {
            assert_eq!(power_accumulate(1, 2i32, 0, mul), 1);
            assert_eq!(power_accumulate(1, 2, 1, mul), 2);
            assert_eq!(power_accumulate(1, 2, 2, mul), 4);
            assert_eq!(power_accumulate(1, 2, 3, mul), 8);
            assert_eq!(power_accumulate(1, 2, 4, mul), 16);
        }

        #[test]
        fn power_semigroup_i32_n0() {
            assert_eq!(power_semigroup(3, 1, mul), 3);
            assert_eq!(power_semigroup(3, 2, mul), 9);
            assert_eq!(power_semigroup(3, 3, mul), 27);
            assert_eq!(power_semigroup(3, 4, mul), 81);
        }

        #[test]
        fn power_semigroup_lambda_f64() {
            assert_eq!(power_semigroup(3.0, 1, |a, b| a + b), 3.0);
            assert_eq!(power_semigroup(3.0, 2, mul), 9.0);
            assert_eq!(power_semigroup(3.0, 3, |a, b| a * b), 27.0);
            assert_eq!(power_semigroup(3.0, 4, mul), 81.0);
        }

    }
}
