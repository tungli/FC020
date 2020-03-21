extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct NoisyParabola {
    x: Vec<f64>,
    y: Vec<f64>,
    pub sigma: f64,
    pub seed: u64
}

#[wasm_bindgen]
impl NoisyParabola {
    pub fn new(sigma: f64, seed: u64) -> Self {
        let rng = rand::rngs::StdRng::seed_from_u64(seed);
        let dist = Normal::new(0.0, sigma).unwrap();
        let noise = dist.sample_iter(rng);
        
        fn parabola(x: f64) -> f64 {
            1.0 + 2.0*x*x
        }

        let x: Vec<f64> = (0..N{}.value()).map(|i| 1.0 + 0.1*i as f64).collect();
        let y: Vec<f64> = x.clone().iter().zip(noise).map(|(x, e)| parabola(*x) + e).collect();
        Self {x, y, sigma, seed}
    }

    pub fn x(&self) -> Vec<f64> {
        self.x.clone()
    }

    pub fn y(&self) -> Vec<f64> {
        self.y.clone()
    }
}

#[wasm_bindgen]
pub struct LinRegOutput {
    params: Vec<f64>,
    sigmas: Vec<f64>,
    pub correlation: f64,
    pub rss: f64,
    belt_l: Vec<f64>,
    belt_u: Vec<f64>,
    predicted: Vec<f64>,
    pub sigma_l: f64,
    pub sigma_u: f64,
}

#[wasm_bindgen]
impl LinRegOutput {
    pub fn params(&self) -> Vec<f64> {
        self.params.clone()
    }

    pub fn sigmas(&self) -> Vec<f64> {
        self.sigmas.clone()
    }

    pub fn belt_l(&self) -> Vec<f64> {
        self.belt_l.clone()
    }

    pub fn belt_u(&self) -> Vec<f64> {
        self.belt_u.clone()
    }
    
    pub fn predicted(&self) -> Vec<f64> {
        self.predicted.clone()
    }
}

#[wasm_bindgen]
pub struct TestIntervalsOutput {
    pub param_a: f64,
    pub param_b: f64,
    pub belt: f64,
    pub sigma_y: f64
}

#[wasm_bindgen]
pub struct TestIntervals {
    alpha: f64,
    sigma: f64,
    n_tests: usize,
    seeds: Vec<u64>,
}


#[wasm_bindgen]
impl TestIntervals {
    pub fn new(alpha: f64, sigma: f64, n_tests: usize, init_seed: u64) -> Self {
        let mut seeds = vec![init_seed; n_tests];
        let mut b = 0_u64;

        for s in seeds.iter_mut() {
            b += 12165613;
            *s += b ;
        }
        Self { alpha, sigma, n_tests, seeds }
    }

    pub fn seeds(&self) -> Vec<u64> {
        self.seeds.clone()
    }

    pub fn test(&self) -> TestIntervalsOutput {
        let mut hits = [0; 4];
        for seed in self.seeds.iter() {
            let data = NoisyParabola::new(self.sigma, *seed);
            let lr = LinRegParabola::new(self.alpha, data.clone());
            let out = lr.calc();
            
            if (out.params[0] - 1.0).abs() < out.sigmas[0] { hits[0] += 1 };
            if (out.params[1] - 2.0).abs() < out.sigmas[1] { hits[1] += 1 };

            if data.x().iter()
                .map(|x| 1.0 + 2.0 * x*x)
                .zip(out.belt_l)
                .zip(out.belt_u)
                .all(|((y, l), u)| y > l && y < u) { hits[2] += 1 };

            if (out.sigma_l < self.sigma) && (out.sigma_u > self.sigma) { hits[3] += 1 };
        }
        let r: Vec<f64> = hits.iter().map(|i| *i as f64 / self.n_tests as f64).collect();
        TestIntervalsOutput { param_a: r[0], param_b: r[1], belt: r[2], sigma_y: r[3] }
    }
}


#[wasm_bindgen]
pub struct LinRegParabola {
    alpha: f64,
    data: NoisyParabola
}

#[wasm_bindgen]
impl LinRegParabola {
    pub fn new(alpha: f64, data: NoisyParabola) -> Self {
        Self { alpha, data } 
    }

    pub fn calc(&self) -> LinRegOutput {
        let (n, p) = (N{}.value(), P{}.value());

        log(format!("#points: {}  #params: {}", n, p).as_str());

        let xs = &self.data.x;
        let ys = &self.data.y;

        let mut model_mat_vec = vec![1.0_f64; n];
        model_mat_vec.extend(xs.iter().map(|x| x*x));
        let model_mat = MatrixMN::<f64, N, P>::from_vec(model_mat_vec);

        let y = VectorN::<f64, N>::from_vec(ys.clone());

        let (ps, cov, rss) = Self::linear_regression(model_mat, y);

        log(format!("a = {},  b = {}, rss = {}", ps[0], ps[1], rss).as_str());

        let params: Vec<f64> = ps.iter().map(|x| *x).collect();

        let t_val = match Self::t_value(self.alpha/2.0, (n - p) as i32) {
            Some(value) => value,
            _ => {
                log("could not find t-value");
                0.0
            }
        };
        log(format!("t-value: {}", t_val).as_str());

        let sigmas: Vec<f64> = cov.map_diagonal(|x| x.sqrt()).iter().map(|x| *x).collect();
        let correlation: f64 = cov[(1,0)]/sigmas[0]/sigmas[1];


        let s_model = (model_mat * cov * model_mat.transpose()).map_diagonal(|x| {x.sqrt()});
        let f = match Self::f_value(self.alpha, p as f64, (n - p) as f64) {
            Some(value) => (2.0*value).sqrt(),
            _ => {
                log("could not find f-value");
                0.0
            }
        };

        log(format!("sqrt(2f-val) = {}", f).as_str());

        let chi_l = Self::chi_value(self.alpha/2.0, (n - p) as i32);
        let chi_u = Self::chi_value(1.0 - self.alpha/2.0, (n - p) as i32);
        
        let (sigma_l, sigma_u) = match (chi_l, chi_u) {
            (Some(c1), Some(c2)) => ((rss/c1).sqrt(), (rss/c2).sqrt()),
            _ => {
                log("could not find chi-values");
                (0.0, 0.0)
            }
        };

        log(format!("sigmas y {} -- {}", sigma_l, sigma_u).as_str());
        
        let model = model_mat * ps;
        let lower_bound = model - s_model * f;
        let upper_bound = model + s_model * f;

        let belt_l: Vec<f64> = lower_bound.iter().map(|x| *x).collect();
        let belt_u: Vec<f64> = upper_bound.iter().map(|x| *x).collect();
        let predicted: Vec<f64> = model.iter().map(|x| *x).collect();

        let s_params: Vec<f64> = sigmas.iter().map(|x| x*t_val).collect();
        LinRegOutput { params, sigmas: s_params, correlation, rss, belt_l, belt_u, predicted, sigma_l, sigma_u }
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

    fn t_cumdist(x: f64, k: i32) -> f64 {
        let z = k as f64 / (x*x + k as f64);
        1.0 - 0.5*betai(0.5*k as f64, 0.5, z)
    }
    
    fn t_value(alpha: f64, k: i32) -> Option<f64> {
        let f = |x| { Self::t_cumdist(x, k) - 1.0 + alpha };
        let mut convergency = SimpleConvergency { eps:1e-15f64, max_iter:100 };
        let root = find_root_brent(0_f64, 1000_f64, &f, &mut convergency);
        match root {
            Ok(value) => Some(value),
            _ => None
        }
    }

    fn f_cumdist(x: f64, d1: f64, d2: f64) -> f64 {
        let d1x = d1*x;
        let z = d1x/(d1x + d2);
        let (d12, d22) = (d1/2.0, d2/2.0);
        betai(d12, d22, z)
    }
    
    fn f_value(alpha: f64, d1: f64, d2: f64) -> Option<f64> {
        let f = |x| { Self::f_cumdist(x, d1, d2) - 1.0 + alpha };
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
        let f = |x| { Self::chi_cumdist(x, k) - 1.0 + alpha };
        let mut convergency = SimpleConvergency { eps:1e-10f64, max_iter:100 };
        let root = find_root_brent(0_f64, 4.0*k as f64, &f, &mut convergency);
        match root {
            Ok(value) => Some(value),
            _ => None
        }
    }
}

