// #![feature(specialization)]
use std::ops::{Add, Mul};
// use crate::traits::Mul;

use num_traits::{Zero, One};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Matrix22<T>
    where T: Copy
    //where T: Copy + Zero + One + Add<Output = T> + Mul<Output = T>
{
    v00: T,
    v01: T,
    v10: T,
    v11: T,
}

impl<T> Mul for Matrix22<T> 
    where T: Copy + Mul<Output = T> + Add<Output = T>
{
    type Output = Matrix22<T>;

    fn mul(self, other: Matrix22<T>) -> Matrix22<T> {
        Matrix22 {
            v00: self.v00 * other.v00 + self.v01 * other.v10,
            v01: self.v00 * other.v01 + self.v01 * other.v11,
            v10: self.v10 * other.v00 + self.v11 * other.v10,
            v11: self.v10 * other.v01 + self.v11 * other.v11,
        }
    }
}

impl<T> Add for Matrix22<T> 
    where T: Copy + Add<Output = T>
{
    type Output = Matrix22<T>;

    fn add(self, other: Matrix22<T>) -> Matrix22<T> {
        Matrix22 {
            v00: self.v00 + other.v00,
            v01: self.v01 + other.v01,
            v10: self.v10 + other.v10,
            v11: self.v11 + other.v11,
        }
    }
}

impl<T> Zero for Matrix22<T>
    where T: Copy + Zero
{
    fn is_zero(&self) -> bool {
        self.v00.is_zero() && self.v01.is_zero() && self.v10.is_zero() && self.v11.is_zero()
    }
    fn zero() -> Self {
        Matrix22 {
            v00: T::zero(),
            v01: T::zero(),
            v10: T::zero(),
            v11: T::zero(),
        }
    }
}
//impl Mul<bool> for Matrix22<bool> {
//    type Output = Matrix22<bool>;
//    fn mul(self, other: Matrix22<bool>) -> Matrix22<bool> {
//        Matrix22 {
//            v00: self.v00 & other.v00 | self.v01 & other.v10,
//            v01: self.v00 & other.v01 | self.v01 & other.v11,
//            v10: self.v10 & other.v00 | self.v11 & other.v10,
//            v11: self.v10 & other.v01 | self.v11 & other.v11,
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use matrix::Matrix22;
    use num_traits::Zero;

    #[test]
    fn test_mul() {
        let m1 = Matrix22 {v00: 1, v01: 2, v10: 3, v11: 4};
        let m2 = Matrix22 {v00: 4, v01: 3, v10: 2, v11: 1};
        let m3 = m1 * m2;
        assert_eq!(m3.v00, 8);
    }

    #[test]
    fn test_zero() {
        let m = Matrix22 {v00: 1, v01: 2, v10: 3, v11: 4};
        assert!(!m.is_zero());
        let m = Matrix22 {v00: 0, v01: 2, v10: 3, v11: 4};
        assert!(!m.is_zero());
        let m = Matrix22 {v00: 1, v01: 0, v10: 3, v11: 4};
        assert!(!m.is_zero());
        let m = Matrix22 {v00: 1, v01: 2, v10: 0, v11: 4};
        assert!(!m.is_zero());
        let m = Matrix22 {v00: 1, v01: 2, v10: 3, v11: 0};
        assert!(!m.is_zero());
        assert!(Matrix22::<i32>::zero().is_zero())

    }

//     #[test]
//     fn test_bool_mul() {
//         let m1 = Matrix22 {v00: true, v01: true, v10: true, v11: false};
//         let m2 = Matrix22 {v00: true, v01: false, v10: false, v11: false};
//         let m3 = m1 * m2;
//         assert_eq!(m3.v00, true);
//         assert_eq!(m3.v01, false);
//         assert_eq!(m3.v10, true);
//         assert_eq!(m3.v11, false);
//     }

}
