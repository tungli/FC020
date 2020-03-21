use rand_distr::{Normal, Distribution};
use rand;
use rand::SeedableRng;

use nalgebra::{U90, U2, MatrixMN, VectorN};
use nalgebra::base::dimension::Dim;
use std::ops::Mul;

use puruspe::{betai, gammp};

use roots::{find_root_brent, SimpleConvergency};

type N = U90;
type P = U2;

fn gen_data(sigma: f64, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let rng = rand::rngs::StdRng::seed_from_u64(seed);
    let dist = Normal::new(0.0, sigma).unwrap();
    let noise = dist.sample_iter(rng);
    
    fn parabola(x: f64) -> f64 {
        1.0 + 2.0*x*x
    }

    let x: Vec<f64> = (0..N{}.value()).map(|i| 1.0 + 0.1*i as f64).collect();
    let y: Vec<f64> = x.clone().iter().zip(noise).map(|(x, e)| parabola(*x) + e).collect();
    (x, y)
}


fn linear_regression(model_mat: MatrixMN<f64, N, P>, y: VectorN<f64, N>) -> (VectorN<f64, P>, MatrixMN<f64, P, P>, f64) {
    let (n, p) = model_mat.shape();
    let ata = model_mat.clone().tr_mul(&model_mat);
    let ata_inv = ata.try_inverse().expect("Failed matrix inversion");
    let params = ata_inv.mul(model_mat.transpose()).mul(y);

    let model = model_mat.mul(&params);
    let residual = y - model;
    let cov = residual.dot(&residual)/(n - p) as f64 * ata_inv;
    let rss = residual.dot(&residual);
    (params, cov, rss)
}

fn f_cumdist(x: f64, d1: f64, d2: f64) -> f64 {
    let d1x = d1*x;
    let z = d1x/(d1x + d2);
    let (d12, d22) = (d1/2.0, d2/2.0);
    betai(d12, d22, z)
}

fn f_value(alpha: f64, d1: f64, d2: f64) -> Option<f64> {
    let f = |x| { f_cumdist(x, d1, d2) - 1.0 + alpha };
    let mut convergency = SimpleConvergency { eps:1e-15f64, max_iter:100 };
    let root = find_root_brent(0_f64, 1000_f64, &f, &mut convergency);
    match root {
        Ok(value) => Some(value),
        _ => None
    }
}

fn chi_cumdist(x: f64, k: i32) -> f64{
    let k2 = k as f64/2.0;
    let x2 = x/2.0;
    gammp(k2, x2)
}

fn chi_value(alpha: f64, k: i32) -> Option<f64> {
    let f = |x| { chi_cumdist(x, k) - 1.0 + alpha };
    let mut convergency = SimpleConvergency { eps:1e-15f64, max_iter:1000 };
    let root = find_root_brent(0_f64, 1e3, &f, &mut convergency);
    match root {
        Ok(value) => Some(value),
        _ => None
    }
}

fn main() {
    let alpha = 0.05;
    let (n, p) = (N{}.value(), P{}.value());
    let sigma = 10.0;
    let (xs, ys) = gen_data(sigma, 65852132);

    let mut model_mat_vec = vec![1.0_f64; n];
    model_mat_vec.extend(xs.iter().map(|x| x*x));
    let model_mat = MatrixMN::<f64, N, P>::from_vec(model_mat_vec);

    let data = VectorN::<f64, N>::from_vec(ys);

    let (params, cov, rss) = linear_regression(model_mat, data);

    let s_model = (model_mat * cov * model_mat.transpose()).map_diagonal(|x| {x.sqrt()});
    let f = (2.0*f_value(alpha, p as f64, (n - p) as f64).unwrap()).sqrt();
    println!("sqrt(2f-value): {}", f);

    let chi_l = chi_value(alpha/2.0, (n - p) as i32).unwrap();
    let chi_u = chi_value(1.0 - alpha/2.0, (n - p) as i32).unwrap();
    let sigma_u = (rss/chi_u).sqrt(); 
    let sigma_l = (rss/chi_l).sqrt(); 
    println!("sigma_bounds: {} {}", sigma_l, sigma_u);
    
//    let f_distr: Vec<f64> = (0..100).map(|i| f_cumdist(i as f64 * 0.1, p as f64, (n - p) as f64)).collect();
//    println!("f_cumdist: {:?}", f_distr);

    let model = model_mat * params;
    let lower_bound = model - s_model * f;
    let upper_bound = model + s_model * f;
//    println!("prediceted: {}", model);
//    println!("l-bound: {}", lower_bound);
//    println!("u-bound: {}", upper_bound);

    println!("Fitted parameters value: a={}, b={}", params[0], params[1]);
    println!("Covariance matrix: {}", cov);
    

}
