use std::ops::Add;
use std::ops::Mul;
use num_traits::One;
use crate::traits::Monoid;

pub fn mul<T>(x: T, y: T) -> T
where
    T: Copy + Mul<Output = T>,
{
    x * y
}

pub fn add<T>(x: T, y: T) -> T
where
    T: Copy + Add<Output = T>,
{
    x + y
}

pub struct Mult<T> {
    op_: fn(T, T)->T,
}

impl<T> Monoid<T> for Mult<T>
    where T: Mul<Output = T> + Copy + One
{
    fn identity_element(&self) -> T {
        T::one()
    }

    fn op(&self) -> fn(T, T) -> T {
        self.op_
    }
}

pub fn mult<T>() -> Mult<T> 
    where T: Copy + Mul<Output = T>
{
    Mult {op_: mul}
}
