pub trait Monoid<T> {
    fn identity_element(&self) -> T;

    fn op(&self) -> fn(T, T) -> T;
}
