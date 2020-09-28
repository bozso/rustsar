use std::{
    ops::{Add, Sub, Mul, Div},
    f32, f64
};

pub trait Sin {
    type Output;
    
    fn sin(self) -> Self::Output;
}

pub trait Asin {
    type Output;
    
    fn asin(self) -> Self::Output;
}

pub trait Cos {
    type Output;
    
    fn cos(self) -> Self::Output;
}

pub trait Acos {
    type Output;
    
    fn acos(self) -> Self::Output;
}

pub trait SinCos<T> : Sin<Output = T> + Cos<Output = T> {}

pub trait Tan {
    type Output;

    fn tan(self) -> Self::Output;
    fn atan2(self) -> Self::Output;
}

pub trait Atan {
    type Output;

    fn atan(self) -> Self::Output;
}

pub trait Hyperbolic {
    type Output;
    
    fn sinh(self) -> Self::Output;
    fn cosh(self) -> Self::Output;    

    fn asinh(self) -> Self::Output;
    fn acosh(self) -> Self::Output;    

    fn tanh(self) -> Self::Output;
    fn atanh(self) -> Self::Output;
}

pub trait Sqrt {
    type Output;
    
    fn sqrt(self) -> Self::Output;
}

pub trait AddSub<T> : Add<Output=T> + Sub<Output=T>  + Copy {}
pub trait MulDiv<T> : Mul<Output=T> + Div<Output=T>  + Copy {}


pub trait Arithmetic<T> : AddSub<T> + MulDiv<T> {} 

pub trait Zero<T> {
    const ZERO: T;
}

pub trait One<T> {
    const ONE: T;
}

pub trait Pi<T> {
    const PI: T;
}

impl Pi<f32> for f32 {
    const PI: f32 = f32::consts::PI;
}

impl Pi<f64> for f64 {
    const PI: f64 = f64::consts::PI;
}

macro_rules! impl_all {
    ($t:ty) => {
        impl Zero<$t> for $t {
            const ZERO: $t = 0.0 as $t;
        }

        impl One<$t> for $t {
            const ONE: $t = 1.0 as $t;
        }
    }
}

impl_all!(i8);
impl_all!(i16);
impl_all!(i32);
impl_all!(i64);

impl_all!(u8);
impl_all!(u16);
impl_all!(u32);
impl_all!(u64);

impl_all!(f32);
impl_all!(f64);

pub trait Number<T> : AddSub<T> + MulDiv<T> + Copy {}
