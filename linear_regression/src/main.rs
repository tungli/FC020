use rand_distr::{Normal, Distribution};
use rand;
use nalgebra::{U90, U2, MatrixMN, VectorN};
use nalgebra::base::dimension::Dim;
use std::ops::Mul;

type N = U90;
type P = U2;

fn gen_data(sigma: f64) -> (Vec<f64>, Vec<f64>) {
    let rng = rand::thread_rng();
    let dist = Normal::new(0.0, sigma).unwrap();
    let noise = dist.sample_iter(rng);
    
    fn parabola(x: f64) -> f64 {
        1.0 + 2.0*x*x
    }

    let x: Vec<f64> = (0..N{}.value()).map(|i| 1.0 + 0.1*i as f64).collect();
    let y: Vec<f64> = x.clone().iter().zip(noise).map(|(x, e)| parabola(*x) + e).collect();
    (x, y)
}


fn linear_regression(model_mat: MatrixMN<f64, N, P>, y: VectorN<f64, N>) -> (VectorN<f64, P>, MatrixMN<f64, P, P>) {
    let (n, p) = model_mat.shape();
    let ata = model_mat.clone().tr_mul(&model_mat);
    let ata_inv = ata.try_inverse().expect("Failed matrix inversion");
    let params = ata_inv.mul(model_mat.transpose()).mul(y);

    let model = model_mat.mul(&params);
    let residual = y - model;
    let cov = residual.dot(&residual)/(n - p) as f64 * ata_inv;
    (params, cov)
}


fn main() {
    let sigma = 0.1;
    let (xs, ys) = gen_data(sigma);

    let mut model_mat_vec = vec![1.0_f64; N{}.value()];
    model_mat_vec.extend(xs.iter().map(|x| x*x));
    let model_mat = MatrixMN::<f64, N, P>::from_vec(model_mat_vec);

    let data = VectorN::<f64, N>::from_vec(ys);

    let (p, c) = linear_regression(model_mat, data);

    println!("p {}", p);
    println!("C {}", c);

}
