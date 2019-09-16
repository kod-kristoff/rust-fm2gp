use std::ops::Mul;
use std::ops::Add;


pub fn mul<T>(x: T, y: T) -> T
    where T: Copy + Mul<Output = T>
{
    x * y
}

pub fn add<T>(x: T, y: T) -> T
    where T: Copy + Add<Output = T>
{
    x + y
}
