extern crate num_integer;

pub mod integer {
    use std::ops::Shr;

    #[inline(always)]
    pub fn half<N>(n: N) -> N
        where N: Shr
    {
        n >> 1
    }

    #[cfg(test)]
    mod tests {
        use half;

        #[test]
        fn test_half() {
            assert_eq!(half(2), 1);
            assert_eq!(half(3), 1);
            assert_eq!(half(7), 3);
        }
    }
}

#[inline]
pub fn is_odd(n: i32) -> bool {
    (n & 0x1) != 0
}

pub fn power_recursive(x: i32, n: i32) -> i32 {
	if n == 0 {
		1
	} else if n == 1 {
		x
	} else {
		if is_odd(n) {
			x * power_recursive(x*x, n / 2)
		} else {
			power_recursive(x*x, n/2)
		}
	}

}

pub fn power_accumulate<T>(
        mut r: T,
        mut x: T,
        mut n: i32,
        op: impl Fn(T, T) -> T
    ) -> T
    where T: Copy
{
	if n == 0 {
		return r
	}
	loop {
		if is_odd(n) {
			r = op(r, x);
			if n == 1 {
				return r
			}
		}
		n = integer::half(n);
		x = op(x, x);
	}
}

pub fn power_semigroup<T>(
        mut x: T,
        mut n: i32,
        op: impl Fn(T, T) -> T
    ) -> T
    where T: Copy
{
    while !is_odd(n) {
        x = op(x, x);
        n = integer::half(n);
    }
    if n == 1 {
        return x
    }
    power_accumulate(x, op(x, x), integer::half(n-1), op)
}

#[cfg(test)]
mod tests {
	use power_accumulate;
	use power_recursive;
	use power_semigroup;
    use is_odd;
    use std::ops::Mul;

    fn mul<T>(x: T, y: T) -> T
        where T: Copy + Mul<Output = T>
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


    #[test]
    fn test_is_odd() {
        assert_eq!(is_odd(7), true);
        assert_eq!(is_odd(8), false);
    }
}
