use std::marker::PhantomData;
use std::ops::{Add, Neg, Rem, Mul,Div,Sub};
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

macro_rules! impl_constant_op {
    ($Op:ident,$op_method:ident) => {
        impl <L:$Op<R>,R> $Op<Constant<R>> for Constant<L>{
            type Output = Constant<<L as $Op<R>>::Output>;
            fn $op_method(self, rhs: Constant<R>) -> Self::Output {
                Constant(<L as $Op<R>>::$op_method(self.0,rhs.0))
            }
        }
    };
}
impl_constant_op!(Add, add);
impl_constant_op!(Mul, mul);
impl_constant_op!(Div, div);
impl_constant_op!(Rem, rem);

impl_constant_op!(Sub, sub);

impl <V:Neg> Neg for Constant<V>{
    type Output = Constant<<V as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Constant(Neg::neg(self.0))
    }
}


#[cfg(test)]
mod tests {
    use std::ops::{Div, Mul};
    use super::*;

    #[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
    pub struct Mat2<T>([[T;2];2]);

    impl <T:Mul+Clone> Mul<T> for Mat2<T>
    {
        type Output = Mat2<T::Output>;
        fn mul(self, rhs: T) -> Self::Output {
            Mat2(self.0.map(|ts|ts.map(|t|rhs.clone()*t)))
        }
    }
    macro_rules! mat2 {
        [$a:expr,$b:expr;$c:expr,$d:expr] => {Mat2([[$a,$b],[$c,$d]])};
    }


    #[test]
    fn add_same_type() {
        let result = Constant(10usize)+Constant(20usize);
        assert_eq!(result, Constant(30));
    }
    #[test]
    fn mul_different_types() {
        let result = Constant(mat2![
            1,2;
            3,4
        ])*Constant(3);
        assert_eq!(result, Constant(mat2![
            3,6;
            9,12
        ]));
    }
}
