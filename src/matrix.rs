use std::ops::{Add, Mul};

use num_traits::{Zero, One};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Matrix22<T>
    where T: Copy + Mul<Output = T>
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

#[cfg(test)]
mod tests {
    use matrix::Matrix22;

    #[test]
    fn test_mul() {
        let m1 = Matrix22 {v00: 1, v01: 2, v10: 3, v11: 4};
        let m2 = Matrix22 {v00: 4, v01: 3, v10: 2, v11: 1};
        let m3 = m1 * m2;
        assert_eq!(m3.v00, 8);
    }

}
