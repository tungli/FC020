use nalgebra::{DMatrix, DVector};
use itertools::Itertools;
use num_traits::Float;

#[derive(Debug)]
pub struct Linspace {
    from: f64,
    to: f64,
    n: usize
}

impl Linspace {
    pub fn new(from: f64, to: f64, n: usize) -> Linspace {
        Linspace { from: from, to: to, n: n }
    }

    fn dx(&self) -> f64 {
        (self.to - self.from)/(self.n - 1) as f64
    }

    pub fn gen(&self) -> impl Iterator<Item = f64> + '_ {
        (0..(self.n))
            .map(move |i| self.from + self.dx()*i as f64)
    }
}

#[derive(Debug)]
pub struct Square {
    l_corner: (f64, f64),
    length: f64,
    n: usize,
    domain: DMatrix<f64>
}

impl Square {
    pub fn new(a: (f64, f64), b: f64, n: usize) -> Square {
        Square {l_corner: a, length: b, n: n, domain: DMatrix::zeros(n, n)}
    }

    pub fn sub2ind(&self, sub: (usize, usize)) -> usize {
        sub.0 * self.n + sub.1
    }

    pub fn ind2sub(&self, ind: usize) -> (usize, usize) {
        (ind % self.n, ind / self.n)
    }

    pub fn x(&self) -> Vec<f64> {
        let x0 = self.l_corner.0;
        let x1 = x0 + self.length;
        Linspace::new(x0, x1, self.n)
            .gen()
            .collect()
    }

    pub fn y(&self) -> Vec<f64> {
        let y0 = self.l_corner.1;
        let y1 = y0 + self.length;
        Linspace::new(y0, y1, self.n)
            .gen()
            .collect()
    }

    pub fn grid(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.n).cartesian_product(0..self.n)
    }

    pub fn laplace(&self) -> (DMatrix<f64>, DVector<f64>) {
        let size = self.n*self.n - 2;
        let mut m = DMatrix::zeros(size, size);
        let mut rhs = DVector::zeros(size);
        (m, rhs)
    }
}


