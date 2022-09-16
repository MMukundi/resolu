use std::marker::PhantomData;
use crate::expression::{Expression, IntoExpression};

pub struct Variable<I,V>{
    identifier: I,
    phantom_value: PhantomData<V>
}
impl <I,V> Variable<I,V> {
    pub fn new(identifier:I)->Self{
        Self{
            identifier,
            phantom_value:PhantomData
        }
    }
}

impl <I,V> Expression for Variable<I,V> {
    type Value = V;
}

impl <I,V,Value> IntoExpression<Value> for Variable<I,V> {
    type IntoExpr = Variable<I,Value>;

    fn into_expression(self) -> Self::IntoExpr {
        Variable::new(self.identifier)
    }
}