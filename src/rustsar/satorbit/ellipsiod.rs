use std::{
    collections::HashMap,
    option::Option,
    ops::AddAssign,
    cmp::PartialOrd,
};

use once_cell::sync::Lazy;
use num_traits::Float;

use crate::{
    math::number::{
        Sqrt, Atan, SinCos, Arithmetic, Zero, Pi, One
    },
    satorbit::cartesian,
};

pub struct Coordinate<T> {
    pub longitude: T,
    pub latitude: T,
    pub height: T,
}

pub struct Ellipsoid<T> {
    a: T,
    b: T,
    a2: Option<T>,
    b2: Option<T>,
    e2: T
}

pub trait Transformable<T> :
    Sqrt<Output = T> + 
    Atan<Output = T> + 
    Arithmetic<T> +
    SinCos<T> + 
    Zero<T> + 
    One<T> + 
    Pi<T> + 
    Copy +
    AddAssign +
    PartialOrd 
{
}

impl<T: Transformable<T>>  Ellipsoid<T> {
    pub fn cart_ell(&mut self, x: T, y: T, z: T) -> Coordinate<T> {
        let a2 = *self.a2.get_or_insert(self.a * self.a);
        let b2 = *self.b2.get_or_insert(self.b * self.b);
        
        let mut n = a2 - b2;
        let p = (x * x + y * y).sqrt();
    
        let mut o = (self.a / p / self.b * z).atan();
        let mut so = o.sin();
        let mut co = o.cos();
        
        o = ((z + n / self.b * so * so * so) /
             (p - n / self.a * co * co * co)).atan();
        so = o.sin();
        co = o.cos();
        
        n = a2 / (a2 * co * co + b2 * so * so).sqrt();
        
        let mut lon: T = (y / x).atan();

        if x < T::ZERO {
            lon += T::PI;
        }
        
        Coordinate::<T> {
            latitude: o,
            longitude: lon,
            height:  p / co - n,
        }
    }
    
    pub fn ell_cart(&self, lon: T, lat: T, h: T) -> cartesian::Coordinate<T> {
        let sin_lat = lat.sin();
        let cos_lat = lat.cos();
        
        let n = self.a / (T::ONE - self.e2 * sin_lat * sin_lat).sqrt();
        
        cartesian::Coordinate::<T> {
            x: (                     n + h) * cos_lat * lon.cos(),
            y: (                     n + h) * cos_lat * lon.sin(),
            z: ((T::ONE - self.e2) * n + h) * sin_lat,
        }
    }
}

pub struct Builder<T> {
    a: T,
    b: T
}

impl<T: Float> Builder<T>
{
    pub fn build(&self) -> Ellipsoid<T> {
        let a2 = self.a * self.a;
        
        Ellipsoid {
            a: self.a,
            b: self.b,
            e2: (a2 - self.b * self.b) / a2,
            a2: Some(a2),
            b2: None,
        }
    }
}

type Cache = HashMap<String, Ellipsoid<f64>>;

pub static CACHE: Lazy<Cache> = Lazy::new(|| {
    let mut c = Cache::new();
    c.insert(
        String::from("wgs84"),
        Builder{
            a: 6378137.0,
            b: 6356752.3142
        }.build()
    );
    c    
});
