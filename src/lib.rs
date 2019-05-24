#[inline]
pub fn half(n: i32) -> i32 {
    n >> 1
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

pub fn power_accumulate(r: i32, x: i32, n: i32) -> i32 {
	if n == 0 {
		return r
	}
	let mut r = r;
	let mut x = x;
	let mut n = n;
	loop {
		if is_odd(n) {
			r = r * x;
			if n == 1 {
				return r
			}
		}
		n = half(n);
		x = x * x;
	}
}

#[cfg(test)]
mod tests {
	use power_accumulate;
	use power_recursive;
    use half;
    use is_odd;

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
		assert_eq!(power_accumulate(1, 2i32, 0), 1);
		assert_eq!(power_accumulate(1, 2, 1), 2);
		assert_eq!(power_accumulate(1, 2, 2), 4);
		assert_eq!(power_accumulate(1, 2, 3), 8);
		assert_eq!(power_accumulate(1, 2, 4), 16);
	}

    #[test]
    fn test_half() {
        assert_eq!(half(2), 1);
        assert_eq!(half(3), 1);
        assert_eq!(half(7), 3);
    }

    #[test]
    fn test_is_odd() {
        assert_eq!(is_odd(7), true);
        assert_eq!(is_odd(8), false);
    }
}
