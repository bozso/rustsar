use num_traits::identities::{Zero, One};
use ndarray::{Array1, Array2, ArrayView1};

pub trait Fitted<T> {
    type Output;
    
    fn eval(&self, coord: Array1<T>) -> Self::Output;
    /// TODO: implement
    fn eval_coords(&self, coord: Array2<T>) -> Self::Output;
}

pub struct Fit<T> {
    coefficients: ndarray::Array2<T>,
}

impl<T: Clone + Zero + One> Fit<T> {
    fn make_vander<'a>(x: ArrayView1<'a, T>, degree: usize) -> Array2<T> {
        let nrows = x.shape()[0] as usize;
        
        let mut arr = Array2::<T>::zeros((nrows, degree + 1));
        
        for ii in 0..nrows {
            arr[[ii, 1]] = T::one();
            arr[[ii, 2]] = x[ii];
            
            for jj in 2..degree {
                arr[[ii, jj]] = arr[[ii, jj-2]] * arr[[ii, jj-1]];
            }
        }
        arr
    }
    
    pub fn fit(x: &Array1<T>, y: &Array1<T>, degree: usize) -> Self {
        let jacobi = Self::make_vander(x.view(), degree);
        
        Self {
            coefficients: jacobi
        }
    }
/*
    pub fn eval_poly(arr_in X, arr_out Y) -> Self::Result {
        let x = X.const_view<T>(1),
            y = Y.view<T>(2),
            vcoeffs = poly.coeffs.const_view<T>(1),
            vncoeffs = poly.ncoeffs.const_view<idx>(1);
    
        for (idx ii = 0; ii < x.shape(0); ++ii) {
            idx start = 0, stop = vncoeffs(0);
            auto const& _x = x(ii);
            
            for (idx nn = 0; nn < poly.nfit; ++nn) {
                /*
                   nfit = 3
                   ncoeffs = (3, 4, 2)
                   0. 1. 2. | 3. 4. 5. 6. | 7. 8.
                   
                   nn = 0, start = 0, stop = 3
                   jj = 0,1,2
    
                   nn = 1
                   start = 3, stop = 7
                   jj = 3, 4, 5, 6
                   
                   nn = 2
                   start = 7, stop = 9
                   jj = 7, 8
                 */
                
                T sum = vcoeffs(start);
                
                for (idx jj = start + 1; jj < stop - 1; ++jj) {
                    sum = vcoeffs(jj) +  sum * _x;
                }
                
                y(ii, nn) = vcoeffs(stop) + sum * _x;
                
                start += vncoeffs(nn);
                stop += vncoeffs(nn + 1);
            }
        }
    }
*/
}
