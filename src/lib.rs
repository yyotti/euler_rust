pub trait Solver<T> {
    fn solve(&self) -> T;
}

pub mod common;
pub mod p0001;
