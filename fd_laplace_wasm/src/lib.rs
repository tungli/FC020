extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod square;
mod cg_sparse;

use square::*;
use cg_sparse::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct FdLaplace {
    data: Vec<Vec<f64>>,
    x: Vec<f64>,
    pub size: usize
}

#[wasm_bindgen]
impl FdLaplace {
    pub fn new(size: usize) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();

        let square = FdLaplace::calc(size);
        let x = square.x();
        let matrix = square.domain;

        log("Matrix calculated");

        log(&matrix.to_string());

        for i in 0..size {
            let mut buffer: Vec<f64> = Vec::new();
            for j in 0..size {
                buffer.push(matrix[(i,j)]);
            }
            data.push(buffer);
        }

        let s = format!("{:?}", data.clone());
        log(&s);

        Self { data, x, size }
    }

    fn anal(&self) -> Vec<Vec<f64>> {
        let x = self.x();
        let size = self.size;

        let mut anal_sol: Vec<Vec<f64>> = Vec::new();

        for i in 0..size {
            let mut buffer: Vec<f64> = Vec::new();
            for j in 0..size {
                let a = (2.0*x[i]).cos()*(-2.0_f64*x[j]).exp();
                buffer.push(a);
            }
            anal_sol.push(buffer);
        }
        anal_sol
    }

    pub fn error_row(&self, n: usize) -> Vec<f64> {
        let a_sol = self.anal();
        self.data[n].clone().iter()
            .zip(a_sol[n].clone())
            .map(|(i, j)| (i - j).abs())
            .collect()
    }
    
    pub fn x(&self) -> Vec<f64> {
        self.x.clone()
    }

    pub fn row(&self, n: usize) -> Vec<f64> {
        self.data[n].clone()
    }
    
    fn calc(size: usize) -> Square {
        let mut domain = Square::new((0.0, 0.0), 1.0, size);
    
        domain.set_upper_bc_dirichlet(|x| (-2.0*x).exp());
        domain.set_left_bc_dirichlet(|x| (2.0*x).cos());
        domain.set_lower_bc_dirichlet(|x| (-2.0*x).exp()*2.0_f64.cos());
        domain.set_right_bc_dirichlet(|x| (2.0*x).cos()*(-2.0_f64).exp());
    
        log("BC set!");
    
        let (m, bc_rhs) = domain.laplace();
    
        log("Matrix assembled!");
        
        let res = conjugate_gradient(&m, &bc_rhs, 1e-8, Some(2000));
    
        log("Matrix solved!");
    
        for i in 1..(size-1) {
            for j in 1..(size-1) {
                let k = domain.sub2ind_interior((i,j));
                domain.domain[(i,j)] = res[k];
            }
        }
        domain
    }
}

