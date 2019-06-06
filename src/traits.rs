
pub trait MulOp<Rhs=Self> {

    type Output;
    fn mul_op(self, rhs: Rhs) -> Self::Output;
}

impl MulOp for bool {
    type Output = bool;

    fn mul_op(self, rhs: bool) -> bool {
        self & rhs
    }
}

pub trait Monoid<T> {
    fn identity_element(&self) -> T;

    fn op(&self) -> fn(T, T) -> T;
}
