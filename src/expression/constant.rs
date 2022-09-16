use std::marker::PhantomData;
use std::ops::Add;
use crate::expression::{Expression, IntoExpression};

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
pub struct Constant<V>(pub V);

impl <V> Expression for Constant<V> {
    type Value = V;
}

impl <V,W:From<V>> IntoExpression<W> for Constant<V> {
    type IntoExpr = Constant<W>;
    fn into_expression(self) -> Self::IntoExpr {
        Constant(self.0.into())
    }
}

impl <L:Add<R>,R> Add<Constant<R>> for Constant<L>{
    type Output = Constant<<L as Add<R>>::Output>;
    fn add(self, rhs: Constant<R>) -> Self::Output {
        Constant(self.0+rhs.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
    pub struct LockstepPair(usize,usize);
    impl Expression for LockstepPair {
        type Value = (usize,usize);
    }
    impl Add<usize> for LockstepPair {
        type Output = Self;

        fn add(self, rhs: usize) -> Self::Output {
            Self(self.0+rhs,self.1+rhs)
        }
    }


    #[test]
    fn add_ints() {
        let result = Constant(10usize)+Constant(20usize);
        assert_eq!(result, Constant(30));
    }
    #[test]
    fn add_different_types() {
        let result = Constant(LockstepPair(1,33))+Constant(20usize);
        assert_eq!(result, Constant(LockstepPair(21,53)));
    }
}
