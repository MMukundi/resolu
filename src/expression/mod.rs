use std::convert::Infallible;
use std::ops::{Add, Try};

pub mod variable;
pub mod constant;

pub type InfallibleResult<T> = Result<T,Infallible>;
fn get<T>(infallible_result:InfallibleResult<T>)->T{
    match infallible_result {
        Ok(val)=>val,
        Err(infallible)=>{
            match infallible {

            }
        }
    }
}

pub trait Expression
{
    type Value;
    fn try_add_expr<E:Expression>(self,expr:E)->Self::Output
        where Self:Add<E>, Self::Output: Try, <<Self as Add<E>>::Output as Try>::Output :Expression, Self:Sized
    {
        Add::add(self,expr)
    }
    fn add_infallible<E:Expression,V:Expression>(self,expr:E)->V
        where Self:Add<E,Output=InfallibleResult<V>>,Self:Sized
    {
        get(self.try_add_expr(expr))
    }
}

pub trait IntoExpression<Value>{
    type IntoExpr:Expression<Value=Value>;
    fn into_expression(self)->Self::IntoExpr;
}
pub trait FromExpression<Value>{
    fn from_expression<E:Expression<Value=Value>>(expression:E)->Self;
}