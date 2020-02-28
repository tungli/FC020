use nalgebra::{DVector, CsMatrix, CsVector, Dynamic};
use std::ops::{Add,Mul};

pub fn conjugate_gradient(m: &CsMatrix<f64>, rhs: &DVector<f64>, tol: f64, max_iter: Option<u32>) -> DVector<f64> {
    let n_iters = max_iter.unwrap_or(1000);
    println!("Iters: {}", n_iters);
    
    let mut x: CsVector<f64, Dynamic> = rhs.clone().into();
    let mut r: CsVector<f64, Dynamic> = rhs.clone().into();
    r = r.add(&(m.mul(&x).mul(-1.0_f64)));
    let mut p: CsVector<f64, Dynamic> = r.clone();
    let mut r2 = r.values_mut().fold(0.0_f64, |acc, x| acc + (*x)*(*x));

    for _ in 0..n_iters {
        let m_p: DVector<f64> = m.mul(&p).into();
        let p_full: DVector<f64> = p.clone().into();
        let a = r2/(p_full.dot(&m_p)); 
        x = x.add(&(p.clone().mul(a)));
        r = r.add(&(m.mul(&p).mul(-a)));
        let r2_new = r.values_mut().fold(0.0_f64, |acc, x| acc + (*x)*(*x));

        println!("Residual {}", r2_new);

        if r2_new < tol {
            return x.into();
        }
        let b = r2_new/r2;
        p = r.add(&(p.mul(b)));
        r2 = r2_new;
    }
    println!("Maximum number of iteration reached.");
    x.into()
}
