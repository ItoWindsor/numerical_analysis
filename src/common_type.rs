#![allow(non_camel_case_types)]

pub trait CommonType_t<T,U> {
    type Output;
}

impl<T> CommonType_t<T,T> for () {
    type Output = T;
}

impl CommonType_t<i32,f64> for () {
    type Output = f64;
}

impl CommonType_t<f64,i32> for () {
    type Output = f64;
}

